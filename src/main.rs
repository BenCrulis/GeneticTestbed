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
use problems::Environment;
use crate::organism::grid::Grid;
use crate::features::FeatureMapper;
use std::hash::Hash;

use problems::travelling_salesman::{
    TSPValue,
    TSPRandomSolution,
    TSPInstance};
use crate::problems::travelling_salesman::{TSPFeatureMapper, SimpleTSPInstanceGenerator};
use std::rc::Rc;
use crate::organism::Genome;
use crate::problems::{DiscreteHyperparameters, ContinuousHyperparameters};

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


trait Config {
    fn get_problem_config_parameters(&self) -> ParameterConfig;
    fn execute(&self) -> Box<dyn Iterator<Item=Box<dyn Iterator<Item=Iteration>>>>;
}


trait AlgorithmExec<V,P,F,H> {
//    fn initialize_grid(&mut self);
    fn exec(&self, problem: &P) -> Box<dyn Iterator<Item=Iteration>>;
}



#[derive(Clone)]
struct AlgoConfig<V,P,F,TF,H> {
    elitism: Rc<dyn Elitism>,
    replacement_selection: Rc<dyn ReplacementSelection<V,F,P,H,TF>>
}

#[derive(Copy, Clone)]
struct CommonParameters {
    population_size: usize,
    number_of_repetitions: u64,
    number_of_iterations: u64,
}

impl Parametrized for CommonParameters {
    fn parameters(&self) -> HashMap<String, Parameter, RandomState> {
        let mut hm = HashMap::new();
        hm.insert("population size".to_string(),Parameter::Integer(self.population_size as i64));
        hm.insert("total repetitions".to_string(), Parameter::Integer(self.number_of_repetitions as i64));
        hm.insert("iterations per run".to_string(), Parameter::Integer(self.number_of_iterations as i64));
        return hm;
    }
}


#[derive(Clone)]
struct ProblemConfig<V,P,F,H> {
    random_organism_generator: Rc<dyn OrganismGenerator<V,P>>,
    problem_instance_generator: Rc<dyn ProblemInstanceGenerator<P>>,
    scorer_generator: Rc<dyn Scoring<V,P>>,
    feature_mapper: Rc<dyn FeatureMapper<V, F, P>>,
    constant_hyperparameters: H,
    hyperparameter_mapper: Rc<dyn Environment<H>>
}

/*
#[derive(Clone)]
struct AllConfig<V,P,F,H> {
    algorithm_configs: Rc<AlgoConfig<V,P,F>>,
    problem_config: Rc<ProblemConfig<V,P,F,H>>,
    common_config: Rc<CommonParameters>,
}
*/


impl<V: Genome<H=H,P=P>,P,F,TF,H> AlgorithmExec<V,P,F,H> for AlgoConfig<V,P,F,TF,H> {
    fn exec(&self, problem: &P) -> Box<dyn Iterator<Item=Iteration>> {

        /*
        let gr = self.replacement_selection.initialize_grid();

        Box::new(AlgorithmState {
            problem_state: Rc::new(unimplemented!()),
            grid: unimplemented!(),
            i: 0
        });
        */
        unimplemented!()
    }
}


struct MyConfig<V,P,F,H> {
    problem_config: Rc<ProblemConfig<V,P,F,H>>,
    common_config: Rc<CommonParameters>,
    algorithms: Vec<Rc<dyn AlgorithmExec<V,P,F,H>>>,
}
impl<V,P,F,H> Clone for MyConfig<V,P,F,H> {
    fn clone(&self) -> Self {
        MyConfig {
            problem_config: self.problem_config.clone(),
            common_config: self.common_config.clone(),
            algorithms: self.algorithms.clone()
        }
    }
}


struct MyConfigIt<V,P,F,H> {
    my_config: Rc<MyConfig<V,P,F,H>>,
    instance: P,
    repetitions: u64,
    index_algo: usize
}

