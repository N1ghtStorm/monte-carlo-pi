use rand::Rng;

const LCG_MULT: u64 = 6364136223846793005;
const LCG_INC: u64 = 1442695040888963407;

fn main() {
    let n = 100_000_000;

    println!("--- rand ---");
    let (pi_rand, inside_rand) = estimate_pi_rand(n);
    println!("Points: {}", n);
    println!("Inside quarter circle: {}", inside_rand);
    println!("Pi estimate: {}", pi_rand);
    println!("Error: {:.2e}\n", (std::f64::consts::PI - pi_rand).abs());

    println!("--- LCG ---");
    let (pi_lcg, inside_lcg) = estimate_pi_lcg(n);
    println!("Points: {}", n);
    println!("Inside quarter circle: {}", inside_lcg);
    println!("Pi estimate: {}", pi_lcg);
    println!("Error: {:.2e}\n", (std::f64::consts::PI - pi_lcg).abs());

    println!("Difference (rand - LCG): {:.2e}", pi_rand - pi_lcg);
}

fn estimate_pi_rand(n: u64) -> (f64, u64) {
    let mut rng = rand::thread_rng();
    let mut inside = 0u64;
    for _ in 0..n {
        let x: f64 = rng.r#gen();
        let y: f64 = rng.r#gen();
        if x * x + y * y <= 1.0 {
            inside += 1;
        }
    }
    let pi = 4.0 * (inside as f64) / (n as f64);
    (pi, inside)
}

fn estimate_pi_lcg(n: u64) -> (f64, u64) {
    let mut state = 1u64;
    let mut inside = 0u64;
    for _ in 0..n {
        let x = lcg_01(&mut state);
        let y = lcg_01(&mut state);
        if x * x + y * y <= 1.0 {
            inside += 1;
        }
    }
    let pi = 4.0 * (inside as f64) / (n as f64);
    (pi, inside)
}

fn lcg_01(state: &mut u64) -> f64 {
    *state = state.wrapping_mul(LCG_MULT).wrapping_add(LCG_INC);
    *state as f64 / (u64::MAX as f64 + 1.0)
}
