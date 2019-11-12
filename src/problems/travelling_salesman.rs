use std::vec::Vec;
use std::collections::HashMap;

extern crate ordered_float;
use ordered_float::OrderedFloat;
use std::hash::Hash;

pub struct TSPValue<T> {
    permutation: Vec<T>
}



pub fn score_tsp<T: Eq + Hash>(distances: &HashMap<(&T,&T), f64>) -> impl Fn(&TSPValue<T>) -> f64 {
    let max: f64 = distances.values().map(|x| OrderedFloat::from(*x)).max().unwrap().into();

    return |tsp_value| {
        let sum = 0.0;
        for x in tsp_value.permutation.iter().zip(
                        tsp_value.permutation.iter().skip(1).cycle()) {
            sum += *distances.get(&x).unwrap();
        }

        return max*tsp_value.permutation.len() as f64-sum;
    };
}




