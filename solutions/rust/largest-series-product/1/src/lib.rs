#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if let Some(c) = string_digits.chars().find(|c| !c.is_numeric()) {
        return Err(Error::InvalidDigit(c));
    };

    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    } else if span == 0 {
        return Ok(1);
    }

    Ok(string_digits
        .chars()
        .map(|c| char::to_digit(c, 10).unwrap() as u64)
        .collect::<Vec<u64>>()
        .windows(span)
        .map(|window| window.iter().product::<u64>())
        .max()
        .unwrap())
}
