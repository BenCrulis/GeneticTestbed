use std::collections::HashMap;
use crate::common::Named;
use crate::algorithm::algorithm::{ReplacementSelection, UpdatableSolver};
use crate::organism::{Genome, OrganismGenerator, Organism};
use crate::features::FeatureMapper;
use crate::organism::grid::Grid;
use rand::{thread_rng, Rng};
use std::rc::Rc;

#[derive(Copy, Clone)]
pub struct SimpleReplacement {}

struct SimpleReplacementExec<V,P> {
    problem: Rc<P>,
    organisms: Vec<Organism<V>>
}

impl Named for SimpleReplacement {
    fn name(&self) -> String {
        String::from("SimpleReplacement")
    }
}

impl<V: Genome<P=P,H=H>,F,P,H> ReplacementSelection<V,F,P,H> for SimpleReplacement {
    fn initialize_solver(
        &self,
        pop_size: usize,
        feature_mapper: &dyn FeatureMapper<V, F, P>,
        problem: &P,
        generator: &dyn OrganismGenerator<V, P>) -> Box<dyn UpdatableSolver<V>> {
        let mut gr = vec![];
        for i in 0..pop_size {
            let mut hm = HashMap::new();
            hm.insert((),generator.generate_organism(problem));
            gr.push(hm);
        }

        unimplemented!()
    }
}

impl<V,P> UpdatableSolver<V> for SimpleReplacementExec<V,P> {
    fn update(&mut self) -> Vec<Organism<V>> {
        let size = self.organisms.len();
        let mut rng = thread_rng();
        let index_a = rng.gen_range(0,size);
        let mut index_b = rng.gen_range(0, size);
        while index_b == index_a {
            let mut index_b = rng.gen_range(0, size);
        }

        unimplemented!()
    }
}
