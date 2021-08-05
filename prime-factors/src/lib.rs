pub fn factors(n: u64) -> Vec<u64> {
    if n == 1 {
        return vec![];
    }

    let mut vec = vec![];
    let mut temp = n;
    let mut last_prime = 3;
    while temp != 1 {
        if temp % 2 == 0 {
            vec.push(2);
            temp /= 2;
            continue;
        }

        last_prime = find_next_prime_factor(temp, last_prime);
        vec.push(last_prime);
        temp /= last_prime;
    }

    vec
}

fn find_next_prime_factor(n: u64, last_prime: u64) -> u64 {
    if n % last_prime == 0 {
        return last_prime;
    }
    let last_possible_prime = (n as f64).sqrt().ceil() as u64;
    (last_prime..last_possible_prime)
        .step_by(2)
        .find(|x| n % x == 0)
        .unwrap_or(n)
}
