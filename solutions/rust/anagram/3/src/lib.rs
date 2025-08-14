use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let target = sorted(word);

    let matches = possible_anagrams
        .iter()
        .filter_map(|candidate| {
            if sorted(candidate) == target && word.to_lowercase() != candidate.to_lowercase() {
                Some(*candidate)
            } else {
                None
            }
        })
        .collect::<Vec<&str>>();

    HashSet::from_iter(matches.into_iter())
}

fn sorted(word: &str) -> String {
    let mut letters: Vec<char> = word.to_lowercase().chars().collect();
    letters.sort_unstable();

    letters.into_iter().collect()
}
