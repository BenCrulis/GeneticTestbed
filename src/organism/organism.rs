use super::super::common::Named;
use crate::evaluation::scoring::Scoring;
use crate::common::Parametrized;

#[derive(Copy, Clone)]
pub struct Organism<T> {
    pub genotype: T,
    pub score: Option<f64>
}


pub trait OrganismGenerator<V,P>: Named + Parametrized {
    fn generate(&self, problem: &P) -> V;
    fn generate_organism(&self, problem: &P) -> Organism<V> {
        return Organism{genotype: self.generate(problem),
                        score: Option::None}
    }
}

pub trait Genome<H>: Clone + Sized {
    fn mutate(&self, hyperparameters: &H) -> Self where Self: Sized;
    fn score(&self, scorer: &dyn Scoring<Genotype=Self>) -> f64 {
        scorer.score(self)
    }
}