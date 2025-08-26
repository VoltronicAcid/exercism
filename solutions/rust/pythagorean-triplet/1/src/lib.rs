use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (0..=(sum / 2))
        .filter_map(|a| {
            let numer = (2 * a.pow(2)) + sum.pow(2) - (2 * sum * a);
            let denom = 2 * (sum - a);

            if denom != 0 && numer % denom == 0 {
                let c = numer / denom;
                let b = sum - c - a;

                if a < b && b < c {
                    return Some([a, b, c]);
                }
            }

            None
        })
        .collect::<HashSet<[u32; 3]>>()
}
