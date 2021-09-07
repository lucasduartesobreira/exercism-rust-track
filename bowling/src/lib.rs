#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: Vec<Frame>,
}

struct Frame {
    first_throw: u16,
    second_throw: Option<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { frames: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.frames.len() >= 10 {
            return Err(Error::GameComplete);
        }

        let last_frame = self.frames.last_mut();
        match last_frame {
            Some(last_frame) => {
                if last_frame.has_a_second_shot_to_go() {
                    last_frame.set_second_shot(pins)?;
                } else {
                    let new_frame = Frame::new(pins)?;
                    self.frames.push(new_frame);
                }

                Ok(())
            }
            None => {
                self.frames.push(Frame::new(pins)?);
                Ok(())
            }
        }
    }

    pub fn score(&self) -> Option<u16> {
        (0..self.frames.len()).try_fold(0, |acc, index| Some(acc + self.get_frame_score(index)?))
    }

    fn get_frame_score(&self, index: usize) -> Option<u16> {
        let actual_frame = self.frames.get(index)?;

        if index == 9 {
            if actual_frame.is_an_spare() || actual_frame.is_an_strike() {
                return Some(actual_frame.sum_pins() + self.frames[index + 1].sum_pins());
            }
            return Some(actual_frame.sum_pins());
        }

        if actual_frame.is_an_strike() {
            let next_frame = self.frames.get(index + 1)?;

            if next_frame.is_an_strike() {
                return Some(20 + self.frames.get(index + 2)?.sum_pins());
            }
        }

        if actual_frame.is_an_spare() {
            return Some(20 + self.frames.get(index + 1)?.first_throw);
        }

        Some(actual_frame.sum_pins())
    }
}

impl Frame {
    fn new(pins: u16) -> Result<Self, Error> {
        if pins >= 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        Ok(Self {
            first_throw: pins,
            second_throw: None,
        })
    }

    fn has_a_second_shot_to_go(&self) -> bool {
        self.second_throw.is_none() && self.first_throw != 10
    }

    fn set_second_shot(&mut self, pins: u16) -> Result<(), Error> {
        if self.first_throw + pins >= 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        self.second_throw = Some(pins);

        Ok(())
    }

    fn is_an_strike(&self) -> bool {
        self.first_throw == 10
    }

    fn is_an_spare(&self) -> bool {
        self.first_throw + self.second_throw.unwrap_or_default() == 10
    }

    fn sum_pins(&self) -> u16 {
        self.first_throw + self.second_throw.unwrap_or_default()
    }
}
