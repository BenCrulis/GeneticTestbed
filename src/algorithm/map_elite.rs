use std::collections::HashMap;
use crate::organism::Organism;
use std::rc::Rc;
use crate::algorithm::config::ProblemConfig;
use crate::algorithm::selection::Elitism;
use crate::algorithm::algorithm::{ReplacementSelection, UpdatableSolver};
use crate::common::{Named, Parametrized};
use rand::thread_rng;
use rand::seq::IteratorRandom;
use std::hash::Hash;
use std::collections::hash_map::Entry;
use rand::seq::SliceRandom;

pub struct MAPElite {}

impl Named for MAPElite {
    fn name(&self) -> String {
        "MAP Elite".to_string()
    }
}

impl Parametrized for MAPElite {}

impl<V: 'static + Clone ,P: 'static,F: 'static + Clone + Eq + Hash,H: 'static> ReplacementSelection<V,P,F,H> for MAPElite {
    fn initialize_solver(&self,
                         pop_size: usize,
                         problem: Rc<P>,
                         elitism: Rc<dyn Elitism>,
                         problem_config: Rc<ProblemConfig<V, P, F, H>>) -> Box<dyn UpdatableSolver<V>> {
        let mut hm = HashMap::new();

        let org = problem_config.random_organism_generator.generate_organism(
            problem.as_ref());

        let feat = problem_config.feature_mapper.project(&org.genotype);

        hm.insert(feat, org);

        Box::new(MAPEliteExec {
            niches: hm,
            problem: problem.clone(),
            problem_config: problem_config.clone(),
            elitism
        })
    }
}


pub struct MAPEliteExec<V,P,F,H> {
    niches: HashMap<F,Organism<V>>,
    problem: Rc<P>,
    problem_config: Rc<ProblemConfig<V,P,F,H>>,
    elitism: Rc<dyn Elitism>
}


impl<V: Clone,P,F: Clone + Eq + Hash,H> UpdatableSolver<V> for MAPEliteExec<V,P,F,H> {
    fn update(&mut self) -> Vec<Organism<V>> {

        let scorer = &self.problem_config.scorer;
        let problem = self.problem.as_ref();
        let elitism = self.elitism.as_ref();

        let mut rng = thread_rng();

        let v: Vec<(&F,&Organism<V>)> = self.niches.iter().collect();

        let &(_,x) = v.choose(&mut rng).unwrap();

        let mut new_org: Organism<V> = x.clone();

        new_org.mutate(self.problem_config.mutator.as_ref(), &self.problem_config.constant_hyperparameters);

        let new_feat = self.problem_config.feature_mapper.project(&new_org.genotype);

        let copied = new_org.clone();

        let ent = self.niches.entry(new_feat);

        ent.and_modify(|retrieved| {
            let score_new = new_org.score_with_cache(scorer.as_ref(), problem);
            let score_retrieved = retrieved.score_with_cache(scorer.as_ref(), problem);

            if elitism.choose(score_new, score_retrieved) {
                *retrieved = new_org;
            }
        }).or_insert(copied);

        return self.niches.values().cloned().collect();
    }
}

