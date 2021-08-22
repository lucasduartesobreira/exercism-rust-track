pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.0).fold(Vec::new(), |mut rows, row_number| {
            rows.push(Self::nth_row(row_number));
            rows
        })
    }

    fn nth_row(n_row: u32) -> Vec<u32> {
        (0..n_row).fold(vec![1u32], |mut row, collum| {
            row.push(row.last().unwrap() * (n_row - collum) / (collum + 1));
            row
        })
    }
}
