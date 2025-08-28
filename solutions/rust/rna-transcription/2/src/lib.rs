use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let valid: [char; 4] = ['G', 'C', 'T', 'A'];

        if let Some(idx) = dna.chars().position(|c| !valid.contains(&c)) {
            Err(idx)
        } else {
            Ok(Dna(dna.to_string()))
        }
    }

    pub fn into_rna(self) -> Rna {
        let map: HashMap<char, char> = vec![('G', 'C'), ('C', 'G'), ('T', 'A'), ('A', 'U')]
            .into_iter()
            .collect::<HashMap<char, char>>();

        let rna = self
            .0
            .chars()
            .filter_map(|c| map.get(&c))
            .collect::<String>();

        Rna::new(rna.as_str()).unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let valid: [char; 4] = ['C', 'G', 'A', 'U'];

        if let Some(idx) = rna.chars().position(|c| !valid.contains(&c)) {
            Err(idx)
        } else {
            Ok(Rna(rna.to_string()))
        }
    }
}
