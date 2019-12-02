use std::hash::Hash;

use super::common::{Named, Parametrized};

pub trait FeatureMapper<V,F: Hash + Clone + Eq + PartialEq, P>:  Named + Parametrized {
    fn number_of_possible_features(&self, problem: &P) -> usize;
    fn project(&self, genome: V) -> F;
    fn default_features(&self) -> F;
}

