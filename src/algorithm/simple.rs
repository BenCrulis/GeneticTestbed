use std::collections::HashMap;
use crate::common::{Named, Parametrized, str_param};
use crate::algorithm::algorithm::{ReplacementSelection, UpdatableSolver};
use crate::organism::{OrganismGenerator, Organism};
use crate::features::FeatureMapper;
use crate::organism::grid::Grid;
use rand::{thread_rng, Rng};
use std::rc::Rc;
use crate::algorithm::selection::Elitism;
use crate::problems::Environment;
use crate::scoring::Scorer;
use crate::algorithm::config::ProblemConfig;
use serde_json::{Map, Value};

#[derive(Copy, Clone)]
pub struct SimpleReplacement {}

struct SimpleReplacementExec<V,P,H> {
    problem: Rc<P>,
    organisms: Vec<Organism<V>>,
    problem_config: Rc<ProblemConfig<V,P,H>>,
    elitism: Rc<dyn Elitism>
}

impl Named for SimpleReplacement {
    fn name(&self) -> String {
        String::from("SimpleReplacement")
    }
}

impl Parametrized for SimpleReplacement {
    fn parameters(&self) -> Value {
        let mut config = Map::new();
        config.insert("use spatial grid".to_string(), false.into());
        config.insert("use spatial hyperparameters".to_string(), false.into());
        config.insert("use features".to_string(), false.into());

        return Value::Object(config);
    }
}

impl<V: Clone + 'static,P: 'static,H: Clone + 'static> ReplacementSelection<V,P,H> for SimpleReplacement {
    fn initialize_solver(
            &self, pop_size: usize,
            problem: Rc<P>,
            elitism: Rc<dyn Elitism>,
            problem_config: Rc<ProblemConfig<V,P,H>>) -> Box<dyn UpdatableSolver<V>> {
        let generator = &problem_config.random_organism_generator;

        let mut gr = Vec::with_capacity(pop_size);
        for _i in 0..pop_size {
            gr.push(generator.generate_organism(problem.as_ref()));
        }
        return Box::new(SimpleReplacementExec {
            problem: problem.clone(),
            organisms: gr,
            problem_config: problem_config.clone(),
            elitism
        });
    }
}

impl<V: Clone,P,H> UpdatableSolver<V> for SimpleReplacementExec<V,P,H> {
    fn update(&mut self) -> Vec<Organism<V>> {
        let scorer = &self.problem_config.scorer;

        let size = self.organisms.len();
        let mut rng = thread_rng();
        let index_a = rng.gen_range(0,size);
        let mut index_replace = rng.gen_range(0, size);

        while index_replace == index_a {
            index_replace = rng.gen_range(0, size);
        }

        let score;
        let score_replace;

        let org = {
            let org_a= self.organisms.get_mut(index_a).unwrap();

            let mut org_b = org_a.clone();

            org_b.mutate(self.problem_config.mutator.as_ref(), &self.problem_config.constant_hyperparameters);

            score = org_b.score_with_cache(scorer.as_ref(), self.problem.as_ref());
            org_b
        };

        {
            let org_c = self.organisms.get_mut(index_replace).unwrap();
            score_replace = org_c.score_with_cache(scorer.as_ref(), self.problem.as_ref());
        }

        let keep_first = self.elitism.choose(score, score_replace);

        if keep_first {
            self.organisms[index_replace] = org;
        }

        return self.organisms.clone();
    }
}
