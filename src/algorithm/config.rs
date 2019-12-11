use std::rc::Rc;
use crate::organism::OrganismGenerator;
use crate::problems::{ProblemInstanceGenerator, Environment};
use crate::features::FeatureMapper;
use crate::scoring::Scorer;
use crate::algorithm::mutation::Mutator;

pub struct ProblemConfig<V,P,H> {
    pub random_organism_generator: Rc<dyn OrganismGenerator<V,P>>,
    pub problem_instance_generator: Rc<dyn ProblemInstanceGenerator<P>>,
    pub constant_hyperparameters: H,
    pub hyperparameter_mapper: Rc<dyn Environment<H>>,
    pub scorer: Rc<dyn Scorer<V,P>>,
    pub mutator: Rc<dyn Mutator<V,H>>
}

impl<V,P,H: Clone> Clone for ProblemConfig<V,P,H> {
    fn clone(&self) -> Self {
        ProblemConfig {
            random_organism_generator: self.random_organism_generator.clone(),
            problem_instance_generator: self.problem_instance_generator.clone(),
            constant_hyperparameters: self.constant_hyperparameters.clone(),
            hyperparameter_mapper: self.hyperparameter_mapper.clone(),
            scorer: self.scorer.clone(),
            mutator: self.mutator.clone()
        }
    }
}