impl<V,P,F,H> Iterator for MyConfigIt<V,P,F,H> {
    type Item = Box<dyn Iterator<Item=Iteration>>;

    fn next(&mut self) -> Option<Self::Item> {

        if self.repetitions > self.my_config.common_config.number_of_repetitions {
            return None;
        }
        else {
            if self.index_algo >= self.my_config.algorithms.len() {
                self.index_algo = 0;
                self.repetitions += 1;
                self.instance = self.my_config.problem_config.problem_instance_generator.generate_problem();
            }
            else {
                let algo = self.my_config.algorithms.get(self.index_algo).unwrap();
                return Some(algo.exec(&self.instance));
            }
        }

        return None;
    }
}

impl<V: 'static,P: 'static,F: 'static,H: 'static> Config for MyConfig<V,P,F,H> {
    fn get_problem_config_parameters(&self) -> ParameterConfig {
        // TODO
        return self.common_config.parameters()
    }

    fn execute(&self) -> Box<dyn Iterator<Item=Box<dyn Iterator<Item=Iteration>>>> {
        Box::new(MyConfigIt{
            my_config: Rc::new(self.clone()),
            instance: self.problem_config.problem_instance_generator.generate_problem(),
            repetitions: 0,
            index_algo: 0
        })
    }
}

struct ProblemState<V,P,F,H> {
    problem_config: Rc<ProblemConfig<V,P,F,H>>,
    common_config: Rc<CommonParameters>,
    algorithms: Vec<Rc<dyn AlgorithmExec<V,P,F,H>>>,
    instance: P,
    repetitions: u64
}


struct AlgorithmState<V,P,F,H> {
    problem_state: Rc<ProblemState<V,P,F,H>>,
    grid: Grid<V,F>,
    i: u64
}

impl<V,P,F,H> Iterator for AlgorithmState<V,P,F,H> {
    type Item = Iteration;

    fn next(&mut self) -> Option<Self::Item> {

        if self.i >= self.problem_state.common_config.number_of_iterations {

            self.i += 1;
        }

        unimplemented!()
    }
}



fn simple_metropolis_ga<V,P,F,H: Copy + 'static>(hyperparameters: H) -> Rc<AlgoConfig<V,P,F,(),H>> where V: Genome<H=H,P=P> {
    return Rc::new(AlgoConfig {
        elitism: Rc::new(MetropolisHastings{}),
        replacement_selection: Rc::new(SimpleReplacement{})
    })
}

fn tsp_problem_config() -> Rc<ProblemConfig<TSPValue<usize>,TSPInstance<usize>,Vec<usize>,DiscreteHyperparameters>> {
    Rc::new(ProblemConfig {
        random_organism_generator: Rc::new(TSPRandomSolution{}),
        problem_instance_generator: Rc::new(SimpleTSPInstanceGenerator{ number_of_cities: 100 }),
        scorer_generator: unimplemented!(),
        feature_mapper: Rc::new(TSPFeatureMapper{ number_cities_mapped: 2 }),
        constant_hyperparameters: DiscreteHyperparameters{ mutation_chance: 0.2 },
        hyperparameter_mapper: unimplemented!()
    })
}


fn main() {
    println!("Hello, world!");

    let common_config = CommonParameters {
        population_size: 100,
        number_of_repetitions: 10,
        number_of_iterations: 10
    };

    let target_population_size = 100_usize;


    let configs: Vec<Rc<dyn Config>> = vec![Rc::new(MyConfig {
        problem_config: tsp_problem_config(),
        common_config: Rc::new(common_config),
        algorithms: vec![simple_metropolis_ga::<TSPValue<usize>,TSPInstance<usize>, Vec<usize>, DiscreteHyperparameters>(DiscreteHyperparameters{mutation_chance: 0.2})]
    })];

    for mut config in configs {
        let p_params = config.get_problem_config_parameters();
        for it in config.execute() {
            for it2 in it {

            }
        }
    }
}
