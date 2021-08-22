pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows = vec![vec![1u32]];
        for i in 1..self.0 {
            rows.push(Self::next_row(rows.get(i as usize - 1).unwrap()));
        }

        rows
    }

    fn next_row(row: &[u32]) -> Vec<u32> {
        (0..=row.len() as u32)
            .map(|x| match x {
                0 => 1,
                _ if x == row.len() as u32 => 1,
                _ => row[x as usize - 1] + row[x as usize],
            })
            .collect()
    }
}
