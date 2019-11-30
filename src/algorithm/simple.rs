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

#[derive(Copy, Clone)]
pub struct SimpleReplacement {}

struct SimpleReplacementExec<V,P,H> {
    problem: Rc<P>,
    organisms: Vec<Organism<V>>,
    elitism: Rc<dyn Elitism>,
    scorer: Rc<dyn Scorer<V,P>>,
    hyperparameters: Rc<H>
}

impl Named for SimpleReplacement {
    fn name(&self) -> String {
        String::from("SimpleReplacement")
    }
}

impl<V: 'static,F,P: 'static,H: 'static> ReplacementSelection<V,F,P,H> for SimpleReplacement {
    fn initialize_solver(
            &self, pop_size: usize,
            feature_mapper: Rc<dyn FeatureMapper<V, F, P>>,
            problem: Rc<P>,
            scorer: Rc<dyn Scorer<V,P>>,
            environment: Rc<dyn Environment<H>>,
            constant_hyperparameters: Rc<H>,
            generator: Rc<dyn OrganismGenerator<V, P>>,
            elitism: Rc<dyn Elitism>) -> Box<dyn UpdatableSolver<V>> {
        let mut gr = vec![];
        for i in 0..pop_size {
            gr.push(generator.generate_organism(problem.clone()));
        }
        return Box::new(SimpleReplacementExec {
            problem: problem.clone(),
            organisms: gr,
            elitism: elitism.clone(),
            scorer: scorer.clone(),
            hyperparameters: constant_hyperparameters.clone()
        });
    }
}

impl<V,P,H> UpdatableSolver<V> for SimpleReplacementExec<V,P,H> {
    fn update(&mut self) -> Vec<Organism<V>> {
        let size = self.organisms.len();
        let mut rng = thread_rng();
        let index_a = rng.gen_range(0,size);
        let mut index_b = rng.gen_range(0, size);
        while index_b == index_a {
            let mut index_b = rng.gen_range(0, size);
        }

        let mut score_a;

        {
            let org_a= self.organisms.get_mut(index_a).unwrap();
            score_a = org_a.score_with_cache(self.scorer.clone(), self.problem.as_ref());

        }

        let mut score_b;

        {
            let org_b = self.organisms.get_mut(index_b).unwrap();
            score_b = org_b.score_with_cache(self.scorer.clone(), self.problem.as_ref());
        }



        unimplemented!()
    }
}
