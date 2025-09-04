use Direction::*;

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            position: (x, y),
            direction: d,
        }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        self.direction = match self.direction {
            North => East,
            East => South,
            South => West,
            West => North,
        };

        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        self.direction = match self.direction {
            North => West,
            West => South,
            South => East,
            East => North,
        };

        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        match self.direction {
            North => {
                self.position.1 += 1;
            }
            East => {
                self.position.0 += 1;
            }
            South => {
                self.position.1 -= 1;
            }
            West => {
                self.position.0 -= 1;
            }
        }

        self
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |acc, ch| match ch {
            'R' => acc.turn_right(),
            'L' => acc.turn_left(),
            'A' => acc.advance(),
            _ => acc,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
