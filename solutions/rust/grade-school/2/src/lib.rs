use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub struct School {
    students: BTreeMap<String, u32>,
}

impl School {
    pub fn new() -> Self {
        School::default()
    }

    pub fn grade(&self, num: u32) -> Vec<String> {
        self.students
            .iter()
            .filter_map(|(name, grade)| (*grade == num).then_some(name.to_owned()))
            .collect::<Vec<String>>()
    }

    pub fn add(&mut self, grade: u32, name: &str) {
        self.students.entry(name.to_string()).or_insert(grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        BTreeSet::from_iter(self.students.values())
            .into_iter()
            .copied()
            .collect()
    }
}
