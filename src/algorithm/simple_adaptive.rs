use std::rc::Rc;
use crate::organism::Organism;
use crate::algorithm::config::ProblemConfig;
use crate::algorithm::selection::Elitism;
use crate::problems::DiscreteHyperparameters;
use crate::algorithm::mutation::Mutator;
use rand::{thread_rng, Rng};
use crate::scoring::Scorer;
use crate::common::{Named, Parametrized};
use serde_json::{Value, Map};
use crate::{MyConfigIt};
use crate::algorithm::algorithm::{UpdatableSolver, ReplacementSelection};

#[derive(Copy, Clone)]
pub struct SimpleAdaptive {
    pub prior_a: i64,
    pub prior_b: i64
}

impl Named for SimpleAdaptive {
    fn name(&self) -> String {
        "Simple Adaptive GA".to_string()
    }
}

impl Parametrized for SimpleAdaptive {
    fn parameters(&self) -> Value {
        let mut hm = Map::new();
        hm.insert("prior a".to_string(), self.prior_a.into());
        hm.insert("prior b".to_string(), self.prior_b.into());
        hm.insert("use spatial grid".to_string(), false.into());
        hm.insert("use spatial hyperparameters".to_string(), false.into());
        hm.insert("use features".to_string(), false.into());
        return Value::Object(hm);
    }
}

impl<V: 'static + Clone, P: 'static> ReplacementSelection<V,P,DiscreteHyperparameters> for SimpleAdaptive {
    fn initialize_solver(&self,
                         pop_size: usize,
                         problem: Rc<P>,
                         elitism: Rc<dyn Elitism>,
                         problem_config: Rc<ProblemConfig<V, P, DiscreteHyperparameters>>) -> Box<dyn UpdatableSolver<V>> {
        let mut pop = Vec::with_capacity(pop_size);

        for _i in 0..pop_size {
            let org = problem_config.random_organism_generator.generate_organism(&problem);
            pop.push(AdaptiveOrg::new(self.prior_a, self.prior_b, org));
        }

        return Box::new(SimpleAdaptiveExec {
            problem: problem.clone(),
            organisms: pop,
            problem_config: problem_config.clone(),
            elitism
        });
    }
}

#[derive(Copy, Clone)]
struct AdaptiveOrg<V> {
    a: i64,
    b: i64,
    org: Organism<V>
}

impl<V> AdaptiveOrg<V> {
    fn new(prior_a: i64, prior_b: i64, organism: Organism<V>) -> Self {
        return AdaptiveOrg {
            a: prior_a,
            b: prior_b,
            org: organism
        }
    }

    fn into_organism(self) -> Organism<V> {
        return self.org;
    }

    fn get_mut_prob(&self) -> f64 {
        let af = self.a as f64;
        return af/(af + self.b as f64);
    }

    fn mut_prob(&mut self) {
        let mut rng = thread_rng();

        while rng.gen::<f64>() < self.get_mut_prob() {
            let p_ref = if rng.gen_bool(0.5) {
                &mut self.a
            }
            else {
                &mut self.b
            };

            if rng.gen_bool(0.5) {
                *p_ref += 1;
            }
            else {
                *p_ref -= 1;
            }

            *p_ref = (*p_ref).max(1);
        }
    }

    fn mutate(&mut self, mutator: &dyn Mutator<V,DiscreteHyperparameters>) {

        let mut rng = thread_rng();

        self.mut_prob();

        let hyper = DiscreteHyperparameters {
            mutation_chance: self.get_mut_prob()
        };

        self.org.mutate(mutator, &hyper);
    }

    fn score_with_cache<P>(&mut self, scorer: &dyn Scorer<V,P>, problem: &P) -> f64 {
        self.org.score_with_cache(scorer, problem)
    }

    fn organism_ref(&self) -> &Organism<V> {
        return &self.org;
    }

    fn organism_ref_mut(&mut self) -> &mut Organism<V> {
        return &mut self.org;
    }
}

pub struct SimpleAdaptiveExec<V,P> {
    problem: Rc<P>,
    organisms: Vec<AdaptiveOrg<V>>,
    problem_config: Rc<ProblemConfig<V,P,DiscreteHyperparameters>>,
    elitism: Rc<dyn Elitism>
}

impl<V: Clone,P> UpdatableSolver<V> for SimpleAdaptiveExec<V,P> {
    fn update(&mut self) -> Vec<Organism<V>> {

        let scorer = &self.problem_config.scorer;

        let size = self.organisms.len();
        let mut rng = thread_rng();
        let index = rng.gen_range(0, size);
        let mut index_replace = rng.gen_range(0, size);

        while index_replace == index {
            index_replace = rng.gen_range(0, size);
        }

        let score;
        let score_replace;

        let org = {
            let org_a= self.organisms.get_mut(index).unwrap();

            let mut org_b = org_a.clone();

            org_b.mutate(self.problem_config.mutator.as_ref());

            score = org_b.score_with_cache(scorer.as_ref(), self.problem.as_ref());
            org_b
        };

        {
            let org_c: &mut AdaptiveOrg<V> = self.organisms.get_mut(index_replace).unwrap();
            score_replace = org_c.score_with_cache(scorer.as_ref(), self.problem.as_ref());
        }

        let keep_first = self.elitism.choose(score, score_replace);

        if keep_first {
            self.organisms[index_replace] = org;
        }

        return self.organisms.iter()
            .map(|ao| ao.organism_ref().clone())
            .collect();
    }
}