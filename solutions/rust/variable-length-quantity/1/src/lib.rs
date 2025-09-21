#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result = Vec::<u8>::new();
    for num in values.into_iter() {
        result.extend_from_slice(&num_to_bytes(*num));
    }

    result
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
        .iter()
        .fold(Vec::<Vec<u8>>::new(), |mut acc, byte| {
            if let Some(prev_num) = acc.last_mut() {
                if let Some(num) = prev_num.last() {
                    if num & 0x80 == 0x80 {
                        prev_num.push(*byte);
                    } else {
                        acc.push(vec![*byte]);
                    }
                }
            } else {
                acc.push(vec![*byte]);
            }
            acc
        })
        .into_iter()
        .try_fold(Vec::<u32>::new(), |acc, number_bytes| {
            let Ok(number) = bytes_to_num(&number_bytes) else {
                return Err(Error::IncompleteNumber);
            };

            Ok([acc, vec![number]].concat())
        })
}

fn bytes_to_num(bytes: &Vec<u8>) -> Result<u32, Error> {
    if bytes.is_empty() || bytes.len() > 5 {
        return Err(Error::IncompleteNumber);
    } else if let Some(num) = bytes.last() {
        if *num > 0x7f {
            return Err(Error::IncompleteNumber);
        }
    }

    Ok(bytes
        .iter()
        .map(|n| *n as u32)
        .fold(0_u32, |num, byte| (num << 7) | (byte & 0x7f)))
}
