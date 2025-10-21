pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let stop_codons = ["UAA", "UAG", "UGA"];
    rna.as_bytes()
        .chunks(3)
        .map(|chunk| str::from_utf8(chunk).unwrap())
        .take_while(|codon| !stop_codons.contains(codon))
        .try_fold(vec![], |acc, chunk| {
            match chunk {
                "AUG" => Some([acc, vec!["Methionine"]].concat()),
                "UUC" | "UUU" => Some([acc, vec!["Phenylalanine"]].concat()),
                "UUA" | "UUG" => Some([acc, vec!["Leucine"]].concat()),
                "UCU" | "UCC" | "UCA" | "UCG" => Some([acc, vec!["Serine"]].concat()),
                "UAU" | "UAC" => Some([acc, vec!["Tyrosine"]].concat()),
                "UGU" | "UGC" => Some([acc, vec!["Cysteine"]].concat()),
                "UGG" => Some([acc, vec!["Tryptophan"]].concat()),
                _ => None,
            }
        })
}
