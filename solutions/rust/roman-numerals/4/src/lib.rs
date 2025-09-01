use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    numerals: String,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.numerals)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let places: usize = (num as f64).log10() as usize + 1;
        let numerals: String = (0..places)
            .map(|place: usize| {
                let digit: u32 = num / (10_u32).pow(place as u32) % 10;

                get_roman_chars(place, digit)
            })
            .rev()
            .collect::<Vec<String>>()
            .join("");

        Roman { numerals }
    }
}

fn get_roman_chars(place: usize, digit: u32) -> String {
    let place_chars: [(char, char, char); 4] = [
        ('I', 'V', 'X'),
        ('X', 'L', 'C'),
        ('C', 'D', 'M'),
        ('M', ' ', ' '),
    ];
    let (one, five, ten) = place_chars[place];

    match digit {
        1..4 => one.to_string().repeat(digit as usize % 5),
        4 => format!("{one}{five}"),
        5 => String::from(five),
        6..9 => format!("{five}{}", one.to_string().repeat(digit as usize % 5)),
        9 => format!("{one}{ten}"),
        _ => String::from(""),
    }
}
