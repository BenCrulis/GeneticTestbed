use self::super::super::common::Named;
use super::super::organism::organism::{Organism};
use super::super::organism::grid::Grid;

use rand::{thread_rng, Rng};
use crate::organism::{OrganismGenerator};
use crate::features::FeatureMapper;
use std::collections::HashMap;



pub trait Elitism: Named {
    fn choose(&self, score_a: f64, score_b: f64) -> bool;
}

#[derive(Copy, Clone)]
pub struct MetropolisHastings {}

impl Named for MetropolisHastings {
    fn name(&self) -> String {
        String::from("Metropolis-Hastings")
    }
}

impl Elitism for MetropolisHastings {
    fn choose(&self, score_a: f64, score_b: f64) -> bool {
        let mut trng = thread_rng();
        assert!(score_a >= 0.0 && score_b >= 0.0);
        return trng.gen::<f64>() < score_a/score_b;
    }
}

#[derive(Copy, Clone)]
pub struct GreedySelection {}

impl Named for GreedySelection {
    fn name(&self) -> String {
        String::from("Greedy_selection")
    }
}

impl Elitism for GreedySelection {
    fn choose(&self, score_a: f64, score_b: f64) -> bool {
        return score_a >= score_b;
    }
}