#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

extern crate rand;
extern crate csv;
extern crate serde;
extern crate serde_json;
extern crate statistical;
extern crate chrono;
extern crate itertools;

use std::vec::Vec;
use std::env;
use std::time::{Instant, SystemTime, Duration};

use serde_json::{json, Map, Value};

mod common;
mod problems;
mod organism;
mod algorithm;
mod features;
mod scoring;

use std::collections::{HashMap, HashSet};
use std::process::Output;
use std::collections::hash_map::RandomState;
use std::iter::{Cycle};
use std::rc::Rc;
use itertools::Itertools;

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
use algorithm::simple_adaptive;
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
use crate::problems::{Hyperparameter, ContinuousSpatialMapper};
use crate::problems::rastrigin::{RastriginValue, RastriginFeature, RastriginGenerator, Rastrigin, RastriginMapper, RastriginMutator, RegRastriginScorer};
use crate::organism::Metric;
use crate::problems::onemax::{OneMaxValue, OneMax, OneMaxGenerator, OneMaxMapper, OneMaxScorer, OneMaxMutator};

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
    generations: f64,
    mean_genetic_distance: Option<f64>
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
            "variance",
            "generations",
            "mean genetic distance"
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
            self.pop_score_variance.to_string(),
            self.generations.to_string(),
            self.mean_genetic_distance.map_or("".to_string(), |x| x.to_string())
        ])
    }
}

trait Config {
    fn get_problem_config_parameters(&self) -> serde_json::Value;
    fn get_common_config(&self) -> CommonParameters;
    fn get_problem_name(&self) -> String;
    fn number_of_algorithms(&self) -> usize;
    fn execute(&self) -> Box<dyn Iterator<Item=Box<dyn Iterator<Item=Iteration>>>>;
}


trait AlgorithmExec<V,P,F,H> {
    fn exec(&self, config: Rc<MyConfigIt<V,P,H>>) -> Box<dyn Iterator<Item=Iteration>>;
}



#[derive(Clone)]
struct AlgoConfig<V,P,H> {
    elitism: Rc<dyn Elitism>,
    replacement_selection: Rc<dyn ReplacementSelection<V,P,H>>
}


#[derive(Copy, Clone)]
struct CommonParameters {
    population_size: usize,
    number_of_repetitions: u64,
    number_of_iterations: u64,
    genome_stats_gap: u64
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






struct MyConfig<V,P,H> {
    problem_config: Rc<ProblemConfig<V,P,H>>,
    common_config: Rc<CommonParameters>,
    algorithms: Vec<Rc<AlgoConfig<V,P,H>>>,
}
impl<V,P,H> Clone for MyConfig<V,P,H> {
    fn clone(&self) -> Self {
        MyConfig {
            problem_config: self.problem_config.clone(),
            common_config: self.common_config.clone(),
            algorithms: self.algorithms.clone()
        }
    }
}


struct MyConfigIt<V,P,H> {
    my_config: Rc<MyConfig<V,P,H>>,
    instance: Rc<P>,
    repetitions: u64,
    index_algo: usize
}

impl<V,P,H> Clone for MyConfigIt<V,P,H> {
    fn clone(&self) -> Self {
        MyConfigIt {
            my_config: self.my_config.clone(),
            instance: self.instance.clone(),
            repetitions: self.repetitions,
            index_algo: self.index_algo
        }
    }
}


impl<V: 'static + Metric,P: 'static,H: 'static> Iterator for MyConfigIt<V,P,H> {
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

impl<V: 'static + Metric,P: 'static,H: 'static> Config for MyConfig<V,P,H> {
    fn get_problem_config_parameters(&self) -> serde_json::Value {
        println!("Getting MyConfig parameters");
        let common_params = self.common_config.parameters();
        let mut algo_configs = Vec::new();

        for (i,algo) in self.algorithms.iter().enumerate() {
            let mut algo_config = Map::new();
            algo_config.insert("algorithm name".to_string(), algo.replacement_selection.name().into());
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

    fn get_problem_name(&self) -> String {
        self.problem_config.problem_instance_generator.name()
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
    problem_config: Rc<ProblemConfig<V,P,H>>,
    common_config: Rc<CommonParameters>,
    algorithms: Vec<Rc<dyn AlgorithmExec<V,P,F,H>>>,
    instance: Rc<P>,
    repetitions: u64
}


struct AlgorithmState<V,P,H> {
    my_config_it: Rc<MyConfigIt<V,P,H>>,
    updatable_solver: Box<dyn UpdatableSolver<V>>,
    i: u64
}

impl<V: Metric,P,H> Iterator for AlgorithmState<V,P,H> {
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

            let mut mean_genetic_distance: Option<f64> = None;
            if self.i % self.my_config_it.my_config.common_config.genome_stats_gap == 0 {

                let mut distances: Vec<f64> = Vec::with_capacity(number_of_organisms * (number_of_organisms - 1) / 2);
                for i in 0..number_of_organisms {
                    for j in (i + 1)..number_of_organisms {
                        distances.push(organisms[i].distance_to(&organisms[j]));
                    }
                }

                if number_of_organisms > 0 {
                    let me = mean(&distances);
                    if me.is_finite() {
                        mean_genetic_distance = Some(me);
                    }
                }
            }

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
            let vari = if sorted_score.len() > 1 {
                variance(sorted_score.as_slice(), Some(mean_val)) } else { 0.0 };
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
                pop_score_variance: vari,
                generations: self.i as f64 / number_of_organisms as f64,
                mean_genetic_distance
            };

            self.i += 1;
            Some(iter)
        }
    }
}



