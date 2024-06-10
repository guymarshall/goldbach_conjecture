use std::process::exit;
use std::time::Instant;

const COUNT: i32 = 1_000_000;

const fn int_sqrt(number: i32) -> i32 {
    let mut x: i32 = number;
    let mut y: i32 = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + number / x) / 2;
    }
    x
}

const fn is_prime(number: i32) -> bool {
    if number <= 1 {
        return false;
    }
    if number == 2 {
        return true;
    }
    if number % 2 == 0 {
        return false;
    }
    let limit: i32 = int_sqrt(number);
    let mut i: i32 = 3;
    while i <= limit {
        if number % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn generate_primes() -> [i32; COUNT as usize] {
    let mut primes: [i32; COUNT as usize] = [0i32; COUNT as usize];
    let mut count: i32 = 0;
    let mut number: i32 = 2;

    while count < COUNT {
        if is_prime(number) {
            primes[count as usize] = number;
            count += 1;
        }
        number += 1;
    }

    primes
}

fn primes_add_up_to_number(primes: &[i32], number_to_check: i32) -> bool {
    primes.iter().any(|prime: &i32| {
        primes.contains(&(number_to_check - prime))
    })
}

fn main() {
    let start_time: Instant = Instant::now();

    const LIMIT: i32 = COUNT * 1000;

    println!("Generating primes upto {COUNT}...");
    let primes: [i32; COUNT as usize] = generate_primes();

    println!("Checking sums of primes...");
    (4..LIMIT).step_by(2).for_each(|number_to_check: i32| {
        if number_to_check % 1000 == 0 {
            println!("{:.4}%", (number_to_check as f64 / LIMIT as f64) * 100.0);
        }
        if !primes_add_up_to_number(&primes, number_to_check) {
            println!("No two primes add to make {number_to_check}");
            exit(0);
        }
    });

    let end_time: Instant = Instant::now();

    println!("Elapsed time: {:?}", end_time - start_time);
}
