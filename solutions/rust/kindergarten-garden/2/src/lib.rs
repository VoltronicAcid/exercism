use std::collections::BTreeMap;

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let mut result: Vec<&str> = Vec::new();
    let plants = BTreeMap::from([
        ('G', "grass"),
        ('C', "clover"),
        ('R', "radishes"),
        ('V', "violets"),
    ]);
    let offset = (student[..1].chars().next().unwrap() as usize - 65) * 2;
    
    for row in diagram.lines() {
        for c in row[offset..(offset + 2)].chars() {
            result.push(plants.get(&c).unwrap());
        }
    }

    result
}
