use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours, minutes) = Clock::get_hours_minutes(hours, minutes);

        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (hours, minutes) = Clock::get_hours_minutes(self.hours, self.minutes + minutes);

        Self { hours, minutes }
    }

    fn get_hours_minutes(hours: i32, minutes: i32) -> (i32, i32) {
        let mins_per_day: i32 = 60 * 24;
        let total = hours * 60 + minutes;

        match total {
            total if total < 0 => {
                let mod_minutes: i32 = mins_per_day + (total % mins_per_day);
                (mod_minutes / 60, mod_minutes % 60)
            }
            _ => ((total / 60) % 24, total % 60),
        }
    }
}
