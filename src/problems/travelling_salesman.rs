use std::vec::Vec;
use std::collections::HashMap;
use super::super::Scoring;

extern crate ordered_float;
use ordered_float::OrderedFloat;
use std::hash::Hash;

use self::super::super::common::*;
use self::super::super::features::FeatureMapper;
use crate::organism::{OrganismGenerator, Genome};
use crate::organism::Organism;
use rand::{thread_rng, Rng};
use rand::prelude::SliceRandom;
use std::ops::Range;

use super::ProblemInstanceGenerator;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Hash)]
pub struct TSPValue<T> {
    pub permutation: Vec<T>
}

pub struct TSPHyperparameters {
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


pub struct TSPInstance<'a, T> {
    distances: HashMap<(&'a T, &'a T), f64>,
    max_dist: f64,
    min_dist: f64,
    number_of_cities: usize
}

pub struct TSPFeatureMapper {
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

impl<T: Hash + Clone + Eq> FeatureMapper<TSPValue<T>, Vec<T>,TSPInstance<'_,T>> for TSPFeatureMapper {
    fn number_of_possible_features(&self, problem: &TSPInstance<T>) -> usize {
        let mut n = problem.number_of_cities;
        let mut r = 1;
        for i in 0..self.number_cities_mapped {
            r *= n;
            n -= 1;
        }

        return r;
    }

    fn project(&self, genome: TSPValue<T>) -> Vec<T> {
        genome.permutation[..self.number_cities_mapped].to_vec()
    }
}



impl<'a, T> TSPInstance<'a, T> {
    fn new(distances: HashMap<(&'a T, &'a T), f64>, number_of_cities: usize) -> Self {
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

pub struct TSPRandomSolution{}

impl TSPRandomSolution {
    pub fn new() -> Self {
        return TSPRandomSolution{};
    }
}

impl Named for TSPRandomSolution {
    fn name(&self) -> String {
        String::from("TSP simple generator")
    }
}

impl Parametrized for TSPRandomSolution {}

impl OrganismGenerator<TSPValue<usize>,TSPInstance<'_,usize>> for TSPRandomSolution {
    fn generate(&self, problem: &TSPInstance<usize>) -> TSPValue<usize> {
        let mut v: Vec<usize> = (0..problem.number_of_cities).collect();
        let mut rng = thread_rng();

        v.shuffle(&mut rng);

        return TSPValue{permutation: v};
    }
}


struct SimpleTSPInstanceGenerator {
    number_of_cities: usize
}

impl Named for SimpleTSPInstanceGenerator {
    fn name(&self) -> String {
        String::from("Simple TSP instance generator")
    }
}

impl Parametrized for SimpleTSPInstanceGenerator {
    fn parameters(&self) -> HashMap<String,Parameter> {
        let mut hm = HashMap::new();
        hm.insert("number_of_cities_generated".to_string(),
                  Parameter::Integer(self.number_of_cities as i64));
        return hm;
    }
}

impl<'a> ProblemInstanceGenerator<TSPInstance<'a,usize>> for SimpleTSPInstanceGenerator {
    fn generate_problem(&self) -> TSPInstance<'a,usize> {
        let mut dists = HashMap::new();

        // TODO

        return TSPInstance::new(dists, self.number_of_cities)
    }
}