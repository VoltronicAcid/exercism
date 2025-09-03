use std::collections::HashSet;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let row_maxes = input
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            let Some(&max) = row.iter().max() else {
                return Vec::new();
            };

            row.iter()
                .enumerate()
                .filter(|&(_, &val)| val == max)
                .map(|(col_idx, _)| (row_idx, col_idx))
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>();

    let row_len = (!input.is_empty()).then_some(input[0].len()).unwrap();
    let col_maxes = (0..row_len)
        .flat_map(|col_idx| {
            let cols = input.iter().map(|row| row[col_idx]).collect::<Vec<_>>();
            let Some(&min) = cols.iter().min() else {
                return Vec::new();
            };

            cols.iter()
                .enumerate()
                .filter(|&(_, &val)| val == min)
                .map(|(row_idx, _)| (row_idx, col_idx))
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>();

    row_maxes
        .intersection(&col_maxes)
        .map(|&(row, col)| (row, col))
        .collect::<Vec<(usize, usize)>>()
}
