pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut acids = vec![];

    for chunk in rna.chars().collect::<Vec<char>>().chunks(3) {
        match chunk {
            ['A', 'U', 'G'] => acids.push("Methionine"),
            ['U', 'U', 'C'] | ['U', 'U', 'U'] => acids.push("Phenylalanine"),
            ['U', 'U', 'A'] | ['U', 'U', 'G'] => acids.push("Leucine"),
            ['U', 'C', 'U'] | ['U', 'C', 'C'] | ['U','C', 'A'] | ['U','C','G'] => acids.push("Serine"),
            ['U', 'A', 'U'] | ['U', 'A', 'C'] => acids.push("Tyrosine"),
            ['U', 'G', 'U'] | ['U', 'G', 'C'] => acids.push("Cysteine"),
            ['U', 'G', 'G'] => acids.push("Tryptophan"),
            ['U', 'A', 'A'] | ['U', 'A', 'G'] | ['U', 'G', 'A'] => break,
            _ => return None,
        }
    }

    Some(acids)
}
