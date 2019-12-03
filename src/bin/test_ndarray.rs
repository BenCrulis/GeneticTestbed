use ndarray::Array;

fn main() {

    let arr = Array::from_shape_fn((3,3,3), |_| 3);

    let arr2 = Array::from_shape_fn(vec![2;3], |_| 5);

    println!("{}",&arr2)

}
