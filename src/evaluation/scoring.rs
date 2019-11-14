use super::super::organism::organism::Organism;
use std::collections::HashMap;

pub trait Scoring {
    type Genotype;

    fn evaluate(&self, organism: Organism<Self::Genotype>, override_cache: bool) -> Organism<Self::Genotype> {
        let Organism{genotype, score} = organism;
        if override_cache || score.is_none() {
            let new_score = Option::Some(self.score(&genotype));
            Organism {
                genotype,
                score: new_score
            }
        }
        else {
            Organism{genotype, score}
        }
    }

    fn score(&self, genotype: &Self::Genotype) -> f64;

    fn config_attributes(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}