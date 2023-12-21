
use rand::{Rng, SeedableRng};

fn main() {
    // ThreadRng
    let mut rng = rand::thread_rng();
    for i in 0..10 {
        println!("ThreadRng {}: {}", i, rng.gen::<f32>());
    }

    println!("\n");

    // StdRng
    let seed = [42; 32];
    let mut rng = rand::rngs::StdRng::from_seed(seed);
    for i in 0..10 {
        println!("StdRng {}: {}", i, rng.gen::<f32>());
    }

    println!("\n");

    // Pcg32
    //
    // NOTE: Since rand_pcg is made available to the compiler, we can use it by
    //       specifying the full path.
    //
    let mut rng = rand_pcg::Pcg32::seed_from_u64(0xDEADBEEF);
    for i in 0..10 {
        println!("PCG {}: {}", i, rng.gen::<f32>());
    }
}
