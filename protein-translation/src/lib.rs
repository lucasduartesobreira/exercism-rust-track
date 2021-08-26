pub struct CodonsInfo<'a> {
    pairs: Vec<(&'a str, &'a str)>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        let (_, name) = self.pairs.iter().find(|(c, _)| *c == codon)?;
        Some(*name)
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

        for codon in codons {
            let codon = self.name_for(codon.as_str());
            match codon {
                Some(codon) => {
                    println!("{}", codon);
                    if codon == "stop codon" {
                        break;
                    }
                    codons_name.push(codon);
                }
                None => return None,
            }
        }

        Some(codons_name)
        /*
         *            .try_fold(Vec::new(), |mut acc, x| {
         *                let x = self.name_for(x.as_str())?;
         *
         *                if x == "stop condon" {
         *                    return None;
         *                }
         *
         *                acc.push(x);
         *                Some(acc)
         *            })
         */
        /*
         *unimplemented!("Return a list of protein names that correspond to the '{}' RNA string or None if the RNA string is invalid", rna);
         */
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo { pairs }
}
