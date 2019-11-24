use std::hash::Hash;

use super::common::{Named, Parametrized};

pub trait FeatureMapper<G,F: Hash + Clone + Eq + PartialEq, P>:  Named + Parametrized {
    fn number_of_possible_features(&self, problem: &P) -> usize;
    fn project(&self, genome: G) -> F;
}

