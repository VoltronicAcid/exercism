#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let (rows, cols) = validate_input(input)?;

    let nums_per_row = cols / 3;

    let digits = input
        .chars()
        .filter(|ch| *ch != '\n')
        .collect::<Vec<char>>()
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .enumerate()
        .fold(
            vec![vec![String::new(); nums_per_row]; rows / 4],
            |mut acc, (idx, chunk)| {
                acc[idx / (4 * nums_per_row)][idx % nums_per_row] += chunk.as_str();

                acc
            },
        )
        .iter()
        .map(|r| r.iter().map(ocr_to_digit).collect::<String>())
        .collect::<Vec<String>>()
        .join(",");

    Ok(digits)
}

fn validate_input(input: &str) -> Result<(usize, usize), Error> {
    let rows = input.chars().filter(|ch| *ch == '\n').count() + 1;
    if rows % 4 != 0 {
        return Err(Error::InvalidRowCount(rows));
    }

    let columns;
    if let Some(width) = input.chars().position(|ch| ch == '\n') {
        match width % 3 {
            1 | 2 => return Err(Error::InvalidColumnCount(width)),
            _ => columns = width,
        }
    } else {
        return Err(Error::InvalidColumnCount(input.len()));
    }

    Ok((rows, columns))
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
