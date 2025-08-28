#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }

    string_digits
        .chars()
        .map(|c| char::to_digit(c, 10).ok_or(Error::InvalidDigit(c)))
        .collect::<Result<Vec<u32>, Error>>()?
        .windows(span)
        .map(|window| window.iter().product::<u32>() as u64)
        .max()
        .ok_or(Error::SpanTooLong)
}
