use crate::organism::grid::Grid;
use crate::organism::{Organism,OrganismGenerator};
use crate::features::FeatureMapper;

use crate::common::*;
use crate::algorithm::selection::Elitism;
use std::rc::Rc;
use crate::problems::Environment;
use crate::scoring::Scorer;
use crate::algorithm::config::ProblemConfig;


pub trait ReplacementSelection<V,P,H>: Named + Parametrized {
    fn initialize_solver(
        &self,
        pop_size: usize,
        problem: Rc<P>,
        elitism: Rc<dyn Elitism>,
        problem_config: Rc<ProblemConfig<V,P,H>>) -> Box<dyn UpdatableSolver<V>>;

}


pub trait UpdatableSolver<V> {
    fn update(&mut self) -> Vec<Organism<V>>;
}


