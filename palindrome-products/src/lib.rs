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
    let map: BTreeMap<u64, Palindrome> = (min..=max)
        .map(|x| {
            (x..=max).filter_map(move |y| {
                if is_palindrome(x, y) {
                    Some((x * y, Palindrome::new(x, y)))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect::<_>();

    if let (Some(min), Some(max)) = (map.iter().next(), map.iter().next_back()) {
        Some((*min.1, *max.1))
    } else {
        None
    }
}

fn is_palindrome(x: u64, y: u64) -> bool {
    let number = x * y;
    let number_as_string = number.to_string();
    let mut number_in_char = number_as_string.chars();
    let mut reversed_number_iter = number_in_char.clone().rev();

    number_in_char.all(|c| c == reversed_number_iter.next().unwrap_or_default())
}
