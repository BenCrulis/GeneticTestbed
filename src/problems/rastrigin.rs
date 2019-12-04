extern crate rand_distr;
extern crate num;
use std::f64::consts::PI;
use crate::problems::{ProblemInstanceGenerator, ContinuousHyperparameters};
use crate::common::{Named, Parametrized};
use serde_json::{Value, Map};
use crate::scoring::Scorer;
use crate::algorithm::mutation::Mutator;
use rand::{thread_rng, Rng};
use rand_distr::Normal;
use crate::features::FeatureMapper;
use num::pow;

pub fn rastrigin(a: f64, x: &[f64]) -> f64 {
    let n: f64 = x.len() as f64;

    let mut sum = 0.0;

    for v in x {
        sum += v * v - a * (2.0 * PI * *v).cos();
    }

    return a * n + sum;
}

pub fn custom_rastrigin(a: f64, x: &[f64]) -> f64 {
    return (100.0 - rastrigin(a, x)).max(0.0);
}

pub fn regularized_rastrigin(b: f64, x: &[f64]) -> f64 {
    let n: f64 = x.len() as f64;

    let mut sum = 0.0;

    let mut reg_sum = 0.0;

    for v in x {
        sum += v * v - (2.0 * PI * *v + PI).cos();
        reg_sum += -v*v;
    }

    let reg = (reg_sum/b).exp();

    return sum*reg/n;
}

#[derive(Clone)]
struct RastriginValue {
    value: Vec<f64>
}

#[derive(Copy, Clone)]
struct Rastrigin {
    a: f64,
    b: f64,
    max_abs_val: f64,
    nb_dimensions: usize
}

impl Named for Rastrigin {
    fn name(&self) -> String {
        "Custom Rastrigin".to_string()
    }
}

impl Parametrized for Rastrigin {
    fn parameters(&self) -> Value {
        let mut hm = Map::new();
        hm.insert("a".to_string(), self.a.into());
        hm.insert("b".to_string(), self.b.into());
        hm.insert("absolute boundary value".to_string(), self.max_abs_val.into());
        hm.insert("number of dimensions".to_string(), self.nb_dimensions.into());
        return Value::Object(hm);
    }
}

impl ProblemInstanceGenerator<Rastrigin> for Rastrigin {
    fn generate_problem(&self) -> Rastrigin {
        *self
    }
}

#[derive(Copy, Clone)]
struct RegRastriginScorer {}

impl Scorer<RastriginValue, Rastrigin> for RegRastriginScorer {
    fn score(&self, genome: &RastriginValue, problem: &Rastrigin) -> f64 {
        regularized_rastrigin(problem.b, genome.value.as_slice())
    }
}

#[derive(Copy, Clone)]
struct RastriginMutator {}

impl Mutator<RastriginValue, ContinuousHyperparameters> for RastriginMutator {
    fn mutate(&self, genome: &mut RastriginValue, hyperparameters: &ContinuousHyperparameters) {
        let mut rng = thread_rng();

        while rng.gen::<f64>() < hyperparameters.mutation_chance {
            let i = rng.gen_range(0, genome.value.len());
            let normal = Normal::new(genome.value[i], hyperparameters.mutation_size).unwrap();
            genome.value[i] = rng.sample(normal)
        }
    }
}

#[derive(Copy, Clone)]
struct RastriginMapper {
    resolution: usize,
    number_of_dimensions: usize,
    max_abs_val: f64
}

#[derive(Clone,Eq,Hash,Ord, PartialOrd, PartialEq)]
struct RastriginFeature {
    bin_coords: Vec<isize>
}

impl Named for RastriginMapper {
    fn name(&self) -> String {
        "Rastrigin binner".to_string()
    }
}

impl Parametrized for RastriginMapper {

}

impl FeatureMapper<RastriginValue, RastriginFeature, Rastrigin> for RastriginMapper {
    fn number_of_possible_features(&self, problem: &Rastrigin) -> usize {
        return pow(self.resolution, self.number_of_dimensions);
    }

    fn project(&self, genome: &RastriginValue) -> RastriginFeature {
        let mut features = Vec::with_capacity(self.number_of_dimensions);
        for x in &genome.value[..self.number_of_dimensions] {
            let x_norm = x/self.max_abs_val;
            let int_x = (x_norm as f64 *self.resolution as f64 *0.5) as isize;
            features.push(int_x);
        }

        return RastriginFeature {
            bin_coords: features
        }
    }

    fn default_features(&self) -> RastriginFeature {
        RastriginFeature { bin_coords: vec![] }
    }
}

#[derive(Copy, Clone)]
struct RastriginGenerator {}