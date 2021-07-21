// sum_of_multiples take a limit and factors as parameter and return
// the sum of all the unique multiples of the given factors up to, but not including, limit
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|i| factors.iter().any(|factor| factor != &0 && i % factor == 0))
        .sum()
}
