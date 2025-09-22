#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().fold(Vec::<u8>::new(), |acc, num| {
        [acc, num_to_bytes(*num)].concat()
    })
}

fn num_to_bytes(num: u32) -> Vec<u8> {
    (0_u32..32)
        .step_by(7)
        .filter_map(|shift| match shift {
            0 => Some((num & 0x7f) as u8),
            _ => match num >> shift {
                0 => None,
                _ => Some((num >> shift | 0x80) as u8),
            },
        })
        .rev()
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    bytes
        .split_inclusive(|byte| *byte < 0x80)
        .try_fold(Vec::<u32>::new(), |mut result, slice| {
            let num = bytes_to_num(slice)?;
            result.push(num);

            Ok(result)
        })
}

fn bytes_to_num(bytes: &[u8]) -> Result<u32, Error> {
    use Error::IncompleteNumber;

    match bytes.last() {
        Some(num) if *num > 0x7f => Err(IncompleteNumber),
        None => Err(IncompleteNumber),
        _ => Ok(bytes
            .iter()
            .fold(0_u32, |num, byte| (num << 7) | (*byte as u32 & 0x7f))),
    }
}
