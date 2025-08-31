pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = get_digits(num);

    num == digits
        .iter()
        .fold(0, |acc, &n| acc + n.pow(digits.len() as u32))
}

fn get_digits(mut num: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();

    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }

    digits
}
