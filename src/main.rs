#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

extern crate rand;
extern crate serde;

use std::vec::Vec;
use std::time::Instant;

mod common;
mod problems;
mod evaluation;
mod organism;
mod selection;
mod features;

use common::Named;
use common::Parametrized;
use common::Parameter;
use common::ParameterConfig;
use common::str_param;

use problems::rastrigin::{rastrigin,custom_rastrigin,regularized_rastrigin};
use evaluation::scoring::Scoring;
use organism::organism::Organism;
use organism::organism::OrganismGenerator;
use std::collections::HashMap;
use std::process::Output;
use std::collections::hash_map::RandomState;
use std::iter::Cycle;

use selection::{SimpleReplacement,ReplacementSelection};
use selection::Elitism;
use selection::MetropolisHastings;
use selection::GreedySelection;
use rand::{thread_rng, Rng};

use problems::ProblemInstanceGenerator;
use crate::organism::grid::Grid;
use crate::features::FeatureMapper;
use std::hash::Hash;

use problems::travelling_salesman::{
    TSPValue,
    TSPRandomSolution,
    TSPHyperparameters,
    TSPInstance};
use crate::problems::travelling_salesman::TSPFeatureMapper;

#[derive(Clone)]
struct Iteration {
    iteration: u64,
    repetition: u64,
    algo_config: HashMap<String,String>,
    timestamp: Instant,
    best_score: f64,
    sum_scores: f64,
    min_score: f64,
    max_score: f64,
    number_of_organisms: usize,
    pop_score_variance: f64,
}

struct MutationHyperparameters {
    mutation_chance: f64
}

trait HyperparameterMapper<H>: Named {
    fn map_hyperparameters(&self, coordinates: &Vec<(usize, usize)>) -> H;
}


trait AlgorithmExec : Iterator {
    fn initialize_grid(&mut self);
    fn step(&mut self) -> Iteration;
}


#[derive(Copy, Clone)]
struct AlgoConfig<'a,V,P,F> {
    elitism: &'a dyn Elitism,
    replacement_selection: &'a dyn ReplacementSelection<V,F,P>
}

#[derive(Copy, Clone)]
struct CommonParameters {
    population_size: usize,
    number_of_repetitions: u64,
    number_of_iterations: u64,
}

#[derive(Copy, Clone)]
struct ProblemConfig<'a,V,P,F,H> {
    random_organism_generator: &'a dyn OrganismGenerator<V,P>,
    problem_instance_generator: &'a dyn ProblemInstanceGenerator<P>,
    scorer_generator: &'a dyn Scoring<Genotype=&'a V>,
    feature_mapper: &'a dyn FeatureMapper<V, F, P>,
    constant_hyperparameters: H,
    hyperparameter_mapper: &'a dyn HyperparameterMapper<H>
}

#[derive(Copy, Clone)]
struct AllConfig<'a,V,F,P,H> {
    algorithm_configs: &'a AlgoConfig<'a,V,P,F>,
    problem_config: &'a ProblemConfig<'a,V,P,F,H>,
    common_config: &'a CommonParameters,
}

struct ProblemState<'a,V,P,F,H> {
    all_config: AllConfig<'a,V,P,F,H>,
    instance: P,
    repetitions: u64
}


struct AlgorithmState<'a,V,P,F,H> {
    all_config: AllConfig<'a,V,F,P,H>,
    grid: Grid<V,F>,
    repetition: u64,
    i: u64
}

trait Config<'a> {
    fn get_problem_config_parameters(&self) -> ParameterConfig;
    fn execute(&'a self) -> Box<dyn Iterator<Item=Box<dyn AlgorithmExec<Item=Iteration>>> + 'a>;
}


impl<'a,V,P,F,H> Iterator for AlgorithmState<'a,V,P,F,H> {
    type Item = Iteration;

    fn next(&mut self) -> Option<Self::Item> {


        let mut iter_res = Iteration {
            iteration: 0,
            repetition: 0,
            algo_config: Default::default(),
            timestamp: Instant::now(),
            best_score: 0.0,
            sum_scores: 0.0,
            min_score: 0.0,
            max_score: 0.0,
            number_of_organisms: 0,
            pop_score_variance: 0.0
        };

        let actual_algo = self.all_config.algorithm_configs;
        let common_config = self.all_config.common_config;
        let problem_config = self.all_config.problem_config;

        if self.i >= common_config.number_of_iterations {
            self.i = 0;
            if self.repetition >= common_config.number_of_repetitions {
                return None;
            } else {
                self.repetition += 1;
                //self.problem = problem_config.problem_instance_generator.generate_problem()
            }

        } else {
            self.i += 1;
        }

        unimplemented!()
    }
}




fn main() {
    println!("Hello, world!");

    let target_population_size = 100_usize;


    let configs: Vec<&mut dyn Config> = vec![];

    for mut config in configs {
        let p_params = config.get_problem_config_parameters();
        for it in config.execute() {

        }
    }
}
