pub mod rastrigin;
pub mod travelling_salesman;

use crate::common::Named;
use crate::common::Parametrized;

pub trait ProblemInstanceGenerator<P>: Named + Parametrized {
    fn generate_problem(&self) -> P;
}