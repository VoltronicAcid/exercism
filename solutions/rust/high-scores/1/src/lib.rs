#[derive(Debug)]
pub struct HighScores {
    values: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            values: Vec::from(scores),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.values[..]
    }

    pub fn latest(&self) -> Option<u32> {
        match self.values.last() {
            Some(&x) => Some(x),
            _ => None,
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        match self.values.iter().max() {
            Some(&x) => Some(x),
            _ => None,
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted = self.values.clone();
        sorted.sort();
        sorted.reverse();

        if sorted.len() >= 3 {
            sorted[..3].to_vec()
        } else {
            sorted
        }
    }
}
