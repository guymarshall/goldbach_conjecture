fn is_prime(number: usize) -> bool {
    if number == 2 || number == 3 {
        return true;
    }

    if number <= 1 || number % 2 == 0 || number % 3 == 0 {
        return false;
    }

    for i in (5..).step_by(6) {
        if i * i > number {
            break;
        }
        if number % i == 0 || number % (i + 2) == 0 {
            return false;
        }
    }

    true
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
