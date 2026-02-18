use rand::Rng;

fn main() {
    let n = 100_000_000;

    let mut rng = rand::thread_rng();
    let mut inside = 0u64;

    for _ in 0..n {
        let x: f64 = rng.r#gen();
        let y: f64 = rng.r#gen();
        if x * x + y * y <= 1.0 {
            inside += 1;
        }
    }

    let pi_estimate = 4.0 * (inside as f64) / (n as f64);
    println!("Points: {}", n);
    println!("Inside quarter circle: {}", inside);
    println!("Pi estimate: {}", pi_estimate);
    println!("Error: {:.2e}", (std::f64::consts::PI - pi_estimate).abs());
}
