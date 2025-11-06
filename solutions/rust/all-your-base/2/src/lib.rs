#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], input_base: u32, output_base: u32) -> Result<Vec<u32>, Error> {
    if output_base < 2 {
        return Err(Error::InvalidOutputBase);
    } else if input_base < 2 {
        return Err(Error::InvalidInputBase);
    }

    let mut base_10 = number.iter().try_fold(0, |total, &digit| {
        (digit < input_base)
            .then_some(total * input_base + digit)
            .ok_or(Error::InvalidDigit(digit))
    })?;
    let mut digits: Vec<u32> = Vec::new();

    loop {
        digits.insert(0, base_10 % output_base);
        base_10 /= output_base;

        if base_10 == 0 { break; }
    }

    Ok(digits)
}
