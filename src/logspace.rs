pub fn logspace(start: f64, end: f64, num: usize) -> Vec<f64> {
    let step = (end - start) / (num - 1) as f64;
    (0..num).map(|i| 10f64.powf(start + i as f64 * step)).collect()
}