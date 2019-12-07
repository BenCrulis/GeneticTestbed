#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

extern crate rand;
extern crate csv;
extern crate serde;
extern crate serde_json;
extern crate statistical;
extern crate chrono;

use std::vec::Vec;
use std::time::{Instant, SystemTime, Duration};

use serde_json::{json, Map, Value};

mod common;
mod problems;
mod organism;
mod algorithm;
mod features;
mod scoring;

use std::collections::HashMap;
use std::process::Output;
use std::collections::hash_map::RandomState;
use std::iter::{Cycle};
use std::rc::Rc;

use common::Named;
use common::Parametrized;
use common::{str_param,int_param};

use problems::rastrigin::{rastrigin,custom_rastrigin,regularized_rastrigin};
use problems::ProblemInstanceGenerator;
use problems::Environment;
use problems::travelling_salesman::{TSPFeatureMapper, SimpleTSPInstanceGenerator};
use problems::{DiscreteHyperparameters, ContinuousHyperparameters, SpatialMapper};
use problems::travelling_salesman::{
    TSPValue,
    TSPRandomSolution,
    TSPInstance};

use organism::grid::Grid;
use organism::organism::Organism;
use organism::organism::OrganismGenerator;

use algorithm::config::ProblemConfig;
use algorithm::selection::Elitism;
use algorithm::algorithm::ReplacementSelection;
use algorithm::selection::MetropolisHastings;
use algorithm::selection::GreedySelection;
use algorithm::simple::SimpleReplacement;
use algorithm::grid_ga;
use algorithm::map_elite;
use algorithm::util::sorted_scores;

use rand::{thread_rng, Rng};

use features::FeatureMapper;
use std::hash::Hash;
use crate::algorithm::algorithm::UpdatableSolver;
use crate::scoring::Scorer;
use crate::algorithm::mutation::Mutator;
use crate::problems::travelling_salesman::{TSPMutator, TSPScorer};
use statistical::*;
use serde::{Serialize, Deserialize, Serializer};
use std::fs::File;
use std::path::Path;
use std::io::{BufWriter, LineWriter, Write};
use crate::problems::Hyperparameter;

#[derive(Clone, Debug)]
struct Iteration {
    iteration: u64,
    repetition: u64,
    index_algo: usize,
    duration: Duration,
    sum_scores: f64,
    min_score: f64,
    max_score: f64,
    mean_score: f64,
    median_score: f64,
    number_of_organisms: usize,
    pop_score_variance: f64,
}

impl Iteration {
    fn write_header(writer: &mut csv::Writer<File>) -> Result<(), csv::Error> {
        writer.write_record(&[
            "repetition",
            "algorithm index",
            "iteration",
            "duration (ns)",
            "sum score",
            "min score",
            "max score",
            "mean score",
            "median score",
            "number of organisms",
            "variance"
        ])
    }
    fn write_row(&self, writer: &mut csv::Writer<File>) -> Result<(),csv::Error> {
        writer.write_record(&[
            self.repetition.to_string(),
            self.index_algo.to_string(),
            self.iteration.to_string(),
            self.duration.as_nanos().to_string(),
            self.sum_scores.to_string(),
            self.min_score.to_string(),
            self.max_score.to_string(),
            self.mean_score.to_string(),
            self.median_score.to_string(),
            self.number_of_organisms.to_string(),
            self.pop_score_variance.to_string()
        ])
    }
}

trait Config {
    fn get_problem_config_parameters(&self) -> serde_json::Value;
    fn get_common_config(&self) -> CommonParameters;
    fn number_of_algorithms(&self) -> usize;
    fn execute(&self) -> Box<dyn Iterator<Item=Box<dyn Iterator<Item=Iteration>>>>;
}


trait AlgorithmExec<V,P,F,H> {
    fn exec(&self, config: Rc<MyConfigIt<V,P,F,H>>) -> Box<dyn Iterator<Item=Iteration>>;
}



#[derive(Clone)]
struct AlgoConfig<V,P,F,H> {
    elitism: Rc<dyn Elitism>,
    replacement_selection: Rc<dyn ReplacementSelection<V,P,F,H>>
}


#[derive(Copy, Clone)]
struct CommonParameters {
    population_size: usize,
    number_of_repetitions: u64,
    number_of_iterations: u64,
}

impl Parametrized for CommonParameters {
    fn parameters(&self) -> serde_json::Value {
        let mut hm = Map::new();
        hm.insert("population size".to_string(),int_param(self.population_size as i64));
        hm.insert("total repetitions".to_string(), int_param(self.number_of_repetitions as i64));
        hm.insert("iterations per run".to_string(), int_param(self.number_of_iterations as i64));
        return serde_json::Value::Object(hm);
    }
}






struct MyConfig<V,P,F,H> {
    problem_config: Rc<ProblemConfig<V,P,F,H>>,
    common_config: Rc<CommonParameters>,
    algorithms: Vec<Rc<AlgoConfig<V,P,F,H>>>,
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
    instance: Rc<P>,
    repetitions: u64,
    index_algo: usize
}

