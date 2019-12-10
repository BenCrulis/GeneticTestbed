use super::super::common::Named;
use crate::common::Parametrized;
use std::rc::Rc;
use crate::algorithm::mutation::Mutator;
use crate::scoring::Scorer;

#[derive(Copy, Clone, PartialEq)]
pub struct Organism<T> {
    pub genotype: T,
    score: Option<f64>
}

impl<T> Organism<T> {

    pub fn mutate<H>(&mut self, mutator: &dyn Mutator<T,H>, hyperparameters: &H) -> bool {
        let changed = mutator.mutate(&mut self.genotype, hyperparameters);
        self.score = None;
        return changed;
    }

    pub fn get_score(&self) -> Option<f64> {
        self.score
    }

    pub fn only_score<P>(&self, scorer: &dyn Scorer<T,P>, problem: &P) -> f64 {
        scorer.score(&self.genotype, problem)
    }

    pub fn score_with_cache<P>(&mut self, scorer: &dyn Scorer<T,P>, problem: &P) -> f64 {
        match self.score {
            None => {
                let s = scorer.score(&self.genotype, problem);
                self.score = Some(s);
                s
            }
            Some(s) => s
        }
    }
}

impl<V: PartialEq> Organism<V> {
    pub fn same_genetic_code(&self, other: &Organism<V>) -> bool {
        return self.genotype.eq(&other.genotype);
    }
}

pub trait Metric {
    fn distance_to(&self, other: &Self) -> f64;
}

impl<V: Metric> Metric for Organism<V> {
    fn distance_to(&self, other: &Organism<V>) -> f64 {
        self.genotype.distance_to(&other.genotype)
    }
}

pub trait OrganismGenerator<V,P>: Named + Parametrized {
    fn generate(&self, problem: &P) -> V;
    fn generate_organism(&self, problem: &P) -> Organism<V> {
        return Organism{genotype: self.generate(problem),
                        score: Option::None}
    }
}