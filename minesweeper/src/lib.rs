pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let field = MineField::parse(minefield);
    field.serialize()
}

#[derive(Debug)]
struct MineField {
    field: Vec<Vec<char>>,
    heigth: usize,
    width: usize,
}

impl MineField {
    fn parse(minefield: &[&str]) -> MineField {
        let heigth = minefield.len();
        if heigth == 0 {
            return Self {
                field: Vec::new(),
                heigth: 0,
                width: 0,
            };
        }

        let width = minefield[0].len();

        Self {
            field: minefield
                .iter()
                .map(|field| field.chars().collect())
                .collect(),
            heigth,
            width,
        }
    }

    fn serialize(&self) -> Vec<String> {
        if self.heigth == 0 {
            return vec![];
        }

        if self.width == 0 {
            return vec!["".to_string()];
        }

        self.field
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                row.iter()
                    .enumerate()
                    .map(|(col_index, &col)| {
                        if col == ' ' {
                            let sum_of_surround = self.sum_of_surround(row_index, col_index);
                            if sum_of_surround != 0 {
                                return (b'0' + sum_of_surround as u8) as char;
                            }
                        }
                        col
                    })
                    .collect()
            })
            .collect()
    }

    fn sum_of_surround(&self, row: usize, collum: usize) -> u8 {
        let minefield = &self.field;

        let mut sum = 0;
        for r in [-1, 0, 1] {
            let r = r + row as i64;
            for c in [-1, 0, 1] {
                let c = c + collum as i64;
                if r >= 0
                    && r < self.heigth as i64
                    && c >= 0
                    && c < self.width as i64
                    && minefield[r as usize][c as usize] == '*'
                {
                    sum += 1;
                }
            }
        }

        sum
    }
}
