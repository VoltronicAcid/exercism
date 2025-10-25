pub fn is_valid(code: &str) -> bool {
    let map: [usize; 10] = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];

    code.chars()
        .rev()
        .try_fold((0_u32, 0_usize), |(count, sum), c| match c {
            c if c.is_ascii_digit() => {
                let d = c.to_digit(10).unwrap() as usize;

                Some((count + 1, sum + if count % 2 == 1 { map[d] } else { d }))
            }
            c if !c.is_ascii_whitespace() => None,
            _ => Some((count, sum)),
        })
        .is_some_and(|(count, sum)| count > 1 && sum.is_multiple_of(10))
}
