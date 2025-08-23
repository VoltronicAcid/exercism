#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let (rows, nums_per_row) = validate_input(input)?;

    let digits = input
        .lines()
        .flat_map(|s| s.chars())
        .collect::<Vec<char>>()
        .chunks(3)
        .enumerate()
        .fold(
            vec![vec![String::new(); nums_per_row]; rows],
            |mut acc, (idx, chunk)| {
                acc[idx / (4 * nums_per_row)][idx % nums_per_row] +=
                    &chunk.iter().collect::<String>();

                acc
            },
        )
        .iter()
        .map(|r| r.iter().map(ocr_to_digit).collect::<Vec<&str>>().join(""))
        .collect::<Vec<String>>();

    Ok(digits.join(","))
}

fn validate_input(input: &str) -> Result<(usize, usize), Error> {
    let rows = input.lines().count();
    if rows % 4 != 0 {
        return Err(Error::InvalidRowCount(rows));
    }

    if let Some(invalid_len) = input
        .lines()
        .map(|line| line.len())
        .find(|len| len % 3 != 0)
    {
        Err(Error::InvalidColumnCount(invalid_len))
    } else {
        let columns = input.lines().last().unwrap().len();

        Ok((rows / 4, columns / 3))
    }
}

fn ocr_to_digit(ocr: &String) -> &str {
    match ocr.as_str() {
        " _ | ||_|   " => "0",
        "     |  |   " => "1",
        " _  _||_    " => "2",
        " _  _| _|   " => "3",
        "   |_|  |   " => "4",
        " _ |_  _|   " => "5",
        " _ |_ |_|   " => "6",
        " _   |  |   " => "7",
        " _ |_||_|   " => "8",
        " _ |_| _|   " => "9",
        _ => "?",
    }
}
