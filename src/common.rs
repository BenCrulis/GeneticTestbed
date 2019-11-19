use std::collections::HashMap;
use ordered_float::OrderedFloat;


#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Parameter {
    Integer(i64),
    Decimal(OrderedFloat<f64>),
    String(String)
}

pub fn decimal_param(val: f64) -> Parameter {
    return Parameter::Decimal(OrderedFloat::from(val));
}

pub fn str_param(str: &str) -> Parameter {
    Parameter::String(String::from(str))
}


pub trait Named {
    fn name(&self) -> String;
}

pub trait Parametrized {
    fn parameters(&self) -> HashMap<String, Parameter> {
        return HashMap::new();
    }
}