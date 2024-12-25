mod kahan;
mod logspace;

fn direct_summation(x: &[f64]) -> f64 {
    let mut sum: f64 = 0.0;
    for &xi in x.iter() {
        sum = sum + xi;
    }
    sum
}

fn main() {
    let num: usize = 4;
    let n: Vec<f64> = logspace::logspace(2.0, 3.0, num);
    println!("{:?}", n);

    let num: usize = 1000;
    let n: Vec<f64> = vec![0.1; num];
    let true_sum: f64 = num as f64 * 0.1;
    let kahan_error: f64 = (kahan::kahan_summation(&n) - true_sum).abs();
    let direct_error: f64 = (direct_summation(&n) - true_sum).abs();
    let rust_error: f64 = (n.iter().sum::<f64>() - true_sum).abs();
    println!("Kahan error: {:.64}", kahan_error);
    println!("Direct error: {:.64}", direct_error);
    println!("Rust error: {:.64}", rust_error);
}