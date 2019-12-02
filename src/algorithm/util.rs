use crate::organism::Organism;
use crate::scoring::Scorer;
use std::cmp::Ordering;

pub fn sorted_scores<V,P>(mut organisms: Vec<Organism<V>>, scorer: &dyn Scorer<V,P>, problem: &P) -> Vec<f64> {

    let mut scores: Vec<f64> = organisms.drain(..).map(|org| org.only_score(scorer, problem)).collect();
    scores.sort_by(|x,y| y.partial_cmp(x).unwrap_or(Ordering::Less));

    scores
}






