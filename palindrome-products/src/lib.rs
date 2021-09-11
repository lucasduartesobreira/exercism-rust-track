#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    a: u64,
    b: u64,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        //unimplemented!("create a palindrome with factors ({}, {})", a, b)
        Palindrome { a, b }
    }

    pub fn value(&self) -> u64 {
        //unimplemented!("return the value of this palindrome")
        self.a * self.b
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        //unimplemented!("insert new factors ({}, {}) into this palindrome", a, b)
        self.a = a;
        self.b = b;
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    /*
     *unimplemented!(
     *    "Find the min and max palindromic numbers which are products of numbers in the inclusive range ({}..{})",
     *    min,
     *    max
     *)
     */
    let mut minimal = None;
    for x in min..=max {
        for y in x + 1..=max {
            let number = x * y;
            let vec = to_vec(number);
            let mut reverse = vec.clone();
            reverse.reverse();

            if vec == reverse {
                minimal = Some(Palindrome::new(x, y));
                break;
            }
        }

        if minimal.is_some() {
            break;
        }
    }

    let mut maximum = None;
    for x in (min..=max).rev() {
        for y in (min + 1..=(x)).rev() {
            let number = x * y;
            let vec = to_vec(number);
            let mut reverse = vec.clone();
            reverse.reverse();

            if vec == reverse {
                maximum = Some(Palindrome::new(y, x));
                break;
            }
        }

        if maximum.is_some() {
            break;
        }
    }

    Some((minimal?, maximum?))
}

fn to_vec(value: u64) -> Vec<u32> {
    value
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<_>()
}
