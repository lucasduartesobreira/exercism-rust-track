pub struct Allergies {
    allergies: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        const ALLERGIES_VALUE: [(u32, Allergen); 8] = [
            (1, Allergen::Eggs),
            (2, Allergen::Peanuts),
            (4, Allergen::Shellfish),
            (8, Allergen::Strawberries),
            (16, Allergen::Tomatoes),
            (32, Allergen::Chocolate),
            (64, Allergen::Pollen),
            (128, Allergen::Cats),
        ];

        let score = score % 256;

        Self {
            allergies: ALLERGIES_VALUE
                .iter()
                .rev()
                .fold(
                    (score, Vec::new()),
                    |(score, mut allergies), &(value, allergie)| {
                        println!("{:?}", (score, value));
                        if score >= value {
                            allergies.push(allergie);
                            (score - value, allergies)
                        } else {
                            (score, allergies)
                        }
                    },
                )
                .1,
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.clone()
    }
}
