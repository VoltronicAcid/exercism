use std::fmt::{Display, Formatter, Result};

const ROMAN_CHARS: [char; 7] = ['I', 'V', 'X', 'L', 'C', 'D', 'M'];

pub struct Roman {
    roman: String,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.roman)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let mut roman = String::from("");
        let mut start = 0;
        let mut num = num;

        while num > 0 {
            let chars = get_chars(&ROMAN_CHARS[start..], num % 10);
            roman = format!("{chars}{roman}");
            start += 2;
            num /= 10;
        }

        Roman { roman }
    }
}

fn get_chars(chars: &[char], value: u32) -> String {
    match value {
        1..4 => chars[0].to_string().repeat(value as usize % 5),
        4 => format!("{}{}", chars[0], chars[1]),
        5 => String::from(chars[1]),
        6..9 => format!(
            "{}{}",
            chars[1],
            chars[0].to_string().repeat(value as usize % 5)
        ),
        9 => format!("{}{}", chars[0], chars[2]),
        _ => String::from(""),
    }
}
