use std::vec::Vec;
use std::collections::HashMap;
use super::super::Scoring;

extern crate ordered_float;
use ordered_float::OrderedFloat;
use std::hash::Hash;

use self::super::super::common::{Named, Parametrized, Parameter};
use self::super::super::features::FeatureMapper;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Hash)]
pub struct TSPValue<T> {
    pub permutation: Vec<T>
}


struct TSPEvaluator<'a, T> {
    distances: &'a HashMap<(&'a T, &'a T), f64>,
    max_dist: f64
}

struct TSPFeatureMapper {
    number_cities_mapped: usize
}

impl Named for TSPFeatureMapper {
    fn name(&self) -> String {
        String::from("Keep first N cities mapper")
    }
}

impl Parametrized for TSPFeatureMapper {
    fn parameters(&self) -> HashMap<String,Parameter> {
        let mut params = HashMap::new();
        params.insert(String::from("number_of_cities_mapped"), Parameter::Integer(self.number_cities_mapped as i64));
        return params;
    }
}

impl<T: Hash + Clone + Eq> FeatureMapper<TSPValue<T>, Vec<T>> for TSPFeatureMapper {
    fn project(&self, genome: TSPValue<T>) -> Vec<T> {
        genome.permutation[..self.number_cities_mapped].to_vec()
    }
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