impl<V,P,F,H> Clone for MyConfigIt<V,P,F,H> {
    fn clone(&self) -> Self {
        MyConfigIt {
            my_config: self.my_config.clone(),
            instance: self.instance.clone(),
            repetitions: self.repetitions,
            index_algo: self.index_algo
        }
    }
}


impl<V: 'static,P: 'static,F: 'static,H: 'static> Iterator for MyConfigIt<V,P,F,H> {
    type Item = Box<dyn Iterator<Item=Iteration>>;

    fn next(&mut self) -> Option<Self::Item> {
        let algo = self.my_config.algorithms.get(self.index_algo).unwrap();

        let updatable_solver = algo.replacement_selection.initialize_solver(
            self.my_config.common_config.population_size,
            self.instance.clone(),
            algo.elitism.clone(),
            self.my_config.problem_config.clone()
        );

        let ex = AlgorithmState {
            my_config_it: Rc::new(self.clone()),
            updatable_solver,
            i: 0
        };

        if self.repetitions >= self.my_config.common_config.number_of_repetitions {
            return None;
        }
        else {
            if self.index_algo >= self.my_config.algorithms.len()-1 {
                self.index_algo = 0;
                self.repetitions += 1;
                self.instance = Rc::new(self.my_config.problem_config.problem_instance_generator.generate_problem());
            }
            else {
                self.index_algo += 1;
            }
        }
        return Some(Box::new(ex));
    }
}

impl<V: 'static,P: 'static,F: 'static,H: 'static> Config for MyConfig<V,P,F,H> {
    fn get_problem_config_parameters(&self) -> serde_json::Value {
        // TODO
        println!("Getting MyConfig parameters");
        let common_params = self.common_config.parameters();
        let mut algo_configs = Vec::new();

        for (i,algo) in self.algorithms.iter().enumerate() {
            let mut algo_config = Map::new();
            algo_config.insert("algorithm index".to_string(), int_param(i as i64));
            algo_config.insert("elitism".to_string(), serde_json::Value::String(algo.elitism.name()));
            algo_config.insert("algorithm config".to_string(), algo.replacement_selection.parameters());
            algo_configs.push(serde_json::Value::Object(algo_config));
        }

        let mut final_config = Map::new();

        final_config.insert("common".to_string(), common_params);
        final_config.insert("algorithms".to_string(), serde_json::Value::Array(algo_configs));

        return serde_json::Value::Object(final_config);
    }

    fn get_common_config(&self) -> CommonParameters {
        *self.common_config
    }

    fn number_of_algorithms(&self) -> usize {
        self.algorithms.len()
    }

    fn execute(&self) -> Box<dyn Iterator<Item=Box<dyn Iterator<Item=Iteration>>>> {
        Box::new(MyConfigIt{
            my_config: Rc::new(self.clone()),
            instance: Rc::new(self.problem_config.problem_instance_generator.generate_problem()),
            repetitions: 0,
            index_algo: 0
        })
    }
}

struct ProblemState<V,P,F,H> {
    problem_config: Rc<ProblemConfig<V,P,F,H>>,
    common_config: Rc<CommonParameters>,
    algorithms: Vec<Rc<dyn AlgorithmExec<V,P,F,H>>>,
    instance: Rc<P>,
    repetitions: u64
}


struct AlgorithmState<V,P,F,H> {
    my_config_it: Rc<MyConfigIt<V,P,F,H>>,
    updatable_solver: Box<dyn UpdatableSolver<V>>,
    i: u64
}

impl<V,P,F,H> Iterator for AlgorithmState<V,P,F,H> {
    type Item = Iteration;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.my_config_it.my_config.common_config.number_of_iterations {
            None
        }
        else {
            let before = Instant::now();

            let organisms = self.updatable_solver.update();

            let duration = Instant::now().duration_since(before);

            let number_of_organisms = organisms.len();

            /*
            println!("Parameters: {}\nNumber of organism: {}",
                     self.my_config_it.my_config.algorithms[self.my_config_it.index_algo].replacement_selection.parameters(),
                    number_of_organisms);
            */

            let sorted_score = sorted_scores(
                organisms,
                self.my_config_it.my_config.problem_config.scorer.as_ref(),
                self.my_config_it.instance.as_ref());

            //println!("Scores: {:?}", &sorted_score);

            let mean_val = mean(sorted_score.as_slice());
            let iter = Iteration {
                iteration: self.i,
                repetition: self.my_config_it.repetitions+1,
                index_algo: self.my_config_it.index_algo,
                duration,
                sum_scores: sorted_score.iter().sum(),
                min_score: *sorted_score.last().unwrap(),
                max_score: *sorted_score.first().unwrap(),
                mean_score: mean_val,
                median_score: median(sorted_score.as_slice()),
                number_of_organisms,
                pop_score_variance: variance(sorted_score.as_slice(), Some(mean_val))
            };

            self.i += 1;
            Some(iter)
        }
    }
}



