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
use std::rc::Rc;

#[derive(Clone)]
struct Iteration {
    iteration: u64,
    repetition: u64,
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





trait AlgorithmExec<P> {
//    fn initialize_grid(&mut self);
    fn step(&mut self, problem: &P) -> Option<Iteration>;
}



#[derive(Clone)]
struct AlgoConfig<V,P,F> {
    elitism: Rc<dyn Elitism>,
    replacement_selection: Rc<dyn ReplacementSelection<V,F,P>>
}

#[derive(Copy, Clone)]
struct CommonParameters {
    population_size: usize,
    number_of_repetitions: u64,
    number_of_iterations: u64,
}

#[derive(Clone)]
struct ProblemConfig<V,P,F,H> {
    random_organism_generator: Rc<dyn OrganismGenerator<V,P>>,
    problem_instance_generator: Rc<dyn ProblemInstanceGenerator<P>>,
    scorer_generator: Rc<dyn Scoring<Genotype=V>>,
    feature_mapper: Rc<dyn FeatureMapper<V, F, P>>,
    constant_hyperparameters: H,
    hyperparameter_mapper: Rc<dyn HyperparameterMapper<H>>
}

#[derive(Clone)]
struct AllConfig<V,P,F,H> {
    algorithm_configs: Rc<AlgoConfig<V,P,F>>,
    problem_config: Rc<ProblemConfig<V,P,F,H>>,
    common_config: Rc<CommonParameters>,
}

trait Config {
    fn get_problem_config_parameters(&self) -> ParameterConfig;
    fn execute(&self) -> Box<dyn Iterator<Item=Box<dyn Iterator<Item=Iteration>>>>;
}

impl<V: 'static,P: 'static ,F: 'static ,H: 'static> Config for Rc<AllConfig<V,P,F,H>> {
    fn get_problem_config_parameters(&self) -> HashMap<String, Parameter, RandomState> {
        unimplemented!()
    }

    fn execute(&self) -> Box<dyn Iterator<Item=Box<dyn Iterator<Item=Iteration>>>> {
        let problem = self.problem_config.problem_instance_generator.generate_problem();


        let it = ProblemState {
            all_config: self.clone(),
            instance: problem,
            repetitions: 0
        };

        return Box::new(it);
    }
}

impl<V,P,F,H> Iterator for ProblemState<V,P,F,H> {
    type Item = Box<dyn Iterator<Item=Iteration>>;

    fn next(&mut self) -> Option<Self::Item> {
        let gr = self.all_config.algorithm_configs.replacement_selection.initialize_grid(
            self.all_config.common_config.population_size,
            self.all_config.problem_config.feature_mapper.as_ref(),
            &self.instance,
            self.all_config.problem_config.random_organism_generator.as_ref()
        );


        unimplemented!()
    }
}


struct ProblemState<V,P,F,H> {
    all_config: Rc<AllConfig<V,P,F,H>>,
    instance: P,
    repetitions: u64
}


struct AlgorithmState<V,P,F,H> {
    problem_state: Rc<ProblemState<V,P,F,H>>,
    grid: Grid<V,F>,
    i: u64
}


impl<V,P,F,H> AlgorithmExec<P> for AlgorithmState<V,P,F,H> {
    fn step(&mut self, problem: &P) -> Option<Iteration> {
        if self.i >= self.problem_state.all_config.common_config.number_of_iterations {
            None
        }
        else {
            let mut iter_res = Iteration {
                iteration: 0,
                repetition: 0,
                timestamp: Instant::now(),
                best_score: 0.0,
                sum_scores: 0.0,
                min_score: 0.0,
                max_score: 0.0,
                number_of_organisms: 0,
                pop_score_variance: 0.0
            };


            Some(iter_res)
        }
    }
}


impl<V,P,F,H> Iterator for AlgorithmState<V,P,F,H> {
    type Item = Iteration;

    fn next(&mut self) -> Option<Self::Item> {

        let mut iter_res = Iteration {
            iteration: 0,
            repetition: 0,
            timestamp: Instant::now(),
            best_score: 0.0,
            sum_scores: 0.0,
            min_score: 0.0,
            max_score: 0.0,
            number_of_organisms: 0,
            pop_score_variance: 0.0
        };

        let actual_algo = self.problem_state.all_config.algorithm_configs.clone();
        let common_config = self.problem_state.all_config.common_config.clone();
        let problem_config = self.problem_state.all_config.problem_config.clone();

        if self.i >= common_config.number_of_iterations {
            return None;

        } else {
            self.i += 1;
        }

        unimplemented!()
    }
}




fn main() {
    println!("Hello, world!");

    let test: Vec<Box<dyn AlgorithmExec<()>>> = vec![];

    let target_population_size = 100_usize;


    let configs: Vec<Rc<dyn Config>> = vec![];

    for mut config in configs {
        let p_params = config.get_problem_config_parameters();
        for it in config.execute() {
            for it2 in it {

            }
        }
    }
}
