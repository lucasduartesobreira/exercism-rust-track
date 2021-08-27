use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    pairs: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.pairs.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let codons = rna
            .chars()
            .collect::<Vec<char>>()
            .windows(3)
            .step_by(3)
            .map(|x| x.iter().collect::<String>())
            .collect::<Vec<String>>();

        let mut codons_name = Vec::new();

        let mut found_a_stop = false;
        for codon in codons {
            let codon = self.name_for(codon.as_str());
            match codon {
                Some(codon) => {
                    if codon == "stop codon" {
                        found_a_stop = true;
                        break;
                    }
                    codons_name.push(codon);
                }
                None => return None,
            }
        }

        // HACK: Find another way to deal with invalid lengths
        if !found_a_stop && rna.len() % 3 != 0 {
            return None;
        }

        Some(codons_name)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        pairs: pairs.into_iter().collect(),
    }
}
