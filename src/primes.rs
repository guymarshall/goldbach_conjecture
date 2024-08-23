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

pub(crate) fn primes_add_up_to_number(number_to_check: usize) -> bool {
    if is_prime(number_to_check - 2) {
        return true;
    }

    (3..)
        .step_by(2)
        .take((number_to_check - 3) / 2)
        .filter(|&number| is_prime(number))
        .any(|number: usize| is_prime(number_to_check - number))
}
