pub fn is_armstrong_number(num: u32) -> bool {
    num == 0 || {
        let digits: Vec<u32> = get_digits(num);

        let power = digits.len() as u32;
        num == digits.iter().fold(0, |acc, &x| acc + x.pow(power))
    }
}

fn get_digits(mut num: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();
    // let mut n = num;

    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }

    digits
}
