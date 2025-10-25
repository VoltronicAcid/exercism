pub fn is_valid(code: &str) -> bool {
    let map: [u32; 10] = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];
    let mut sum = 0;
    let mut count = 0;

    for c in code.chars().rev() {
        match c.to_digit(10) {
            Some(d) => {
                count += 1;
                sum += if count % 2 == 0 { map[d as usize] } else { d };
            }
            None => {
                if c.is_ascii_whitespace() {
                    continue;
                } else {
                    return false;
                }
            }
        }
    }

    count > 1 && sum.is_multiple_of(10)
}
