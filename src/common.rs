use std::collections::HashMap;
use ordered_float::OrderedFloat;


#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub enum Parameter {
    Integer(i64),
    Decimal(OrderedFloat<f64>),
    String(String)
}

pub type ParameterConfig = HashMap<String,Parameter>;

pub fn decimal_param(val: f64) -> Parameter {
    return Parameter::Decimal(OrderedFloat::from(val));
}

pub fn str_param(str: &str) -> Parameter {
    Parameter::String(String::from(str))
}

pub fn update_parameters(param1: ParameterConfig, param2: ParameterConfig) -> ParameterConfig {
    let mut hm = param1.clone();
    hm.extend(param2);
    return hm;
}

pub trait Named {
    fn name(&self) -> String;
}

pub trait Parametrized {
    fn parameters(&self) -> HashMap<String, Parameter> {
        return HashMap::new();
    }
}