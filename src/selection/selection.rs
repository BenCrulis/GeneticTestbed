use self::super::super::common::Named;
use rand::{thread_rng, Rng};

pub trait Elitism: Named {
    fn chose(&self, score_a: f64, score_b: f64) -> bool;
}

pub struct MetropolisHastings {}

impl Named for MetropolisHastings {
    fn name(&self) -> String {
        String::from("Metropolis-Hastings")
    }
}

impl Elitism for MetropolisHastings {
    fn chose(&self, score_a: f64, score_b: f64) -> bool {
        let mut trng = thread_rng();
        return trng.gen::<f64>() < score_a/score_b;
    }
}

pub struct GreedySelection {}

impl Named for GreedySelection {
    fn name(&self) -> String {
        String::from("Greedy_selection")
    }
}

impl Elitism for GreedySelection {
    fn chose(&self, score_a: f64, score_b: f64) -> bool {
        return score_a > score_b;
    }
}