use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let target = sorted(word);

    HashSet::from_iter(possible_anagrams.iter().filter_map(|candidate| {
        if word.to_lowercase() != candidate.to_lowercase() && sorted(candidate) == target {
            Some(*candidate)
        } else {
            None
        }
    }))
}

fn sorted(word: &str) -> String {
    let mut letters: Vec<char> = word.to_lowercase().chars().collect();
    letters.sort_unstable();

    letters.into_iter().collect()
}
