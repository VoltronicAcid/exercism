use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h
        .iter()
        .fold(BTreeMap::new(), |mut hash_map, (key, val_vec)| {
            for c in val_vec.iter().flat_map(|c| c.to_lowercase()) {
                hash_map.insert(c, *key);
            }

            hash_map
        })
}
