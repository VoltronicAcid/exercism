/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const M: i32 = 26;

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    use AffineCipherError::NotCoprime;

    (gcd(a, M) == 1)
        .then(|| {
            let enc_chars = plaintext
                .to_ascii_lowercase()
                .chars()
                .filter_map(|c| {
                    if c.is_ascii_alphabetic() {
                        char::from_u32(
                            (a as u32 * (c as u32 - 'a' as u32) + b as u32).rem_euclid(M as u32)
                                + 'a' as u32,
                        )
                    } else if c.is_ascii_digit() {
                        Some(c)
                    } else {
                        None
                    }
                })
                .collect::<Vec<char>>();

            if enc_chars.len() <= 5 {
                return enc_chars.iter().collect::<String>();
            }

            let map_win = enc_chars
                .chunks(5)
                .map(|window| window.iter().collect::<String>())
                .collect::<Vec<String>>();

            map_win.join(" ")
        })
        .ok_or(NotCoprime(a))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    use AffineCipherError::NotCoprime;

    (gcd(a, M) == 1)
        .then(|| {
            ciphertext
                .chars()
                .filter_map(|c| {
                    if c.is_ascii_whitespace() {
                        return None;
                    }

                    if c.is_ascii_digit() {
                        return Some(c);
                    }

                    match (0..M).find(|&num| (a * num) % M == 1) {
                        Some(mmi) => char::from_u32(
                            ((mmi * (c as i32 - 'a' as i32 - b) + M).rem_euclid(M) + 'a' as i32)
                                as u32,
                        ),
                        _ => None,
                    }
                })
                .collect::<String>()
        })
        .ok_or(NotCoprime(a))
}

fn gcd(num1: i32, num2: i32) -> i32 {
    if num2 == 0 {
        num1
    } else if num1 > num2 {
        gcd(num2, num1 % num2)
    } else {
        gcd(num2, num1)
    }
}
