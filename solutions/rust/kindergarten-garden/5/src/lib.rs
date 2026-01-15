pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let start = (student.chars().next().unwrap() as usize - 'A' as usize) * 2;
    let end = start + 2;

    diagram
        .lines()
        .flat_map(|line| {
            line[start..end]
                .chars()
                .map(|c| match c {
                    'G' => "grass",
                    'C' => "clover",
                    'R' => "radishes",
                    'V' => "violets",
                    _ => "",
                })
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<&str>>()
}
