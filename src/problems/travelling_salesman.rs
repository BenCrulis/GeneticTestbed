use std::vec::Vec;
use std::collections::HashMap;
use super::super::Scoring;

extern crate ordered_float;
use ordered_float::OrderedFloat;
use std::hash::Hash;

use self::super::super::common::{Named, Parametrized, Parameter};
use self::super::super::features::FeatureMapper;
use crate::organism::{OrganismGenerator, Genome};
use crate::organism::Organism;
use rand::{thread_rng, Rng};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Hash)]
pub struct TSPValue<T> {
    pub permutation: Vec<T>
}

struct TSPHyperparameters {
    mutation_chance: f64
}

impl<T: Clone> Genome<TSPHyperparameters> for TSPValue<T> {
    fn mutate(&self, hyperparameters: &TSPHyperparameters) -> Self where Self: Sized {
        let mut new = self.permutation.clone();

        let mut rng = thread_rng();

        while rng.gen::<f64>() < hyperparameters.mutation_chance {

            let index_a = rng.gen_range(0,new.len());

            let index_b= rng.gen_range(0, new.len());
            while index_b == index_a {

            }
        }
        return TSPValue{permutation: new};
    }
}


struct TSPInstance<'a, T> {
    distances: &'a HashMap<(&'a T, &'a T), f64>,
    max_dist: f64,
    min_dist: f64,
    number_of_cities: usize
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



impl<'a, T> TSPInstance<'a, T> {
    fn new(distances: &'a HashMap<(&'a T, &'a T), f64>, number_of_cities: usize) -> Self {
        let max_dist: f64 = distances.values().map(|x| OrderedFloat::from(*x)).max().unwrap().into();
        let min_dist: f64 = distances.values().map(|x| OrderedFloat::from(*x)).min().unwrap().into();
        TSPInstance {
            distances,
            max_dist,
            min_dist,
            number_of_cities
        }
    }
}


impl<T: Eq + Hash> Scoring for TSPInstance<'_, T> {
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

struct TSPRandomSolution{}

impl Named for TSPRandomSolution {
    fn name(&self) -> String {
        String::from("TSP simple generator")
    }
}

impl Parametrized for TSPRandomSolution {

}

impl<T> OrganismGenerator<TSPValue<T>,TSPInstance<'_,T>> for TSPRandomSolution {
    fn generate(&self, problem: &TSPInstance<T>) -> TSPValue<T> {
        unimplemented!()
    }
}
