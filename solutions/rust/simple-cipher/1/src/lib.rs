use rand::Rng;

const A: u32 = 'a' as u32;

pub fn encode(key: &str, mesg: &str) -> Option<String> {
    if key.is_empty() || key.chars().any(|c| !c.is_ascii_lowercase()) {
        return None;
    }

    Some(
        mesg.chars()
            .zip(key.chars().cycle())
            .filter_map(|(m, k)| char::from_u32((m as u32 - A + (k as u32 - A)) % 26 + A))
            .collect::<String>(),
    )
}

pub fn decode(key: &str, mesg: &str) -> Option<String> {
    if key.is_empty() || key.chars().any(|c| !c.is_ascii_lowercase()) {
        return None;
    }

    Some(
        mesg.chars()
            .zip(key.chars().cycle())
            .filter_map(|(m, k)| char::from_u32(((26 + m as u32 - A) - (k as u32 - A)) % 26 + A))
            .collect::<String>(),
    )
}

pub fn encode_random(mesg: &str) -> (String, String) {
    let mut rng = rand::rng();

    let key = (0..200)
        .map(|_| rng.random_range('a'..='z'))
        .collect::<String>();

    (key.clone(), encode(&key, mesg).unwrap())
}
