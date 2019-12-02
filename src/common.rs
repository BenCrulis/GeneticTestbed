use std::collections::HashMap;
use ordered_float::OrderedFloat;
use serde_json::{Value, Number, Map};

pub fn decimal_param(val: f64) -> Value {
    return Value::Number(serde_json::Number::from_f64(val).unwrap());
}

pub fn str_param(str: &str) -> Value {
    Value::String(str.to_string())
}

pub fn int_param(int: i64) -> Value {
    return Value::Number(int.into())
}

pub trait Named {
    fn name(&self) -> String;
}

pub trait Parametrized {
    fn parameters(&self) -> Value {
        return Value::Object(Map::new());
    }
}