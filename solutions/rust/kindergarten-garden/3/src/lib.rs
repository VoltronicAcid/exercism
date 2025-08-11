use std::collections::BTreeMap;

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let plants = BTreeMap::from([
        ("G", "grass"),
        ("C", "clover"),
        ("R", "radishes"),
        ("V", "violets"),
    ]);
    let offset = (student[..1].chars().next().unwrap() as usize - 65) * 2;

    diagram
        .lines()
        .flat_map(|row| [&row[offset..(offset + 1)], &row[(offset + 1)..(offset + 2)]])
        .map(|c| *plants.get(&c[..]).unwrap())
        .collect()
}
