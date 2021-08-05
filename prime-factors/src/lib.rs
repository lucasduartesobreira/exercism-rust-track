pub fn factors(n: u64) -> Vec<u64> {
    if n == 1 {
        return vec![];
    }

    let mut factors = vec![];
    let mut n = n;
    let mut last_prime = 2;

    while n != 1 {
        last_prime = find_next_prime_factor(n, last_prime);
        factors.push(last_prime);
        n /= last_prime;
    }

    factors
}

fn find_next_prime_factor(n: u64, last_prime: u64) -> u64 {
    if n % last_prime == 0 {
        return last_prime;
    }

    if last_prime == 2 {
        return find_next_prime_factor(n, 3);
    }

    let last_possible_prime = (n as f64).sqrt().ceil() as u64;
    (last_prime..last_possible_prime)
        .step_by(2)
        .find(|x| n % x == 0)
        .unwrap_or(n)
}
