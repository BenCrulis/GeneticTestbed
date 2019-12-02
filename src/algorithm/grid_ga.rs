use crate::common::{Named, Parametrized};
use crate::organism::grid::Grid;
use std::rc::Rc;
use crate::algorithm::config::ProblemConfig;
use crate::algorithm::selection::Elitism;
use serde_json::{Map, Value};
use crate::algorithm::algorithm::{ReplacementSelection, UpdatableSolver};
use std::hash::Hash;
use crate::organism::Organism;
use crate::problems::Hyperparameter;
use std::collections::HashMap;
use rand::thread_rng;

#[derive(Copy, Clone)]
struct GeneralizedMAPElite {
    use_features: bool,
    use_hyperparameter_mapping: bool,
    number_of_spatial_dimensions: usize
}

impl Named for GeneralizedMAPElite {
    fn name(&self) -> String {
        String::from("Generalized MAP Elite algorithm")
    }
}

impl Parametrized for GeneralizedMAPElite {
    fn parameters(&self) -> serde_json::Value {
        let mut config = Map::new();
        config.insert("use spatial grid".to_string(), self.number_of_spatial_dimensions.into());
        config.insert("use spatial hyperparameters".to_string(), self.use_hyperparameter_mapping.into());
        config.insert("use features".to_string(), self.use_features.into());

        return Value::Object(config);
    }
}

struct GeneralizedMAPEliteExec<V,P,F,H> {
    algo_config: GeneralizedMAPElite,
    problem: Rc<P>,
    organisms: Grid<V,F>,
    problem_config: Rc<ProblemConfig<V,P,F,H>>,
    elitism: Rc<dyn Elitism>
}

impl<V: Clone + 'static,P: 'static,F: Hash + Clone + Eq + 'static,H: Hyperparameter + 'static> ReplacementSelection<V,P,F,H> for GeneralizedMAPElite {
    fn initialize_solver(&self, pop_size: usize, problem: Rc<P>, elitism: Rc<dyn Elitism>, problem_config: Rc<ProblemConfig<V, P, F, H>>) -> Box<dyn UpdatableSolver<V>> {

        let possibles_features = if self.use_features {
            problem_config.feature_mapper.number_of_possible_features(problem.as_ref())
        }
        else {
            1
        };

        let pop_per_cell = (pop_size/possibles_features);

        let dim_size = (pop_per_cell as f64).powf(1.0/problem_config.hyperparameter_mapper.number_of_hyperparameters() as f64) as usize;

        println!("dim size: {}", dim_size);

        let number_of_cells = dim_size*problem_config.hyperparameter_mapper.number_of_hyperparameters() as usize;

        let mut organisms = Vec::with_capacity(number_of_cells);

        for i in 0..number_of_cells {
            let org = problem_config.random_organism_generator.generate_organism(problem.as_ref());
            let mut hm = HashMap::new();

            let features = if self.use_features {
                problem_config.feature_mapper.project(&org.genotype)
            }
            else {
                problem_config.feature_mapper.default_features()
            };

            hm.insert(features, org);
            organisms.push(hm);
        }

        return Box::new(GeneralizedMAPEliteExec {
            algo_config: *self,
            problem: problem.clone(),
            organisms: Grid {cells : organisms},
            problem_config: problem_config.clone(),
            elitism
        });
    }
}


impl<V,P,F,H> UpdatableSolver<V> for GeneralizedMAPEliteExec<V,P,F,H> {
    fn update(&mut self) -> Vec<Organism<V>> {
        let mut rng = thread_rng();


        unimplemented!()
    }
}
