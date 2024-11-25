mod primes;

use primes::primes_add_up_to_number;
use rayon::prelude::*;
use std::{process::exit, time::Instant};

fn main() {
    let start_time: Instant = Instant::now();

    const LIMIT: usize = 1_000_000_000;

    (4..)
        .step_by(2)
        .take((LIMIT - 4) / 2)
        .par_bridge()
        .for_each(|number_to_check: usize| {
            if number_to_check % 10000 == 0 {
                print!("\r{:.4}%", (number_to_check as f64 / LIMIT as f64) * 100.0);
            }
            if !primes_add_up_to_number(number_to_check) {
                println!("\nNo two primes add to make {number_to_check}");
                exit(0);
            }
        });
    println!();

    let end_time: Instant = Instant::now();

    println!("Elapsed time: {:?}", end_time - start_time);
}
