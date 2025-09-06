pub fn encode(mut n: u64) -> String {
    let words = (0_u64..7)
        .filter_map(|_| {
            if n == 0 {
                return None;
            }
            let val = n % 1000;
            n /= 1000;

            Some(val as u16)
        })
        .map(num_to_word)
        .enumerate()
        .map(append_value_name)
        .filter(|word| !word.is_empty())
        .collect::<Vec<String>>();

    if words.is_empty() {
        String::from("zero")
    } else {
        words.into_iter().rev().collect::<Vec<String>>().join(" ")
    }
}

fn num_to_word(num: u16) -> String {
    let mut words: Vec<String> = Vec::new();
    let hundreds = match num / 100 {
        9 => "nine hundred",
        8 => "eight hundred",
        7 => "seven hundred",
        6 => "six hundred",
        5 => "five hundred",
        4 => "four hundred",
        3 => "three hundred",
        2 => "two hundred",
        1 => "one hundred",
        _ => "",
    }
    .to_string();

    if !hundreds.is_empty() {
        words.push(hundreds);
    }

    let tens = match num % 100 / 10 {
        9 => "ninety",
        8 => "eighty",
        7 => "seventy",
        6 => "sixty",
        5 => "fifty",
        4 => "forty",
        3 => "thirty",
        2 => "twenty",
        1 => match num % 100 {
            19 => "nineteen",
            18 => "eighteen",
            17 => "seventeen",
            16 => "sixteen",
            15 => "fifteen",
            14 => "fourteen",
            13 => "thirteen",
            12 => "twelve",
            11 => "eleven",
            10 => "ten",
            _ => "",
        },
        _ => "",
    }
    .to_string();

    if !tens.is_empty() {
        words.push(tens.clone());

        if num % 100 > 19 && num % 10 > 0 {
            let ones = match num % 10 {
                9 => "-nine",
                8 => "-eight",
                7 => "-seven",
                6 => "-six",
                5 => "-five",
                4 => "-four",
                3 => "-three",
                2 => "-two",
                1 => "-one",
                _ => "",
            };

            if let Some(val) = words.last_mut() {
                *val += ones;
            };
        }
    } else {
        let ones = match num % 10 {
            9 => "nine",
            8 => "eight",
            7 => "seven",
            6 => "six",
            5 => "five",
            4 => "four",
            3 => "three",
            2 => "two",
            1 => "one",
            _ => "",
        }
        .to_string();

        if !ones.is_empty() {
            words.push(ones);
        }
    }

    words.join(" ").trim().to_string()
}

fn append_value_name((idx, mut word): (usize, String)) -> String {
    if !word.is_empty() {
        match idx {
            1 => word.push_str(" thousand"),
            2 => word.push_str(" million"),
            3 => word.push_str(" billion"),
            4 => word.push_str(" trillion"),
            5 => word.push_str(" quadrillion"),
            6 => word.push_str(" quintillion"),
            _ => {}
        }
    }

    word
}
