/*
The sum of the sequence: S_n = \sum_{i = 1}^{n} x_i
Let's assume S_1 = x_1.
Since S_{i + 1} = S_i + x_{i + 1},
implementing this using the Kahan Algorithm:
x'_{i + 1} = x_{i + 1} - c_i
S_{i + 1} = S_i + x'_{i + 1}
c_i = [(S_{i - 1} + x'_i) - S_{i - 1}] - x'_{i} for i >= 2

Let's assume x_0 = 1.0, x_1 = 2.0, x_2 = 3.0, x_3 = 4.0, x_4 = 5.0 and S_0 = 0.0.
x'_0 = 1.0 - 0.0 = 1.0
S'_0 = 0.0
S_0 = S_0 + x'_0 = 0.0 + 1.0 = 1.0
c = [(S'_0 + x'_0) - S'_0] - x'_0 = [0.0 + 1.0 - 0.0] - 1.0 = epsilon

x'_1 = 2.0 - epsilon = 2.0
S'_1 = 1.0
S_1 = 1.0 + 2.0 = 3.0
c = [(S'_0 + x'_0) - S'_0] - x'_0 = [1.0 + 2.0 - 1.0] - 2.0 = epsilon

In this way, the sum of the vector is calculated by subtracting epsilon at each step.
*/

pub fn kahan_summation<T: AsRef<[f64]>>(x: T) -> f64 {
    let x: &[f64] = x.as_ref(); // T 타입의 x를 슬라이스로 변환
    let mut c: f64 = 0.0;
    let mut sum: f64 = 0.0;
    let mut x_prime: f64;
    let mut sum_prime: f64;
    for &xi in x.iter() {
        x_prime = xi - c;
        sum_prime = sum;
        sum = sum + x_prime;
        c = ((sum_prime + x_prime) - sum_prime) - x_prime; // 반올림 오차를 저장.
    }
    sum
}