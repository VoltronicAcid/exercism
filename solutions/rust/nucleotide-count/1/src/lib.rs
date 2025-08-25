use std::collections::HashMap;

const VALID_CHARS: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    validate_dna(dna)?;

    if !VALID_CHARS.contains(&nucleotide) {
        return Err(nucleotide);
    }

    Ok(dna.chars().filter(|&ch| ch == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    validate_dna(dna)?;

    let count = VALID_CHARS.iter().fold(HashMap::new(), |mut acc, &ch| {
        if let Ok(total) = count(ch, dna) {
            acc.insert(ch, total);
        }

        acc
    });

    Ok(count)
}

fn validate_dna(dna: &str) -> Result<(), char> {
    if let Some(invalid_char) = dna.chars().find(|ch| !VALID_CHARS.contains(ch)) {
        return Err(invalid_char);
    }

    Ok(())
}
