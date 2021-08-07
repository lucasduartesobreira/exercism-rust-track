use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_nucleotide(nucleotide) {
        return Err(nucleotide);
    }

    match nucleotide_counts(dna) {
        Ok(hash) => Ok(*hash.get(&nucleotide).unwrap()),
        Err(e) => Err(e),
    }
}

fn is_nucleotide(c: char) -> bool {
    c == 'A' || c == 'C' || c == 'G' || c == 'T'
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    dna.chars().try_fold(
        "ACGT"
            .chars()
            .map(|c| (c, 0))
            .collect::<HashMap<char, usize>>(),
        |mut acc, map| {
            if !is_nucleotide(map) {
                return Err(map);
            }
            *acc.entry(map).or_insert(0) += 1;

            Ok(acc)
        },
    )
}
