use super::super::organism::organism::Organism;
use std::collections::HashMap;

pub trait Scoring<V,P> {

    fn evaluate(&self, organism: Organism<V>, override_cache: bool) -> Organism<V> {
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

    fn score(&self, genotype: &V) -> f64;

    fn config_attributes(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}