use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    input
        .iter()
        .flat_map(|(points, letters)| {
            letters
                .iter()
                .map(|uppercase| (uppercase.to_ascii_lowercase(), *points))
        })
        .collect::<BTreeMap<char, i32>>()
}