fn simple_ga<V: Clone + 'static,P: 'static,H: Copy + 'static>(elitism: Rc<dyn Elitism>) -> Rc<AlgoConfig<V,P,H>> {
    return Rc::new(AlgoConfig {
        elitism,
        replacement_selection: Rc::new(SimpleReplacement{})
    })
}

fn grid_ga<V: 'static + Clone + PartialEq,
    P: 'static,
    F: 'static + Eq + Clone + Hash,
    H: 'static + Hyperparameter + Clone>(
        elitism: Rc<dyn Elitism>,
        feature_mapper: Option<Rc<dyn FeatureMapper<V,F,P>>>,
        use_hyperparameter_mapping: bool,
        number_of_spatial_dimensions: usize,
        default_features: F) -> Rc<AlgoConfig<V,P,H>> {
    return Rc::new(AlgoConfig {
        elitism,
        replacement_selection: Rc::new(grid_ga::GeneralizedMAPElite {
            feature_mapper,
            use_hyperparameter_mapping,
            number_of_spatial_dimensions,
            default_feature: default_features
        })
    });
}


fn all_algos_configs<V: 'static + Clone + PartialEq,
    P: 'static,
    F: 'static + Eq + Clone + Hash ,
    H: 'static + Hyperparameter + Copy + Clone>(feature_mapper: Rc<dyn FeatureMapper<V,F,P>>,
            full_feature_mapper: Rc<dyn FeatureMapper<V,F,P>>) -> Vec<Rc<AlgoConfig<V,P,H>>> {
    let feat = full_feature_mapper.default_features();
    let greedy = Rc::new(GreedySelection{});
    let mh = Rc::new(MetropolisHastings{});
    vec![simple_ga(greedy.clone()),
         grid_ga(greedy.clone(), None, false, 1, feat.clone()),
         grid_ga(greedy.clone(), Some(feature_mapper.clone()), false, 1, feat.clone()),
         grid_ga(greedy.clone(), None, true, 1, feat.clone()),
         grid_ga(greedy.clone(), Some(feature_mapper.clone()), true, 1, feat.clone()),
         Rc::new(AlgoConfig {
            elitism: greedy.clone(),
            replacement_selection: Rc::new(map_elite::MAPElite{
                feature_mapper: full_feature_mapper.clone()
            })
        }),
        // Metropolis Hastings variant
         simple_ga(mh.clone()),
         grid_ga(mh.clone(), None, false, 1, feat.clone()),
         grid_ga(mh.clone(), Some(feature_mapper.clone()), false, 1, feat.clone()),
         grid_ga(mh.clone(), None, true, 1, feat.clone()),
         grid_ga(mh.clone(), Some(feature_mapper.clone()), true, 1, feat.clone()),
         Rc::new(AlgoConfig {
             elitism: mh.clone(),
             replacement_selection: Rc::new(map_elite::MAPElite{
                 feature_mapper: full_feature_mapper.clone()
             })
         })
    ]
}

fn all_algo_config_with_adaptive<V: 'static + Clone + PartialEq,
    P: 'static ,
    F: 'static + Eq + Clone + Hash>(feature_mapper: Rc<dyn FeatureMapper<V,F,P>>,
            full_feature_mapper: Rc<dyn FeatureMapper<V,F,P>>) -> Vec<Rc<AlgoConfig<V,P,DiscreteHyperparameters>>> {
    let mut v = all_algos_configs(feature_mapper, full_feature_mapper);
    v.push(Rc::new(AlgoConfig {
        elitism: Rc::new(GreedySelection{}),
        replacement_selection: Rc::new(simple_adaptive::SimpleAdaptive{
            prior_a: 1,
            prior_b: 1
        })
    }));
    v.push(Rc::new(AlgoConfig {
        elitism: Rc::new(MetropolisHastings{}),
        replacement_selection: Rc::new(simple_adaptive::SimpleAdaptive{
            prior_a: 1,
            prior_b: 1
        })
    }));
    return v;
}

