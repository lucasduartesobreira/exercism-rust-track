#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<Option<u16>>,
    max_rolls: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            rolls: Vec::new(),
            max_rolls: 21,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.rolls.len() == 20 {
            let last_roll = self.rolls.last().unwrap().unwrap_or_default();
            let last_last_roll = self.rolls.get(19).unwrap().unwrap();

            if last_roll + last_last_roll == 10 {
                self.max_rolls = 22;
            }
        }

        if self.rolls.len() >= self.max_rolls {
            return Err(Error::GameComplete);
        }

        if self.rolls.len() % 2 != 0 || self.rolls.is_empty() {
            if pins > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }

            self.rolls.push(Some(pins));

            if pins == 10 {
                self.rolls.push(None);
            }
        } else {
            let first_throw = self.rolls.last().unwrap();

            if first_throw.unwrap() + pins <= 10 {
                self.rolls.push(Some(pins));
            } else {
                return Err(Error::NotEnoughPinsLeft);
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        println!("{:?}", self.rolls);
        let multiplier_iter =
            self.rolls
                .iter()
                .enumerate()
                .map(|(index, &opt_pins)| match opt_pins {
                    Some(pins) => {
                        if pins == 10 {
                            let new_iter = self.rolls[index..].iter();
                            return Some(
                                new_iter
                                    .filter(|x| x.is_none())
                                    .enumerate()
                                    .take_while(|&(index, _)| index < 3)
                                    .fold(0, |acc, (index, sla)| acc + sla.unwrap()),
                            );
                        }
                        None
                    }
                    None => None,
                });
        Some(0)
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}
