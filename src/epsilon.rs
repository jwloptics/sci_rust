pub fn machine_epsilon() -> f64{
    let mut eps: f64 = 1.0;

    while (1.0 + eps / 2.0) != 1.0 {
        eps /= 2.0;
    }
    eps
}