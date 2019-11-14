




#[test]
pub fn test_rastrigin() {
    println!("rastrigin(1.0,1.0) = {}", rastrigin(&vec![1.,1.]));
    println!("custom_rastrigin(1.0,1.0) = {}", custom_rastrigin(&vec![1.,1.]));
    println!("regularized_rastrigin(0.0,0.0) = {}", regularized_rastrigin(&vec![0.0,0.0]))

}