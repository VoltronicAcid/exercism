/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    use AffineCipherError::NotCoprime;

    (is_coprime_m(a))
        .then(|| {
            let enc_chars = plaintext
                .to_ascii_lowercase()
                .chars()
                .filter_map(|c| {
                    (c.is_ascii_alphanumeric()).then(|| {
                        if c.is_ascii_digit() {
                            c
                        } else {
                            (((a as u32 * (c as u32 - 97) + b as u32) % 26 + 97) as u8) as char
                        }
                    })
                })
                .collect::<Vec<char>>();

            if enc_chars.len() > 5 {
                let map_win = enc_chars
                    .chunks(5)
                    .map(|window| window.iter().collect::<String>())
                    .collect::<Vec<String>>();

                map_win.join(" ")
            } else {
                enc_chars.iter().collect::<String>()
            }
        })
        .ok_or(NotCoprime(a))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    use AffineCipherError::NotCoprime;

    (is_coprime_m(a))
        .then(|| {
            ciphertext
                .chars()
                .filter_map(|c| {
                    (!c.is_ascii_whitespace()).then(|| {
                        if c.is_ascii_digit() {
                            return c;
                        }

                        let c = c as i32;
                        let a_code = b'a' as i32;

                        let mmi = mod_inverse(a, 26);
                        let shift = c - a_code - b;

                        let char_code = ((mmi * shift + 26).rem_euclid(26)) + a_code;

                        (char_code as u8) as char
                    })
                })
                .collect::<String>()
        })
        .ok_or(NotCoprime(a))
}

fn is_coprime_m(num: i32) -> bool {
    gcd(num, 26) == 1
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

fn mod_inverse(a: i32, m: i32) -> i32 {
    loop {
        for mmi in 0..m {
            if (a * mmi) % m == 1 {
                return mmi;
            }
        }
    }
}
