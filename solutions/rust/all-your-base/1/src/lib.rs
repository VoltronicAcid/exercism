#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    use Error::*;

    if from_base < 2 {
        return Err(InvalidInputBase);
    }

    if let Some(invalid_digit) = number.iter().find(|&n| *n >= from_base) {
        return Err(InvalidDigit(*invalid_digit));
    };

    if to_base < 2 {
        return Err(InvalidOutputBase);
    }

    if number.is_empty() {
        return Ok(vec![0]);
    }

    let mut value: u32 = number
        .iter()
        .rev()
        .enumerate()
        .map(|(exp, &num)| num * from_base.pow(exp as u32))
        .sum::<u32>();

    let mut len: usize = 0;
    while to_base.pow(len as u32) <= value {
        len += 1;
    }

    if len == 0 {
        return Ok(vec![0]);
    }

    let mut rslt: Vec<u32> = Vec::new();
    for exp in (0..(len as u32)).rev() {
        rslt.push(value / to_base.pow(exp));
        value %= to_base.pow(exp);
    }

    Ok(rslt)
}
