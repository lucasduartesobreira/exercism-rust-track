use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Palindrome {
    a: u64,
    b: u64,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome { a, b }
    }

    pub fn value(&self) -> u64 {
        self.a * self.b
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.a = a;
        self.b = b;
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
        .find(|p| is_palindrome(p.a, p.b));

    let max_palindrome: Option<Palindrome> = (0..=diff)
        .flat_map(|dec| (0..=dec).map(move |i| (max - dec + i, max - i)))
        .filter(|(a, b)| a >= &min && b >= &min)
        .map(|(a, b)| Palindrome::new(a, b))
        .find(|p| is_palindrome(p.a, p.b));

    Some((min_palindrome?, max_palindrome?))
}

fn is_palindrome(x: u64, y: u64) -> bool {
    let number = x * y;
    let number_as_string = number.to_string();
    let mut number_in_char = number_as_string.chars();
    let mut reversed_number_iter = number_in_char.clone().rev();

    number_in_char.all(|c| c == reversed_number_iter.next().unwrap_or_default())
}
