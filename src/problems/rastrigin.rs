use std::f64::consts::PI;

pub const A: f64 = 10.0;


pub fn rastrigin(x: &Vec<f64>) -> f64 {
    let n: f64 = x.len() as f64;

    let mut sum = 0.0;

    for v in x {
        sum += v * v - A * (2.0 * PI * *v).cos();
    }

    return A * n + sum;
}

pub fn custom_rastrigin(x: &Vec<f64>) -> f64 {
    return (100.0 - rastrigin(x)).max(0.0);
}


pub const B: f64 = 10.0;


pub fn regularized_rastrigin(x: &Vec<f64>) -> f64 {
    let n: f64 = x.len() as f64;

    let mut sum = 0.0;

    let mut reg_sum = 0.0;

    for v in x {
        sum += v * v - (2.0 * PI * *v + PI).cos();
        reg_sum += -v*v;
    }

    let reg = (reg_sum/B).exp();

    return sum*reg/n;
}