pub fn encode(mut num: u64) -> String {
    (0_u32..19)
        .step_by(3)
        .rev()
        .fold(Vec::<String>::new(), |mut words, exp| {
            if num == 0 && words.is_empty() {
                words.push(ONES_TEENS[0].to_string());
            } else if let Some(word) = num_to_word(&mut num, exp) {
                words.push(word);
            };

            words
        })
        .join(" ")
}

fn num_to_word(num: &mut u64, exp: u32) -> Option<String> {
    let divisor = 10_u64.pow(exp);

    if *num < divisor {
        None
    } else {
        let mut word;
        let to_word = *num / divisor;
        *num %= divisor;

        if to_word > 99 {
            word = format!("{} hundred", ONES_TEENS[(to_word / 100) as usize]);

            if let Some(tens) = num_to_word(&mut (to_word % 100), 0) {
                word = format!("{word} {tens}");
            }
        } else if to_word > 19 {
            word = TENS[(to_word / 10) as usize].to_string();

            if let Some(ones) = num_to_word(&mut (to_word % 10), 0) {
                word = format!("{word}-{ones}");
            }
        } else {
            word = ONES_TEENS[to_word as usize].to_string();
        }

        word += match exp {
            3 => " thousand",
            6 => " million",
            9 => " billion",
            12 => " trillion",
            15 => " quadrillion",
            18 => " quintillion",
            _ => "",
        };

        Some(word)
    }
}

const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];
#[rustfmt::skip]
const ONES_TEENS: [&str; 20] = [
    "zero","one","two","three","four",
    "five","six","seven","eight","nine",
    "ten","eleven","twelve","thirteen","fourteen",
    "fifteen","sixteen","seventeen","eighteen","nineteen",
];
