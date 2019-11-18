extern crate rand;

use std::vec::Vec;
use std::time::Instant;

mod common;
mod problems;
mod evaluation;
mod organism;
mod selection;

use common::Named;

use problems::rastrigin::{rastrigin,custom_rastrigin,regularized_rastrigin};
use evaluation::scoring::Scoring;
use organism::organism::Organism;
use std::collections::HashMap;
use std::process::Output;
use std::collections::hash_map::RandomState;
use std::iter::Cycle;

use selection::Elitism;
use selection::MetropolisHastings;
use selection::GreedySelection;

trait Population {

}

struct Grid<V,F> {
    cells: Vec<HashMap<F,Organism<V>>>
}

trait HyperparameterMapper<H>: Named {
    fn map_hyperparameters(&self, coordinates: &Vec<(usize, usize)>) -> H;
}


trait OrganismGenerator<V>: Named {
    fn generate(&self) -> V;
}

trait ProblemInstanceGenerator<P>: Named {
    fn generate_problem(&self) -> P;
}

struct Placeholder;

trait ReplacementSelection: Named {
    fn select_replace<V,F>(&self, grid: &mut Grid<V,F>);
}

struct AlgoConfig<'a> {
    elitism: &'a dyn Elitism,
}

struct ProblemConfig<'a,'b,V,H,P> {
    number_of_repetitions: u64,
    number_of_iteration: u64,
    random_organism_generator: &'b dyn OrganismGenerator<V>,
    problem_instance_generator: &'b dyn ProblemInstanceGenerator<P>,
    scorer_generator: &'b dyn Scoring<Genotype=&'a V>,
    mutator: &'b dyn Fn(&V) -> (V,bool),
    constant_hyperparameters: H,
    hyperparameter_mapper: &'b dyn HyperparameterMapper<H>
}


struct Iteration {
    iteration: i64,
    repetition: i64,
    algo_config: HashMap<String,String>,
    timestamp: Instant,
    best_score: f64,
    pop_score_variance: f64,
    quality_diversity: f64
}

trait Config {
    fn get_problem_config_parameters(&self) -> HashMap<String,String>;
    fn execute(&mut self) -> Box<dyn Iterator<Item=Iteration>>;
}


struct AlgoState {

}


/*
fn new<V,H,T>(number_of_repetitions: u64,
           number_of_iteration: u64,
           elitism: Box<dyn Elitism>,
           problem_config: Box<ProblemConfig<V,H,T>>) -> Self;
*/

fn main() {
    println!("Hello, world!");

    let configs: Vec<&mut dyn Config> = vec![];

    for mut config in configs {
        let p_params = config.get_problem_config_parameters();
        for it in config.execute() {

        }
    }
}
