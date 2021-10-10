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

        (row.saturating_sub(1)..=row + 1).fold(0, |acc, r| {
            let line = minefield.get(r);
            match line {
                Some(line) => {
                    (collum.saturating_sub(1)..=collum + 1).fold(acc, |sum, c| match line.get(c) {
                        Some('*') => sum + 1,
                        _ => sum,
                    })
                }
                None => acc,
            }
        })
    }
}
