use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    nucleotide_counts(dna)?
        .get(&nucleotide)
        .map_or(Err(nucleotide), |count| Ok(*count))
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    dna.chars().try_fold(
        HashMap::from([('A', 0_usize), ('C', 0), ('G', 0), ('T', 0)]),
        |mut counts, c| match counts.contains_key(&c) {
            true => {
                counts.entry(c).and_modify(|count| *count += 1);
                Ok(counts)
            }
            false => Err(c),
        },
    )
}
