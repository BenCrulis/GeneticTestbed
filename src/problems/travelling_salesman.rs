use std::vec::Vec;
use std::collections::HashMap;
use super::super::Scoring;

extern crate ordered_float;
use ordered_float::OrderedFloat;
use std::hash::Hash;

pub struct TSPValue<T> {
    permutation: Vec<T>
}

struct TSPEvaluator<'a, T> {
    distances: &'a HashMap<(&'a T, &'a T), f64>,
    max_dist: f64
}

impl<'a, T> TSPEvaluator<'a, T> {
    fn new(distances: &'a HashMap<(&'a T, &'a T), f64>) -> Self {
        let max_dist: f64 = distances.values().map(|x| OrderedFloat::from(*x)).max().unwrap().into();
        TSPEvaluator{
            distances,
            max_dist
        }
    }
}


impl<T: Eq + Hash> Scoring for TSPEvaluator<'_, T> {
    type Genotype = TSPValue<T>;

    fn score(&self, genotype: &Self::Genotype) -> f64 {

        let mut sum = 0.0;
        for x in genotype.permutation.iter().zip(
            genotype.permutation.iter().skip(1).cycle()) {
            sum += *self.distances.get(&x).unwrap();
        }

        return self.max_dist*genotype.permutation.len() as f64-sum;
    }
}
