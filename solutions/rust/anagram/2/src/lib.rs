use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let matches: &mut HashSet<&str> = &mut HashSet::new();
    let target = get_character_counts(word);

    for candidate in possible_anagrams.iter() {
        let count = get_character_counts(candidate);

        if !words_are_equal(word, candidate) && count == target {
            matches.insert(candidate);
        }
    }

    matches.clone()
}

fn words_are_equal(word1: &str, word2: &str) -> bool {
    word1.to_lowercase() == word2.to_lowercase()
}

fn get_character_counts(word: &str) -> HashMap<String, u32> {
    word.chars().fold(HashMap::new(), |mut map, c| {
        map.entry(c.to_lowercase().to_string())
            .and_modify(|count| *count += 1)
            .or_insert(1);

        map
    })
}
