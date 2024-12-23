mod epsilon;

fn main() {
    let eps: f64 = epsilon::machine_epsilon();
    println!("Epsilon: {}", eps);    
}