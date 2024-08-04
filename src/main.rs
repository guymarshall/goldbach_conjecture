use std::{process::exit, time::Instant};

fn is_prime(number: usize) -> bool {
    if number <= 1 {
        return false;
    }
    if number == 2 {
        return true;
    }
    if number % 2 == 0 {
        return false;
    }

    let ceiling_root: usize = (number as f64).sqrt().ceil() as usize;

    (3..=ceiling_root)
        .step_by(2)
        .all(|i: usize| number % i != 0)
}

fn primes_add_up_to_number(number_to_check: usize) -> bool {
    if is_prime(number_to_check - 2) {
        return true;
    }

    (3..)
        .step_by(2)
        .take((number_to_check - 3) / 2)
        .filter(|&number| is_prime(number))
        .any(|number: usize| is_prime(number_to_check - number))
}

fn main() {
    let start_time: Instant = Instant::now();

    const LIMIT: usize = 1_000_000_000;

    (4..)
        .step_by(2)
        .take((LIMIT - 4) / 2)
        .for_each(|number_to_check: usize| {
            if number_to_check % 10000 == 0 {
                println!("{:.4}", (number_to_check as f64 / LIMIT as f64) * 100.0);
            }
            if !primes_add_up_to_number(number_to_check) {
                println!("No two primes add to make {number_to_check}");
                exit(0);
            }
        });

    let end_time: Instant = Instant::now();

    println!("Elapsed time: {:?}", end_time - start_time);
}
