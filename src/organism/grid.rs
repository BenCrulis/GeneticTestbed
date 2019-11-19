use std::collections::HashMap;
use super::organism::Organism;

#[derive(Clone)]
pub struct Grid<V,F> {
    pub cells: Vec<HashMap<F,Organism<V>>>
}