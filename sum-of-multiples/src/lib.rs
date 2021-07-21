// sum_of_multiples take a limit and factors as parameter and return
// the sum of all the unique multiples of the given factors up to, but not including, limit
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum_of_multiples = 0;
    let filtered_factors = filter_0_factor(factors);

    for i in 1..limit {
        for factor in &filtered_factors {
            if i % factor == 0 {
                sum_of_multiples += i;
                break;
            }
        }
    }

    sum_of_multiples
}

// filter_0_factor take the factors and return a vector containing all factors that aren't 0
fn filter_0_factor(factors: &[u32]) -> Vec<u32> {
    let mut filtered = Vec::new();

    for factor in factors {
        if *factor != 0 {
            filtered.push(*factor);
        }
    }
    filtered
}
