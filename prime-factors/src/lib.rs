pub fn factors(n: u64) -> Vec<u64> {
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
    match (n, last_prime) {
        (n, last_prime) if is_factor(n, last_prime) => last_prime,
        (_, 2) => find_next_prime_factor(n, 3),
        (n, last_prime) => {
            let last_possible_prime = (n as f64).sqrt().ceil() as u64;
            (last_prime..last_possible_prime)
                .step_by(2)
                .find(|&x| is_factor(n, x))
                .unwrap_or(n)
        }
    }
}

fn is_factor(n: u64, factor: u64) -> bool {
    n % factor == 0
}
