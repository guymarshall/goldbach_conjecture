use std::process::exit;
use std::time::Instant;

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

fn primes_add_up_to_number(number_to_check: i32) -> bool {
    if is_prime(number_to_check - 2) {
        return true;
    }
    (3..=number_to_check)
        .step_by(2)
        .filter(|number| is_prime(*number))
        .any(|number| { is_prime(number_to_check - number) })
}

fn main() {
    let start_time: Instant = Instant::now();

    const COUNT: i32 = 1_000_000;
    const LIMIT: i32 = COUNT * 1000;

    (4..LIMIT).step_by(2).for_each(|number_to_check: i32| {
        if number_to_check % 1000 == 0 {
            println!("{:.4}%", (number_to_check as f64 / LIMIT as f64) * 100.0);
        }
        if !primes_add_up_to_number(number_to_check) {
            println!("No two primes add to make {number_to_check}");
            exit(0);
        }
    });

    let end_time: Instant = Instant::now();

    println!("Elapsed time: {:?}", end_time - start_time);
}
