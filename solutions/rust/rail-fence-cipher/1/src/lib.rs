use std::collections::{BTreeMap, VecDeque};
use std::iter::{Chain, Cycle, Rev};
use std::ops::Range;
pub struct RailFence {
    rails: usize,
    indices: Cycle<Chain<Range<usize>, Rev<Range<usize>>>>,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        let indices: Cycle<Chain<Range<usize>, Rev<Range<usize>>>> = (0..(rails as usize))
            .chain((1..(rails as usize - 1)).rev())
            .cycle();

        RailFence {
            rails: rails as usize,
            indices,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut idx_iter = self.indices.clone();

        text.chars()
            .fold(Vec::<String>::new(), |mut acc, c| {
                let idx = idx_iter.next().unwrap();

                if idx == acc.len() {
                    acc.push(c.to_string());
                } else {
                    acc[idx] += format!("{c}").as_str();
                }

                acc
            })
            .join("")
    }

    pub fn decode(&self, cipher: &str) -> String {
        let counts =
            self.indices
                .clone()
                .take(cipher.len())
                .fold(BTreeMap::new(), |mut acc, num| {
                    acc.entry(num).and_modify(|count| *count += 1).or_insert(1);

                    acc
                });

        let mut rows: Vec<VecDeque<char>> = vec![VecDeque::new(); self.rails];
        let mut cyph_iter = cipher.chars();

        counts.iter().for_each(|(key, val)| {
            for c in cyph_iter.by_ref().take(*val) {
                rows[*key].push_back(c);
            }
        });

        self.indices
            .clone()
            .take(cipher.len())
            .filter_map(|idx| rows[idx].pop_front())
            .collect::<String>()
    }
}
