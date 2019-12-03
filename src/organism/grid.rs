use std::collections::HashMap;
use super::organism::Organism;
use crate::organism::OrganismGenerator;
use ndarray::{Array, ArrayD};

#[derive(Clone)]
pub struct Grid<V,F> {
    pub cells: ArrayD<HashMap<F,Organism<V>>>
}

