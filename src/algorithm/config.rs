use std::rc::Rc;
use crate::organism::OrganismGenerator;
use crate::problems::{ProblemInstanceGenerator, Environment};
use crate::features::FeatureMapper;
use crate::scoring::Scorer;
use crate::algorithm::mutation::Mutator;

#[derive(Clone)]
pub struct ProblemConfig<V,P,F,H> {
    pub random_organism_generator: Rc<dyn OrganismGenerator<V,P>>,
    pub problem_instance_generator: Rc<dyn ProblemInstanceGenerator<P>>,
    pub feature_mapper: Rc<dyn FeatureMapper<V, F, P>>,
    pub constant_hyperparameters: H,
    pub hyperparameter_mapper: Rc<dyn Environment<H>>,
    pub scorer: Rc<dyn Scorer<V,P>>,
    pub mutator: Rc<dyn Mutator<V,H>>
}