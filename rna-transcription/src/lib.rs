#[derive(Debug, PartialEq)]
pub struct Dna(String);

#[derive(Debug, PartialEq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some((index_error, _)) = dna
            .chars()
            .enumerate()
            .find(|(_, c)| *c != 'A' && *c != 'C' && *c != 'G' && *c != 'T')
        {
            return Err(index_error);
        }

        Ok(Dna(dna.to_string()))
    }

    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                _ => 'U',
            })
            .collect::<String>())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if let Some((index_error, _)) = rna
            .chars()
            .enumerate()
            .find(|(_, c)| *c != 'A' && *c != 'C' && *c != 'G' && *c != 'U')
        {
            return Err(index_error);
        }
        Ok(Rna(rna.to_string()))
    }
}
