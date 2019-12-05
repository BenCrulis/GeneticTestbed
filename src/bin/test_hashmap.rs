use std::collections::HashMap;
use ndarray::{Array, IxDynImpl};
use ndarray::Dim;

#[derive(Clone,PartialEq,Debug)]
struct Test {
    score: f64,
    name: String,
    genotype: Vec<usize>
}

fn main() {





    let mut arr = Array::from_shape_fn(vec![2], |x: Dim<IxDynImpl>| {
        let mut hm: HashMap<Vec<usize>, Test> = HashMap::new();
        hm.insert(vec![0], Test {
            score: 1.0,
            name: "test 1".to_string(),
            genotype: vec![3]
        });
        return hm;
    });

    println!("arr: {:?}", &arr);

    let arr_ref = &mut arr;

    let new_org = {
        let mut v = arr_ref.view_mut();
        let hm: &mut HashMap<Vec<usize>, Test> = v.get_mut(0).unwrap();
        let org = hm.get_mut(&vec![0]).unwrap();
        let new_org = org.clone();
        org.name = "test 2".to_string();
        println!("hm={:?}", &hm);
        new_org
    };

    println!("arr: {:?}", arr_ref);
    println!("new_org: {:?}", &new_org);

    {
        let mut v = arr_ref.view_mut();
        let hm: &mut HashMap<Vec<usize>, Test> = v.get_mut(0).unwrap();
        hm.insert(new_org.genotype.to_vec(), new_org);
    }

    println!("arr: {:?}", arr_ref);

}