use std::vec::Vec;
use std::collections::HashMap;

extern crate ordered_float;
use ordered_float::OrderedFloat;
use std::hash::Hash;

use self::super::super::common::*;
use self::super::super::features::FeatureMapper;
use crate::organism::{OrganismGenerator};
use crate::organism::Organism;
use rand::{thread_rng, Rng};
use rand::prelude::SliceRandom;
use std::ops::Range;

use super::ProblemInstanceGenerator;
use crate::problems::DiscreteHyperparameters;
use std::rc::Rc;
use crate::scoring::Scorer;
use crate::algorithm::mutation::Mutator;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Hash)]
pub struct TSPValue<T> {
    pub permutation: Vec<T>
}

#[derive(Copy, Clone)]
pub struct TSPScorer {}

impl<T: Eq + Hash + Clone> Scorer<TSPValue<T>, TSPInstance<T>> for TSPScorer {
    fn score(&self, genome: &TSPValue<T>, problem: &TSPInstance<T>) -> f64 {
        let mut sum = 0.0;
        for (x,y) in genome.permutation.iter().zip(
            genome.permutation.iter().skip(1).cycle()) {
            let t = (x.clone(), y.clone());
            sum += *problem.distances.get(&t).unwrap();
        }

        return problem.max_dist*genome.permutation.len() as f64-sum;
    }
}

#[derive(Copy, Clone)]
pub struct TSPMutator {}

impl<T: Clone> Mutator<TSPValue<T>, DiscreteHyperparameters> for TSPMutator {
    fn mutate(&self, genome: &mut TSPValue<T>, hyperparameters: &DiscreteHyperparameters) {
        let mut cities = &mut genome.permutation;

        let mut rng = thread_rng();

        while rng.gen::<f64>() < hyperparameters.mutation_chance {

            let index_a = rng.gen_range(0,cities.len());

            let mut index_b= rng.gen_range(0, cities.len());
            while index_b == index_a {
                index_b= rng.gen_range(0, cities.len());
            }

            let tmp = cities[index_b].clone();
            cities[index_b] = cities[index_a].clone();
            cities[index_a] = tmp;
        }
    }
}

#[derive(Clone)]
pub struct TSPInstance<T> {
    distances: HashMap<(T, T), f64>,
    max_dist: f64,
    min_dist: f64,
    number_of_cities: usize
}

pub struct TSPFeatureMapper {
    pub number_cities_mapped: usize
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

impl<T: Hash + Clone + Eq> FeatureMapper<TSPValue<T>, Vec<T>,TSPInstance<T>> for TSPFeatureMapper {
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

    fn default_features(&self) -> Vec<T> {
        Vec::new()
    }
}


impl<T> TSPInstance<T> {
    fn new(distances: HashMap<(T, T), f64>, number_of_cities: usize) -> Self {
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

impl OrganismGenerator<TSPValue<usize>,TSPInstance<usize>> for TSPRandomSolution {
    fn generate(&self, problem: Rc<TSPInstance<usize>>) -> TSPValue<usize> {
        let mut v: Vec<usize> = (0..problem.number_of_cities).collect();
        let mut rng = thread_rng();

        v.shuffle(&mut rng);

        return TSPValue{permutation: v};
    }
}


pub struct SimpleTSPInstanceGenerator {
    pub number_of_cities: usize
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

impl<'a> ProblemInstanceGenerator<TSPInstance<usize>> for SimpleTSPInstanceGenerator {
    fn generate_problem(&self) -> TSPInstance<usize> {
        let mut dists = HashMap::new();
        let mut rng = thread_rng();

        for i in 0..self.number_of_cities {
            for j in (i+1)..self.number_of_cities {

                let dist = rng.gen_range(0.0,100.0);

                dists.insert((i,j), dist);
                dists.insert((j,i), dist);
            }
        }

        return TSPInstance::new(dists, self.number_of_cities)
    }
}