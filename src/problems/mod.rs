pub mod rastrigin;
pub mod travelling_salesman;

use crate::common::Named;
use crate::common::Parametrized;
use std::ops::Div;

pub trait ProblemInstanceGenerator<P>: Named + Parametrized {
    fn generate_problem(&self) -> P;
}

pub trait Hyperparameter {
    fn number_of_hyperparameters() -> usize;
}

pub trait Environment<H: Hyperparameter>: Named {
    fn number_of_hyperparameters(&self) -> usize {
        return H::number_of_hyperparameters()
    }
    fn map_hyperparameters(&self, coordinates: &Vec<(usize, usize)>) -> H;
}

impl Named for SpatialMapper {
    fn name(&self) -> String {
        String::from("Spatial mapping")
    }
}

#[derive(Copy, Clone)]
pub struct DiscreteHyperparameters {
    pub mutation_chance: f64
}

#[derive(Copy, Clone)]
pub struct ContinuousHyperparameters {
    pub mutation_chance: f64,
    pub mutation_size: f64
}

impl Hyperparameter for DiscreteHyperparameters {
    fn number_of_hyperparameters() -> usize {
        return 1;
    }
}

impl Hyperparameter for ContinuousHyperparameters {
    fn number_of_hyperparameters() -> usize {
        return 2;
    }
}

//
// Continuous hyperparameters mapping
//

impl Environment<ContinuousHyperparameters> for SpatialMapper {
    fn map_hyperparameters(&self, coordinates: &Vec<(usize, usize)>) -> ContinuousHyperparameters {
        let (v1, m1) = coordinates.get(0).unwrap();
        let (v2, m2) = coordinates.get(1).unwrap();
        let prob1: f64 = (v1+1) as f64/((m1+2) as f64);
        let prob2: f64 = (v2+1) as f64/((m2+2) as f64);
        return ContinuousHyperparameters {
            mutation_chance: prob1,
            mutation_size: prob2
        }
    }
}

//
// Discrete hyperparameters handling
//

#[derive(Copy, Clone)]
pub struct SpatialMapper {
    pub number_of_additional_dimensions: usize,
}

impl Environment<DiscreteHyperparameters> for SpatialMapper {
    fn map_hyperparameters(&self, coordinates: &Vec<(usize, usize)>) -> DiscreteHyperparameters {
        let (val,max) = coordinates.get(0).unwrap();
        let prob: f64 = (val+1) as f64/((max+2) as f64);
        return DiscreteHyperparameters{
            mutation_chance: prob
        };
    }
}

