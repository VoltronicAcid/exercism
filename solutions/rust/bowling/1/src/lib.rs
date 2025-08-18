#![allow(dead_code, unused_variables)]
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default)]
pub struct BowlingGame {
    frames: Vec<Vec<u16>>,
    is_complete: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_complete {
            return Err(Error::GameComplete);
        }

        let get_new_frame = |pins| -> Result<Vec<u16>, Error> {
            if pins > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }

            let mut frame = Vec::new();
            frame.push(pins);

            Ok(frame)
        };

        match self.frames.last() {
            Some(frame) => {
                let frames_len = self.frames.len();
                let frame_sum = frame.iter().sum::<u16>();

                if frame.len() == 2 || frame_sum == 10 {
                    self.frames.push(get_new_frame(pins)?);
                } else {
                    if pins + frame_sum > 10 {
                        return Err(Error::NotEnoughPinsLeft);
                    }

                    if frames_len == 11 {
                        self.frames.push(get_new_frame(pins)?);
                    } else {
                        self.frames.pop();
                        self.frames.push(vec![frame_sum, pins]);
                    }
                }
            }
            None => {
                self.frames.push(get_new_frame(pins)?);
            }
        }

        match self.frames.get(9) {
            Some(last_frame) => {
                let frame_sum = last_frame.iter().sum::<u16>();
                let last_frame_is_strike = last_frame.len() == 1 && frame_sum == 10;
                let last_frame_is_spare = last_frame.len() == 2 && frame_sum == 10;
                let last_frame_is_open_frame = last_frame.len() == 2 && frame_sum < 10;

                self.is_complete = match self.frames.len() {
                    10 => last_frame_is_open_frame,
                    11 => last_frame_is_spare,
                    12 => last_frame_is_strike,
                    _ => false,
                }
            }
            None => {
                self.is_complete = false;
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_complete {
            return None;
        }

        let score = self.frames[0..10]
            .iter()
            .enumerate()
            .fold(0, |acc: u16, (idx, rolls)| {
                let mut frame_score = rolls.iter().sum::<u16>();
                let is_strike = rolls.len() == 1 && frame_score == 10;
                let is_spare = rolls.len() == 2 && frame_score == 10;

                if is_strike || is_spare {
                    let next_frame = &self.frames[idx + 1];
                    frame_score += next_frame[0];

                    if is_strike && next_frame.len() == 2 {
                        frame_score += next_frame[1];
                    } else if is_strike && next_frame.len() == 1 {
                        frame_score += self.frames[idx + 2][0];
                    }
                }

                acc + frame_score
            });

        Some(score)
    }
}
