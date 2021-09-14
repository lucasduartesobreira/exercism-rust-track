use std::{collections::BTreeSet, ops::RangeInclusive};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Palindrome {
    value: u64,
    factors: BTreeSet<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        let mut factors = BTreeSet::new();
        factors.insert((a, b));
        Palindrome {
            factors,
            value: a * b,
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factors.insert((a, b));
    }

    pub fn is_valid(&self) -> bool {
        let number = self.value;
        let number_as_string = number.to_string();
        let mut number_in_char = number_as_string.chars();
        let mut reversed_number_iter = number_in_char.clone().rev();

        number_in_char.all(|c| c == reversed_number_iter.next().unwrap_or_default())
    }

    pub fn update_factors_inside_range(&mut self, range: RangeInclusive<u64>) {
        let primes = self.get_factors();

        if primes.is_empty() {
            return;
        }

        // Combine factors of the palindrome to get all the possible other factors that are in the given range
        (0..primes.len())
            .map(|index| {
                let (first_half, second_half) = primes.split_at(index);

                (first_half.iter().product(), second_half.iter().product())
            })
            .filter(|(f1, f2)| range.contains(f1) && range.contains(f2))
            .for_each(|factors| {
                self.factors.insert(factors);
            });
    }

    fn get_factors(&self) -> Vec<u64> {
        let &(a, b) = self.factors.iter().next().unwrap();

        let a_factors = Factors::new(a);
        let b_factors = Factors::new(b);

        a_factors.chain(b_factors).collect::<Vec<_>>()
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    let diff = max - min;

    let min_palindrome: Option<Palindrome> = (0..=diff)
        .flat_map(|inc| (0..=inc).rev().map(move |i| (min + inc - i, min + i)))
        .filter(|(a, b)| a <= &max && b <= &max)
        .map(|(a, b)| Palindrome::new(a, b))
        .find(|p| p.is_valid());

    let max_palindrome: Option<Palindrome> = (0..=diff)
        .flat_map(|dec| (0..=dec).map(move |i| (max - dec + i, max - i)))
        .filter(|(a, b)| a >= &min && b >= &min)
        .map(|(a, b)| Palindrome::new(a, b))
        .find(|p| p.is_valid());

    match min_palindrome {
        Some(mut min_palindrome) => {
            let mut max_palindrome = max_palindrome.unwrap();

            min_palindrome.update_factors_inside_range(min..=max);

            if min_palindrome.value() == max_palindrome.value() {
                max_palindrome.factors = min_palindrome.factors.clone();
            } else {
                max_palindrome.update_factors_inside_range(min..=max);
            }

            Some((min_palindrome, max_palindrome))
        }
        None => None,
    }
}

struct Factors {
    number: u64,
    actual_factor: u64,
}

impl Factors {
    fn new(number: u64) -> Self {
        Self {
            number,
            actual_factor: 2,
        }
    }
}

impl Iterator for Factors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        (self.actual_factor..=self.number).find(|possible_factor| {
            if self.number % possible_factor == 0 {
                self.number /= possible_factor;
                self.actual_factor = *possible_factor;
                true
            } else {
                false
            }
        })
    }
}
