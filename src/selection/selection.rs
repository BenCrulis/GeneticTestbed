use self::super::super::common::Named;
use super::super::organism::organism::{Organism};
use super::super::evaluation::scoring::Scoring;
use super::super::organism::grid::Grid;

use rand::{thread_rng, Rng};

pub trait ReplacementSelection<V,F>: Named {
    fn select_replace(&self, grid: &mut Grid<V,F>, scorer: &dyn Scoring<Genotype=V>);
}

pub struct SimpleReplacement {}

impl Named for SimpleReplacement {
    fn name(&self) -> String {
        String::from("SimpleReplacement")
    }
}

impl<V,F> ReplacementSelection<V,F> for SimpleReplacement {
    fn select_replace(&self, grid: &mut Grid<V, F>, scorer: &dyn Scoring<Genotype=V>) {
        let size = grid.cells.len();
        let mut rng = thread_rng();
        let index_a = rng.gen_range(0,size);
        let mut index_b = rng.gen_range(0, size);
        while index_b == index_a {
            let mut index_b = rng.gen_range(0, size);
        }
    }
}

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