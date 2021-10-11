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
                            let sum_of_surround =
                                u32::from(self.sum_of_surround(row_index, col_index));
                            if sum_of_surround != 0 {
                                return char::from_digit(sum_of_surround, 10).unwrap();
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

        (row.saturating_sub(1)..=row + 1)
            .filter_map(|row_index| minefield.get(row_index))
            .fold(0, |acc, actual_row| {
                acc + (collum.saturating_sub(1)..=collum + 1)
                    .filter_map(|collum_index| actual_row.get(collum_index))
                    .filter(|&actual_char| actual_char == &'*')
                    .count() as u8
            })
    }
}
