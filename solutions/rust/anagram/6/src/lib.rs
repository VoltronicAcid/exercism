use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let target = sorted(word);
    let lower_word = word.to_lowercase();

    HashSet::from_iter(possible_anagrams.iter().filter_map(|candidate| {
        match lower_word != candidate.to_lowercase() && sorted(candidate) == target {
            true => Some(*candidate),
            false => None,
        }
    }))
}

fn sorted(word: &str) -> String {
    let mut letters: Vec<char> = word.to_lowercase().chars().collect();
    letters.sort_unstable();

    letters.into_iter().collect()
}
