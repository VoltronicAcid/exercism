pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let nucleotides = rna.chars().collect::<Vec<char>>();
    let codons = nucleotides
        .chunks(3)
        .into_iter()
        .map(|chunk| chunk.into_iter().collect::<String>());

    let mut proteins: Vec<&str> = Vec::new();
    for codon in codons {
        if codon.len() != 3 {
            return None;
        }

        match codon.as_str() {
            "AUG" => proteins.push("Methionine"),
            "UGG" => proteins.push("Tryptophan"),
            "UUA" | "UUG" => proteins.push("Leucine"),
            "UAU" | "UAC" => proteins.push("Tyrosine"),
            "UGU" | "UGC" => proteins.push("Cysteine"),
            "UUU" | "UUC" => proteins.push("Phenylalanine"),
            "UAA" | "UAG" | "UGA" => {
                break;
            }
            "UCU" | "UCC" | "UCA" | "UCG" => proteins.push("Serine"),
            _ => {
                return None;
            }
        }
    }

    Some(proteins)
}
