use rand::Rng;
use std::collections::HashSet;
use std::sync::{LazyLock, Mutex};

static NAMES: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: Robot::generate_name(),
        }
    }

    fn generate_name() -> String {
        let mut names = NAMES.lock().unwrap();

        let mut rng = rand::rng();
        loop {
            let lttr1: char = char::from_u32(rng.random_range(65u32..=90)).unwrap();
            let lttr2: char = char::from_u32(rng.random_range(65u32..=90)).unwrap();
            let nmbr = rng.random_range(0..=999);
            let name = format!("{lttr1}{lttr2}{nmbr:03}");

            if !names.contains(&name) {
                names.insert(name.clone());

                return name;
            }
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        self.name = Robot::generate_name();
    }
}
