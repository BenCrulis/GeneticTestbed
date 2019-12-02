use std::collections::HashMap;
use crate::common::Named;
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

#[derive(Copy, Clone)]
pub struct SimpleReplacement {}

struct SimpleReplacementExec<V,P,F,H> {
    problem: Rc<P>,
    organisms: Vec<Organism<V>>,
    problem_config: Rc<ProblemConfig<V,P,F,H>>,
    elitism: Rc<dyn Elitism>
}

impl Named for SimpleReplacement {
    fn name(&self) -> String {
        String::from("SimpleReplacement")
    }
}

impl<V: Clone + 'static,P: 'static,F: 'static,H: Clone + 'static> ReplacementSelection<V,P,F,H> for SimpleReplacement {
    fn initialize_solver(
            &self, pop_size: usize,
            problem: Rc<P>,
            elitism: Rc<dyn Elitism>,
            problem_config: Rc<ProblemConfig<V,P,F,H>>) -> Box<dyn UpdatableSolver<V>> {
        let generator = &problem_config.random_organism_generator;

        let mut gr = vec![];
        for i in 0..pop_size {
            gr.push(generator.generate_organism(problem.clone()));
        }
        return Box::new(SimpleReplacementExec {
            problem: problem.clone(),
            organisms: gr,
            problem_config: problem_config.clone(),
            elitism
        });
    }
}

impl<V: Clone,P,F,H> UpdatableSolver<V> for SimpleReplacementExec<V,P,F,H> {
    fn update(&mut self) -> Vec<Organism<V>> {
        let scorer = &self.problem_config.scorer;

        let size = self.organisms.len();
        let mut rng = thread_rng();
        let index_a = rng.gen_range(0,size);
        let mut index_b = rng.gen_range(0, size);

        while index_b == index_a {
            index_b = rng.gen_range(0, size);
        }

        let mut score_a;

        {
            let org_a= self.organisms.get_mut(index_a).unwrap();
            score_a = org_a.score_with_cache(scorer.as_ref(), self.problem.as_ref());

        }

        let mut score_b;

        {
            let org_b = self.organisms.get_mut(index_b).unwrap();
            score_b = org_b.score_with_cache(scorer.as_ref(), self.problem.as_ref());
        }

        let keep_first = self.elitism.choose(score_a, score_b);

        let mut to_replace = index_a;
        let mut to_keep = index_b;
        if keep_first {
            to_replace = index_b;
            to_keep = index_a;
        }

        let mut new_org: Organism<V> = self.organisms.get(to_keep)
            .unwrap()
            .clone();

        new_org.mutate(self.problem_config.mutator.clone(),
                    &self.problem_config.constant_hyperparameters);

        self.organisms[to_replace] = new_org;


        return self.organisms.clone();
    }
}
