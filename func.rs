fn average(xs: &[i32]) -> f64 {
    let mut sum: i32 = 0;
    for i in 0..xs.len() {
        sum += xs[i];
    }
    sum as f64 / xs.len() as f64
}