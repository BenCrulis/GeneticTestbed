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
use std::hash::Hash;
use crate::problems::travelling_salesman::TSPValue;
use crate::organism::grid::Grid;
use crate::features::FeatureMapper;


struct MutationHyperparameters {
    mutation_chance: f64
}



trait HyperparameterMapper<H>: Named {
    fn map_hyperparameters(&self, coordinates: &Vec<(usize, usize)>) -> H;
}




trait ProblemInstanceGenerator<P>: Named {
    fn generate_problem(&self) -> P;
}

struct Placeholder;


trait AlgorithmConfig {
    fn test(&self);
}

#[derive(Copy, Clone)]
struct AlgoConfig<'a,V,F> {
    elitism: &'a dyn Elitism,
    replacement_selection: &'a dyn ReplacementSelection<V,F>
}

impl<'a,V,F> AlgorithmConfig for AlgoConfig<'a,V,F> {
    fn test(&self) {
        unimplemented!()
    }
}

#[derive(Copy, Clone)]
struct CommonParameters<'a, H> {
    number_of_repetitions: u64,
    number_of_iteration: u64,
    constant_hyperparameters: H,
    hyperparameter_mapper: &'a dyn HyperparameterMapper<H>
}

#[derive(Copy, Clone)]
struct ProblemConfig<'a,V,P,F> {
    random_organism_generator: &'a dyn OrganismGenerator<V>,
    problem_instance_generator: &'a dyn ProblemInstanceGenerator<P>,
    scorer_generator: &'a dyn Scoring<Genotype=&'a V>,
    feature_mapper: &'a dyn FeatureMapper<V, F>
}

#[derive(Clone)]
struct GeneralConfig<'a,V,P,F> {
    problem_config: ProblemConfig<'a,V,P,F>,
    algorithm_configs: Vec<&'a dyn AlgorithmConfig>
}

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


trait Config<'a, H> {
    fn get_problem_config_parameters(&self) -> HashMap<String,String>;
    fn execute(&'a self, common_parameters: &'a CommonParameters<'a, H>) -> Box<dyn Iterator<Item=Iteration> + 'a>;
}


#[derive(Clone)]
struct AlgoState<'a,'b,V,P,H,F> {
    algorithm_configs: Vec<&'a dyn AlgorithmConfig>,
    problem_config: &'a ProblemConfig<'a,V,P,F>,
    common_config: &'b CommonParameters<'b, H>,
    repetition: u64,
    i: u64,
    index_algo: usize,
    problem: P,
    grid: Grid<V,F>
}

impl<'a,'b,V,P,H,F> Iterator for AlgoState<'a,'b,V,P,H,F> {
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

        let actual_algo = self.algorithm_configs.get(self.index_algo).unwrap();

        if self.i >= self.common_config.number_of_iteration {
            self.i = 0;
            if self.index_algo >= self.algorithm_configs.len() {
                self.index_algo = 0;
                if self.repetition >= self.common_config.number_of_repetitions {
                    return None;
                } else {
                    self.repetition += 1;
                    self.problem = self.problem_config.problem_instance_generator.generate_problem()
                }
            } else {
                self.index_algo += 1;
            }
        } else {
            self.i += 1;
        }

        unimplemented!()
    }
}


impl<'a, H, V, P, F> Config<'a, H> for GeneralConfig<'a,V, P, F> {
    fn get_problem_config_parameters(&self) -> HashMap<String, String, RandomState> {
        unimplemented!()
    }

    fn execute(&'a self, common_parameters: &'a CommonParameters<'a, H>) -> Box<dyn Iterator<Item=Iteration> + 'a> {
        Box::new(AlgoState {
            algorithm_configs: self.algorithm_configs.clone(),
            problem_config: &self.problem_config,
            common_config: common_parameters,
            repetition: 0,
            i: 0,
            index_algo: 0,
            problem: self.problem_config.problem_instance_generator.generate_problem(),
            grid: Grid { cells: vec![] }
        })
    }
}



fn main() {
    println!("Hello, world!");

    let common_config = CommonParameters {
        number_of_repetitions: 10,
        number_of_iteration: 10,
        constant_hyperparameters: (),
        hyperparameter_mapper: unimplemented!()
    };

    let configs: Vec<&mut dyn Config<()>> = vec![];

    for mut config in configs {
        let p_params = config.get_problem_config_parameters();
        for it in config.execute(&common_config) {

        }
    }
}
