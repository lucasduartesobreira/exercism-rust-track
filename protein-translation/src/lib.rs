pub struct CodonsInfo<'a> {
    // This field is here to make the template compile and not to
    // complain about unused type lifetime parameter "'a". Once you start
    // solving the exercise, delete this field and the 'std::marker::PhantomData'
    // import.
    pairs: Vec<(&'a str, &'a str)>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        match codon {
            "AUG" => Some("methionine"),
            "UUU" | "UUC" => Some("phenylalanine"),
            "UUA" | "UUG" => Some("leucine"),
            "UCU" | "UCC" | "UCA" | "UCG" => Some("serine"),
            "UAU" | "UAC" => Some("tyrosine"),
            "UGU" | "UGC" => Some("cysteine"),
            "UGG" => Some("tryptophan"),
            "CGA" | "AGA" | "AGG" => Some("arginine"),
            "AUU" => Some("isoleucine"),
            "GUU" => Some("valine"),
            "UAA" | "UAG" | "UGA" => Some("stop codon"),
            _ => None,
        }
        /*
         *unimplemented!(
         *    "Return the protein name for a '{}' codon or None, if codon string is invalid",
         *    codon
         *);
         */
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
    /*
     *unimplemented!(
     *    "Construct a new CodonsInfo struct from given pairs: {:?}",
     *    pairs
     *);
     */
}
