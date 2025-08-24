use std::collections::{HashMap, HashSet};

pub struct School {
    rosters: HashMap<u32, HashSet<String>>,
    students: HashSet<String>,
}

impl School {
    pub fn new() -> School {
        let rosters = HashMap::new();
        let students = HashSet::new();

        School { rosters, students }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self.students.insert(student.to_string()) == false {
            return;
        }

        let student = student.to_string();

        if let Some(names) = self.rosters.get_mut(&grade) {
            names.insert(student);
        } else {
            let mut roster: HashSet<String> = HashSet::new();
            roster.insert(student);
            self.rosters.insert(grade, roster);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut curr_grades: Vec<u32> = self.rosters.clone().into_keys().collect();
        curr_grades.sort();

        curr_grades
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.rosters.get(&grade) {
            Some(names) => {
                let mut names: Vec<String> = names.clone().into_iter().collect();
                names.sort();

                names
            }
            None => Vec::<String>::new(),
        }
    }
}
