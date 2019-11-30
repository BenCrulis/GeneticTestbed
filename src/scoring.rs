use std::rc::Rc;
use crate::organism::Organism;

pub trait Scorer<V,P> {
    fn score(&self, genome: &V, problem: &P) -> f64;
}

