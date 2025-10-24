#![allow(dead_code, unused_variables)]
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    use crate::Error::*;

    if input.is_empty() || !input.lines().count().is_multiple_of(4) {
        return Err(InvalidRowCount(input.lines().count()));
    }

    if let Some(line) = input
        .lines()
        .find(|&line| line.is_empty() || !line.len().is_multiple_of(3))
    {
        return Err(InvalidColumnCount(line.len()));
    };

    let result = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(4)
        .map(|line| {
            (0..line[0].len())
                .step_by(3)
                .map(|start| {
                    match line
                        .iter()
                        .map(|&line| &line[start..start + 3])
                        .collect::<String>()
                        .as_str()
                    {
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
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(",");

    Ok(result)
}
