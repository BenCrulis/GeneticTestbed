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
use rand::{thread_rng, Rng};
use ndarray::{Array, ArrayView, ViewRepr, ArrayViewMut, IxDynImpl, Dim, ArrayD, ArrayViewD};
use rand::seq::SliceRandom;
use std::iter::Zip;

#[derive(Copy, Clone)]
pub struct GeneralizedMAPElite {
    pub use_features: bool,
    pub use_hyperparameter_mapping: bool,
    pub number_of_spatial_dimensions: usize
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

pub struct GeneralizedMAPEliteExec<V,P,F,H> {
    algo_config: GeneralizedMAPElite,
    problem: Rc<P>,
    organisms: Grid<V,F>,
    problem_config: Rc<ProblemConfig<V,P,F,H>>,
    elitism: Rc<dyn Elitism>
}

impl<V: Clone + 'static + PartialEq,P: 'static,F: Hash + Clone + Eq + 'static,H: Hyperparameter + 'static + Clone> ReplacementSelection<V,P,F,H> for GeneralizedMAPElite {
    fn initialize_solver(&self, pop_size: usize, problem: Rc<P>, elitism: Rc<dyn Elitism>, problem_config: Rc<ProblemConfig<V, P, F, H>>) -> Box<dyn UpdatableSolver<V>> {

        let possibles_features = if self.use_features {
            problem_config.feature_mapper.number_of_possible_features(problem.as_ref())
        }
        else {
            1
        };

        //println!("number of possibles features: {}", possibles_features);

        assert!(pop_size > possibles_features); // required for fair comparison by maintaining same pop size between algos

        let pop_per_cell = pop_size/possibles_features;


        let num_dims = problem_config.hyperparameter_mapper.number_of_hyperparameters().max(self.number_of_spatial_dimensions);


        let dim_size = (pop_per_cell as f64).powf(1.0/num_dims as f64) as usize;
        //let dim_size = (pop_size as f64).powf(1.0/num_dims as f64) as usize;


        let organisms: ArrayD<HashMap<F,Organism<V>>> = Array::from_shape_fn(vec![dim_size; num_dims], |_|{
            let org = problem_config.random_organism_generator.generate_organism(problem.as_ref());
            let mut hm = HashMap::new();

            let features = if self.use_features {
                problem_config.feature_mapper.project(&org.genotype)
            }
            else {
                problem_config.feature_mapper.default_features()
            };

            hm.insert(features, org);
            return hm;
        });

        return Box::new(GeneralizedMAPEliteExec {
            algo_config: *self,
            problem: problem.clone(),
            organisms: Grid {cells : organisms},
            problem_config: problem_config.clone(),
            elitism
        });
    }
}


impl<V: Clone + PartialEq,P,F: Clone + Hash + Eq,H: Hyperparameter + Clone> UpdatableSolver<V> for GeneralizedMAPEliteExec<V,P,F,H> {
    fn update(&mut self) -> Vec<Organism<V>> {
        let mut rng = thread_rng();


        let shp = self.organisms.cells.view().shape().to_vec();

        let mut id_a: Vec<usize> = Vec::with_capacity(shp.len());

        for &d in &shp {
            let val_a = rng.gen_range(0,d);
            id_a.push(val_a);
        }

        let mut id_b = id_a.clone();
        let i = rng.gen_range(0,id_a.len());
        let d_max = (shp[i] as i64 -1).max(0) as usize;

        let mut val_b = id_b[i] as i64;

        val_b += if i == 0 {
            1
        } else if i == d_max {
            -1
        } else if rng.gen_bool(0.5) {
            1
        } else {
            -1
        };

        val_b = val_b.max(0).min(d_max as i64);

        id_b[i] = val_b as usize;

        let mut org_a: Organism<V> = {
            let v: ArrayViewD<HashMap<F,Organism<V>>> = self.organisms.cells.view();
            let hm_a: &HashMap<F,Organism<V>> = v.get(id_a.as_slice()).unwrap();
            let vec: Vec<(&F, &Organism<V>)> = hm_a.iter().collect();
            let &(_, org) = vec.choose(&mut rng).unwrap();
            org.clone()
        };

        let old = org_a.clone();

        let old_feature = if self.algo_config.use_features {
            self.problem_config.feature_mapper.project(&org_a.genotype)
        }
        else {
            self.problem_config.feature_mapper.default_features()
        };

        let hyper = if self.algo_config.use_hyperparameter_mapping {
            let coord: Vec<(usize,usize)> = id_a.iter().zip(shp.iter()).map(|(&x,&y)| (x,y)).collect();
            self.problem_config.hyperparameter_mapper.map_hyperparameters(&coord)
        }
        else {
            self.problem_config.constant_hyperparameters.clone()
        };

        org_a.mutate(self.problem_config.mutator.as_ref(), &hyper);

        let feature_a = if self.algo_config.use_features {
            self.problem_config.feature_mapper.project(&org_a.genotype)
        }
        else {
            self.problem_config.feature_mapper.default_features()
        };

        let score_a = org_a.score_with_cache(self.problem_config.scorer.as_ref(), self.problem.as_ref());

        let mut score_b = score_a;
        let mut replace = {
            // ArrayViewMut<ViewRepr<&mut ?>, IxDyn<IxDynImpl>>
            let mut v: ArrayViewMut<HashMap<F, Organism<V>>, Dim<IxDynImpl>> = self.organisms.cells.view_mut();
            let hm_b: &mut HashMap<F,Organism<V>> = v.get_mut(id_b.as_slice()).unwrap();
            let op_org_b = hm_b.get_mut(&feature_a);

            match op_org_b {
                Some(org_b) => {
                    score_b = org_b.score_with_cache(self.problem_config.scorer.as_ref(), self.problem.as_ref());
                    assert!(score_b.is_finite());
                    self.elitism.choose(score_a, score_b)
                },
                None => true
            }
        };

        replace = replace && (old_feature != feature_a || id_a != id_b );

        if replace {
            let mut v: ArrayViewMut<HashMap<F,Organism<V>>, Dim<IxDynImpl>> = self.organisms.cells.view_mut();
            let feat_map: &mut HashMap<F, Organism<V>> = v.get_mut(id_b.as_slice()).unwrap();
            feat_map.insert(feature_a, org_a);
        }


        self.organisms.cells.view().iter().flat_map(|hm: &HashMap<F, Organism<V>>| {
            hm.values().cloned().collect::<Vec<Organism<V>>>()
        }).collect()
    }
}
