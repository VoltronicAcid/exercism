use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    HashSet::from_iter(possible_anagrams.iter().filter_map(|&candidate| {
        (word.to_lowercase() != candidate.to_lowercase() && sorted(word) == sorted(&candidate))
            .then_some(candidate)
    }))
}

fn sorted(word: &str) -> Vec<char> {
    let mut letters: Vec<char> = word.to_lowercase().chars().collect();
    letters.sort_unstable();

    letters
}
