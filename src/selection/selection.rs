use self::super::super::common::Named;
use super::super::organism::organism::{Organism};
use super::super::evaluation::scoring::Scoring;
use super::super::organism::grid::Grid;

use rand::{thread_rng, Rng};
use crate::organism::{Genome, OrganismGenerator};
use crate::features::FeatureMapper;
use std::collections::HashMap;

pub trait ReplacementSelection<V,F,P>: Named {
    fn initialize_grid(
        &self,
        pop_size: usize,
        feature_mapper: &dyn FeatureMapper<V,F, P>,
        problem: &P,
        generator: &dyn OrganismGenerator<V,P>) -> Grid<V,F>;
    fn select_replace(&self, grid: &mut Grid<V,F>, scorer: &dyn Scoring<Genotype=V>);
}

#[derive(Copy, Clone)]
pub struct SimpleReplacement {}

impl Named for SimpleReplacement {
    fn name(&self) -> String {
        String::from("SimpleReplacement")
    }
}

impl<V,P> ReplacementSelection<V,(),P> for SimpleReplacement {
    fn initialize_grid(
        &self,
        pop_size: usize,
        feature_mapper: &dyn FeatureMapper<V, (), P>,
        problem: &P,
        generator: &dyn OrganismGenerator<V, P>) -> Grid<V, ()> {
        let mut gr = vec![];
        for i in 0..pop_size {
            let mut hm = HashMap::new();
            hm.insert((),generator.generate_organism(problem));
            gr.push(hm);
        }

        return Grid{
            cells: gr
        }
    }


    fn select_replace(&self, grid: &mut Grid<V, ()>, scorer: &dyn Scoring<Genotype=V>) {
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
        return score_a > score_b;
    }
}