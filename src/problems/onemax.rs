use crate::organism::{Metric, OrganismGenerator, Organism};
use crate::common::{Named, Parametrized};
use serde_json::{Value, Map};
use crate::problems::{ProblemInstanceGenerator, DiscreteHyperparameters};
use crate::scoring::Scorer;
use crate::algorithm::mutation::Mutator;
use rand::{thread_rng, Rng};
use crate::features::FeatureMapper;

#[derive(Clone,Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OneMaxValue {
    values: Vec<u16>
}

impl Metric for OneMaxValue {
    fn distance_to(&self, other: &Self) -> f64 {
        self.values.iter()
            .zip(other.values.iter())
            .map(|(x,y)| if x.eq(y) {1} else {0})
            .sum::<u16>() as f64
    }
}

#[derive(Copy, Clone)]
pub struct OneMax {
    pub size: usize
}


impl Named for OneMax {
    fn name(&self) -> String {
        "OneMax".to_string()
    }
}

impl Parametrized for OneMax {
    fn parameters(&self) -> Value {
        let mut hm = Map::new();
        hm.insert("problem size (max score)".to_string(), self.size.into());
        return Value::Object(hm);
    }
}

impl ProblemInstanceGenerator<OneMax> for OneMax {
    fn generate_problem(&self) -> OneMax { *self }
}

#[derive(Copy, Clone)]
pub struct OneMaxScorer {}

impl Scorer<OneMaxValue, OneMax> for OneMaxScorer {
    fn score(&self, genome: &OneMaxValue, problem: &OneMax) -> f64 {
        (genome.values.iter().sum::<u16>() as f64) / (genome.values.len() as f64)
    }
}

#[derive(Copy, Clone)]
pub struct OneMaxMutator {}

impl Mutator<OneMaxValue, DiscreteHyperparameters> for OneMaxMutator {
    fn mutate(&self, genome: &mut OneMaxValue, hyperparameters: &DiscreteHyperparameters) -> bool {

        let mut rng = thread_rng();

        let mut mutated = false;
        while rng.gen::<f64>() < hyperparameters.mutation_chance {
            let i = rng.gen_range(0, genome.values.len());
            genome.values[i] = if genome.values[i] == 0 {1} else {0};
            mutated = true;
        }

        mutated
    }
}

pub struct OneMaxMapper {
    pub number_of_bits: usize
}

impl Named for OneMaxMapper {
    fn name(&self) -> String {
        "One Max bit mapper".to_string()
    }
}

impl Parametrized for OneMaxMapper {
    fn parameters(&self) -> Value {
        let mut hm = Map::new();
        hm.insert("number of bits mapped".to_string(), self.number_of_bits.into());
        return Value::Object(hm);
    }
}

impl FeatureMapper<OneMaxValue, Vec<u16>, OneMax> for OneMaxMapper {
    fn number_of_possible_features(&self, problem: &OneMax) -> usize {
        2_usize.pow(self.number_of_bits as u32)
    }

    fn project(&self, genome: &OneMaxValue) -> Vec<u16> {
        return genome.values[..self.number_of_bits].to_vec();
    }

    fn default_features(&self) -> Vec<u16> {
        return Vec::new();
    }
}

#[derive(Copy, Clone)]
pub struct OneMaxGenerator {}

impl Named for OneMaxGenerator {
    fn name(&self) -> String {
        "Only zeroes generator".to_string()
    }
}

impl Parametrized for OneMaxGenerator {}

impl OrganismGenerator<OneMaxValue, OneMax> for OneMaxGenerator {
    fn generate(&self, problem: &OneMax) -> OneMaxValue {
        OneMaxValue {
            values: vec![0;problem.size]
        }
    }
}



