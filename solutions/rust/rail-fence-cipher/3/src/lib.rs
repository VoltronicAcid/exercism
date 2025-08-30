use std::iter::{Chain, Cycle, Rev, Take};
use std::ops::Range;

type Indices = Take<Cycle<Chain<Range<usize>, Rev<Range<usize>>>>>;
pub struct RailFence {
    rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        self.get_iter(text.len())
            .zip(text.chars())
            .fold(Vec::<String>::new(), |mut acc, (idx, ch)| {
                match acc.get_mut(idx) {
                    Some(s) => s.push(ch),
                    None => acc.push(ch.to_string()),
                }

                acc
            })
            .join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut char_indices = self.get_iter(cipher.len()).collect::<Vec<usize>>();
        char_indices.sort();

        let mut mapped_chars = char_indices
            .into_iter()
            .zip(cipher.chars())
            .collect::<Vec<(usize, char)>>();

        self.get_iter(cipher.len())
            .filter_map(|idx| {
                let index = mapped_chars.iter().position(|&(i, _)| i == idx)?;
                let (_, c) = mapped_chars.remove(index);

                Some(c)
            })
            .collect::<String>()
    }

    fn get_iter(&self, len: usize) -> Indices {
        (0..self.rails)
            .chain((1..(self.rails - 1)).rev())
            .cycle()
            .take(len)
    }
}
