pub fn is_armstrong_number(num: u32) -> bool {
    num == 0 || {
        let mut digits: Vec<u32> = Vec::new();
        let mut n = num;

        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }

        let power = digits.len() as u32;
        // let sum = digits.iter().fold(0, |acc, &x| acc + x.pow(power));

        // sum == num
        num == digits.iter().fold(0, |acc, &x| acc + x.pow(power))
    }
}
