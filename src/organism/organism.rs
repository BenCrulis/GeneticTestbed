use super::super::common::Named;
use crate::common::Parametrized;

#[derive(Copy, Clone)]
pub struct Organism<T> {
    pub genotype: T,
    pub score: Option<f64>
}

impl<T,H,P> Organism<T> where T: Genome<H=H,P=P>  {
    fn get_score(&mut self, problem: &P) -> f64 {
        self.genotype.score(problem)
    }

    fn score_with_cache(&mut self, problem: &P) -> f64 {
        match self.score {
            None => {
                let s = self.genotype.score(problem);
                self.score = Some(s);
                s
            }
            Some(s) => s
        }
    }
}


pub trait OrganismGenerator<V,P>: Named + Parametrized {
    fn generate(&self, problem: &P) -> V;
    fn generate_organism(&self, problem: &P) -> Organism<V> {
        return Organism{genotype: self.generate(problem),
                        score: Option::None}
    }
}

pub trait Genome: Clone + Sized {
    type H;
    type P;
    fn mutate(&self, hyperparameters: &Self::H) -> Self;
    fn score(&self, problem: &Self::P) -> f64;
}