fn tsp_problem_config() -> Rc<ProblemConfig<TSPValue<usize>,TSPInstance<usize>,DiscreteHyperparameters>> {
    Rc::new(ProblemConfig {
        random_organism_generator: Rc::new(TSPRandomSolution{}),
        problem_instance_generator: Rc::new(SimpleTSPInstanceGenerator{ number_of_cities: 50,
            number_of_dimensions: 2}),
        constant_hyperparameters: DiscreteHyperparameters{ mutation_chance: 0.5 },
        hyperparameter_mapper: Rc::new(SpatialMapper{ number_of_additional_dimensions: 0 }),
        mutator: Rc::new(TSPMutator{}),
        scorer: Rc::new(TSPScorer{})
    })
}

fn rastrigin_problem_config() -> Rc<ProblemConfig<RastriginValue, Rastrigin, ContinuousHyperparameters>> {
    let mut_size = 0.5;

    Rc::new(ProblemConfig {
        random_organism_generator: Rc::new(RastriginGenerator{}),
        problem_instance_generator: Rc::new(Rastrigin{
            a: 10.0,
            b: 1000.0,
            max_abs_val: 5.0,
            nb_dimensions: 10
        }),
        constant_hyperparameters: ContinuousHyperparameters { mutation_chance: 0.5, mutation_size: mut_size },
        hyperparameter_mapper: Rc::new(ContinuousSpatialMapper{ mean_mutation_size: mut_size }),
        scorer: Rc::new(RegRastriginScorer{}),
        mutator: Rc::new(RastriginMutator{})
    })
}

fn onemax_config() -> Rc<ProblemConfig<OneMaxValue, OneMax, DiscreteHyperparameters>> {
    Rc::new(ProblemConfig {
        random_organism_generator: Rc::new(OneMaxGenerator{}),
        problem_instance_generator: Rc::new(OneMax { size: 100 }),
        constant_hyperparameters: DiscreteHyperparameters{ mutation_chance: 0.5 },
        hyperparameter_mapper: Rc::new(SpatialMapper{ number_of_additional_dimensions: 0 }),
        scorer: Rc::new(OneMaxScorer{}),
        mutator: Rc::new(OneMaxMutator{})
    })
}

fn main() {
    let file_prefix = "final";

    let common_config = CommonParameters {
        population_size: 2500,
        number_of_repetitions: 30,
        number_of_iterations: 100000,
        genome_stats_gap: 50
    };

    let mut configs: Vec<Rc<dyn Config>> = Vec::new();

    let mut arg_set: HashSet<String> = env::args().map(|s| s.to_lowercase()).collect();

    if arg_set.is_empty() {
        arg_set.insert("tsp".to_string());
    }

    for arg in &arg_set {
        if arg == "tsp" {
            configs.push(Rc::new(MyConfig {
                problem_config: tsp_problem_config(),
                common_config: Rc::new(common_config),
                algorithms: all_algo_config_with_adaptive::<TSPValue<usize>,TSPInstance<usize>, Vec<usize>>(
                    Rc::new(TSPFeatureMapper{ number_cities_mapped: 1 }),
                    Rc::new(TSPFeatureMapper{ number_cities_mapped: 2 })
                )
            }));
        }
        else if arg == "rastrigin" {
            configs.push(Rc::new(MyConfig {
                problem_config: rastrigin_problem_config(),
                common_config: Rc::new(common_config),
                algorithms: all_algos_configs(Rc::new(RastriginMapper{
                    resolution: 10,
                    number_of_dimensions: 1,
                    max_abs_val: 5.0
                }),
                  Rc::new(RastriginMapper{
                      resolution: 7,
                      number_of_dimensions: 4,
                      max_abs_val: 5.0
                  }))
            }));
        }
        else if arg == "onemax" {
            configs.push(Rc::new(MyConfig {
                problem_config: onemax_config(),
                common_config: Rc::new(common_config),
                algorithms: all_algos_configs(Rc::new(OneMaxMapper{ number_of_octets: 1 }),
                                                          Rc::new(OneMaxMapper{ number_of_octets: 1 }))
            }));
        }
        else {
            println!("Unknown config :\"{}\"", arg);
        }
    }

    println!("Configs:");
    for (i,cfg) in configs.iter().enumerate() {
        println!("{}: {}", i, cfg.get_problem_name());
    }
    println!();

    let mut total_number_repetitions = 0;

    for conf in &configs {
        let common_conf = conf.get_common_config();
        total_number_repetitions += common_conf.number_of_repetitions*conf.number_of_algorithms() as u64;
    }

    println!("Computing {} runs", total_number_repetitions);

    let start_moment = chrono::Local::now();
    let mut i: u32 = 1;

    for (config_index, config) in configs.iter().enumerate() {
        let p_params = config.get_problem_config_parameters();
        println!("Config n°{}:\n{:?}", config_index ,p_params);

        let problem_name = config.get_problem_name().replace("/","_");

        let file = std::fs::File::create(
            Path::new(format!("{}_{}_results.csv", file_prefix, problem_name).as_str()));
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
