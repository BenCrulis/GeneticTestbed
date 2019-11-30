use crate::organism::grid::Grid;
use crate::organism::{Organism,OrganismGenerator};
use crate::features::FeatureMapper;

use crate::common::*;
use crate::algorithm::selection::Elitism;
use std::rc::Rc;
use crate::problems::Environment;
use crate::scoring::Scorer;


pub trait ReplacementSelection<V,F,P,H>: Named {
    fn initialize_solver(
        &self,
        pop_size: usize,
        feature_mapper: Rc<dyn FeatureMapper<V,F,P>>,
        problem: Rc<P>,
        scorer: Rc<dyn Scorer<V,P>>,
        environment: Rc<dyn Environment<H>>,
        constant_hyperparameters: Rc<H>,
        generator: Rc<dyn OrganismGenerator<V,P>>,
        elitism: Rc<dyn Elitism>) -> Box<dyn UpdatableSolver<V>>;

}


pub trait UpdatableSolver<V> {
    fn update(&mut self) -> Vec<Organism<V>>;
}



struct GeneralizedMAPElite {
    use_features: bool,
    use_hyperparameter_mapping: bool,
    number_of_spatial_dimensions: usize
}

impl Named for GeneralizedMAPElite {
    fn name(&self) -> String {
        String::from("Generalized MAP Elite algorithm")
    }
}


