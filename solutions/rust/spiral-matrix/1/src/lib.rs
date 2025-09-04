use std::cmp::Ordering::Equal;
use std::iter::repeat_n;

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut spiral = vec![vec![1_u32; size as usize]; size as usize];
    let deltas = [(0_i32, 1_i32), (1, 0), (0, -1), (-1, 0)].iter().cycle();
    let mut y = 0;
    let mut x = 0;
    let mut num = 1;

    (1..size)
        .flat_map(|n| match n.cmp(&(size - 1)) {
            Equal => repeat_n(n, 3),
            _ => repeat_n(n, 2),
        })
        .rev()
        .zip(deltas)
        .flat_map(|(count, dir)| repeat_n(dir, count as usize))
        .for_each(|&(dy, dx)| {
            y += dy;
            x += dx;
            num += 1;

            spiral[y as usize][x as usize] = num;
        });

    spiral
}
