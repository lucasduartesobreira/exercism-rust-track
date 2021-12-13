#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, PartialEq)]
pub enum Frame {
    Open(u16, u16),
    Spare(u16),
    Bonus(u16),
    Strike,
}

#[derive(Debug, PartialEq)]
enum RollingState {
    Regular,
    ExtraSpare,
    ExtraStrike(u16),
    Over,
}

pub struct BowlingGame {
    pub frames: Vec<Frame>,
    partial_score: Option<u16>,
    rolling_state: RollingState,
}

impl Frame {
    fn first_throw(&self) -> u16 {
        match *self {
            Frame::Open(t, _) => t,
            Frame::Spare(t) => t,
            Frame::Bonus(t) => t,
            Frame::Strike => 10,
        }
    }

    fn pins_down(&self) -> u16 {
        match *self {
            Frame::Open(t1, t2) => t1 + t2,
            Frame::Spare(_) => 10,
            Frame::Bonus(t) => t,
            Frame::Strike => 10,
        }
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: vec![],
            partial_score: None,
            rolling_state: RollingState::Regular,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        match self.rolling_state {
            RollingState::Over => return Err(Error::GameComplete),
            RollingState::Regular => {
                match (self.partial_score, pins) {
                    (None, 10) => self.frames.push(Frame::Strike),
                    (None, pins) => self.partial_score = Some(pins),
                    (Some(first_throw), second_throw) if first_throw + second_throw > 10 => {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                    (Some(first_throw), second_throw) if first_throw + second_throw == 10 => {
                        self.frames.push(Frame::Spare(first_throw));
                        self.partial_score = None;
                    }
                    (Some(first_throw), second_throw) => {
                        self.frames.push(Frame::Open(first_throw, second_throw));
                        self.partial_score = None;
                    }
                }

                if self.frames.len() == 10 {
                    match self.frames.last() {
                        Some(Frame::Strike) => self.rolling_state = RollingState::ExtraStrike(2),
                        Some(Frame::Spare(_)) => self.rolling_state = RollingState::ExtraSpare,
                        _ => self.rolling_state = RollingState::Over,
                    }
                }
            }
            RollingState::ExtraStrike(extra_count) => {
                let frame = match (self.partial_score, pins) {
                    (None, 10) => Frame::Strike,
                    (None, first_throw) => {
                        self.partial_score = Some(first_throw);
                        Frame::Bonus(pins)
                    }
                    (Some(first_throw), second_throw) if first_throw + second_throw > 10 => {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                    _ => Frame::Bonus(pins),
                };

                self.frames.push(frame);

                self.rolling_state = if extra_count == 1 {
                    RollingState::Over
                } else {
                    RollingState::ExtraStrike(extra_count - 1)
                };
            }
            RollingState::ExtraSpare => {
                let frame = match pins {
                    10 => Frame::Strike,
                    _ => Frame::Bonus(pins),
                };

                self.frames.push(frame);

                self.rolling_state = RollingState::Over;
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.rolling_state != RollingState::Over {
            self.frames.iter().for_each(|f| print!("{:?}, ", f));
            return None;
        }
        let pins_down: u16 = self.frames.iter().map(Frame::pins_down).sum();
        let bonus: u16 = self.frames[0..10]
            .windows(3)
            .map(|window| match (&window[0], &window[1], &window[2]) {
                (Frame::Strike, Frame::Strike, frame3) => 10 + frame3.first_throw(),
                (Frame::Strike, frame2, _) => frame2.pins_down(),
                (Frame::Spare(_), frame2, _) => frame2.first_throw(),
                _ => 0,
            })
            .sum();

        let bonus_9th_frame: u16 = if self.frames.len() > 10 {
            match &self.frames[8..=10] {
                [Frame::Strike, Frame::Strike, frame] => 10 + frame.first_throw(),
                _ => 0,
            }
        } else {
            0
        };

        self.frames.iter().for_each(|f| print!("{:?}, ", f));

        println!("score: {}, bonus: {}, extra_points: ", pins_down, bonus);

        Some(pins_down + bonus + bonus_9th_frame)
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}
