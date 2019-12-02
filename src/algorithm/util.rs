use crate::organism::Organism;
use crate::scoring::Scorer;
use std::cmp::Ordering;

pub fn sorted_scores<V,P>(mut organisms: Vec<Organism<V>>, scorer: &dyn Scorer<V,P>, problem: &P) -> Vec<f64> {

    let mut scores: Vec<f64> = organisms.drain(..).map(|org| org.only_score(scorer, problem)).collect();
    scores.sort_by(|x,y| y.partial_cmp(x).unwrap_or(Ordering::Less));

    scores
}


struct Coordinates {
    coords: Vec<(usize,usize)>
}

impl Coordinates {
    pub fn new(coordinates: Vec<(usize,usize)>) -> Option<Self> {
        for (v,m) in &coordinates {
            if v >= m {
                return None;
            }
        }

        return Some(Coordinates {
            coords: coordinates
        });
    }

    pub fn get_only_coords(&self) -> Vec<usize> {
        return self.coords.iter().map(|x| x.0).collect();
    }

    pub fn get_coords_slice(&self) -> &[(usize,usize)] {
        return self.coords.as_slice();
    }

    pub fn flatten_coords(&self) -> usize {
        let mut i = 0;

        //TODO must finish
        for (v,max) in &self.coords {

        }


        return i;
    }
}







