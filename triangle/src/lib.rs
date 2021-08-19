pub struct Triangle {
    a: u64,
    b: u64,
    c: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        match sides {
            [a, b, c] if a == 0 || b == 0 || c == 0 => None,
            [a, b, c] if a > b + c || b > a + c || c > a + b => None,
            [a, b, c] => Some(Self { a, b, c }),
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.a == self.c && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.a != self.c && self.b != self.c
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b && self.a != self.c
            || self.a == self.c && self.b != self.c
            || self.c == self.b && self.a != self.b
    }
}
