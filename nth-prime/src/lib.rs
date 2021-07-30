pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2, 3, 5, 7, 11, 13];
    let n = n as usize;

    if n < primes.len() {
        return primes[n];
    }

    let mut last_prime = primes[primes.len() - 1];
    while primes.len() - 1 != n {
        let temp = last_prime + 2;

        let mut is_prime = true;
        for &prime in &primes {
            if prime as f64 > (temp as f64).sqrt() {
                break;
            }
            if temp % prime == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(temp);
        }

        last_prime = temp;
    }

    last_prime
}
