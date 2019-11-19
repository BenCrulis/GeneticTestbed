use std::hash::Hash;

use super::common::{Named, Parametrized};

pub trait FeatureMapper<G,T: Hash + Clone + Eq + PartialEq>:  Named + Parametrized {
    fn project(&self, genome: G) -> T;
}

