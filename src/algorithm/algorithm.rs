use crate::organism::grid::Grid;
use crate::organism::{Genome,Organism,OrganismGenerator};
use crate::features::FeatureMapper;

use crate::common::*;


pub trait ReplacementSelection<V: Genome<P=P,H=H>,F,P,H>: Named {
    fn initialize_solver(
        &self,
        pop_size: usize,
        feature_mapper: &dyn FeatureMapper<V,F,P>,
        problem: &P,
        generator: &dyn OrganismGenerator<V,P>) -> Box<dyn UpdatableSolver<V>>;

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