fn simple_metropolis_ga<V: Clone + 'static,P: 'static,F: 'static,H: Copy + 'static>() -> Rc<AlgoConfig<V,P,F,H>> {
    return Rc::new(AlgoConfig {
        elitism: Rc::new(GreedySelection{}),
        replacement_selection: Rc::new(SimpleReplacement{})
    })
}

fn grid_ga<V: 'static + Clone + PartialEq,
    P: 'static,
    F: 'static + Eq + Clone + Hash,
    H: 'static + Hyperparameter + Clone>(
        use_features: bool,
        use_hyperparameter_mapping: bool,
        number_of_spatial_dimensions: usize) -> Rc<AlgoConfig<V,P,F,H>> {
    return Rc::new(AlgoConfig {
        elitism: Rc::new(GreedySelection{}),
        replacement_selection: Rc::new(grid_ga::GeneralizedMAPElite {
            use_features,
            use_hyperparameter_mapping,
            number_of_spatial_dimensions
        })
    });
}


fn all_algos_configs<V: 'static + Clone + PartialEq,P: 'static ,F: 'static + Eq + Clone + Hash ,H: 'static + Hyperparameter + Copy + Clone>() -> Vec<Rc<AlgoConfig<V,P,F,H>>> {
    vec![simple_metropolis_ga(),
         grid_ga(false, false, 1),
         grid_ga(true, false, 1),
         grid_ga(false, true, 1),
         grid_ga(true, true, 1),
        Rc::new(AlgoConfig {
            elitism: Rc::new(GreedySelection{}),
            replacement_selection: Rc::new(map_elite::MAPElite{})
        })]
}

fn tsp_problem_config() -> Rc<ProblemConfig<TSPValue<usize>,TSPInstance<usize>,Vec<usize>,DiscreteHyperparameters>> {
    Rc::new(ProblemConfig {
        random_organism_generator: Rc::new(TSPRandomSolution{}),
        problem_instance_generator: Rc::new(SimpleTSPInstanceGenerator{ number_of_cities: 50,
            number_of_dimensions: 2}),
        feature_mapper: Rc::new(TSPFeatureMapper{ number_cities_mapped: 1 }),
        constant_hyperparameters: DiscreteHyperparameters{ mutation_chance: 0.5 },
        hyperparameter_mapper: Rc::new(SpatialMapper{ number_of_additional_dimensions: 0 }),
        mutator: Rc::new(TSPMutator{}),
        scorer: Rc::new(TSPScorer{})
    })
}



fn main() {
    let file_prefix = "long";

    let common_config = CommonParameters {
        population_size: 2500,
        number_of_repetitions: 1,
        number_of_iterations: 1000000
    };

    let configs: Vec<Rc<dyn Config>> = vec![Rc::new(MyConfig {
        problem_config: tsp_problem_config(),
        common_config: Rc::new(common_config),
        algorithms: all_algos_configs::<TSPValue<usize>,TSPInstance<usize>, Vec<usize>, DiscreteHyperparameters>()
    })];

    let mut total_number_repetitions = 0;

    for conf in &configs {
        let common_conf = conf.get_common_config();
        total_number_repetitions = common_conf.number_of_repetitions*conf.number_of_algorithms() as u64;
    }

    println!("Computing {} runs", total_number_repetitions);

    let start_moment = chrono::Local::now();
    let mut i: u32 = 1;

    for (config_index, config) in configs.iter().enumerate() {
        let p_params = config.get_problem_config_parameters();
        println!("Config n°{}:\n{:?}", config_index ,p_params);
        let file = std::fs::File::create(
            Path::new(format!("{}_results_{}.csv", file_prefix, config_index).as_str()));
        let mut writer = file.unwrap();

        let written = writer.write_all("\"".as_bytes()).and(
        writer.write_all(p_params.to_string().as_bytes())).and(
        writer.write_all("\"\n".as_bytes()));

        match written {
            Ok(()) => {
                println!("JSON header written");
            }
            _ => {
                println!("JSON header could not be written");
            }
        }

        let mut csv_writer = csv::Writer::from_writer(writer);
        Iteration::write_header(&mut csv_writer);
        for it in config.execute() {
            let now_moment = chrono::Local::now();
            let duration = now_moment.signed_duration_since(start_moment);
            let speed = duration/i as i32;
            let estimated_reamining_duration = speed*(total_number_repetitions-i as u64) as i32;
            let estimated_end = now_moment+estimated_reamining_duration;
            println!("Run {}/{}", i, total_number_repetitions);
            println!("Estimated end: {}", estimated_end.to_string());

            for iteration in it {
                iteration.write_row(&mut csv_writer);
            }

            i += 1;
        }
    }

    let after_moment = chrono::Local::now();
    let total_duration = after_moment.signed_duration_since(start_moment);
    println!("Finished running tests in {} minute(s)\nExiting...", total_duration.num_minutes().to_string());
}
