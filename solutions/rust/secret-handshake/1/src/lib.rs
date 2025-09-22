const ACTIONS: [&str; 4] = ["wink", "double blink", "close your eyes", "jump"];

pub fn actions(flags: u8) -> Vec<&'static str> {
    let mut results = (0_u8..4).fold(Vec::<&str>::new(), |mut acc, shift| {
        if flags & (1 << shift) > 0 {
            acc.push(ACTIONS[shift as usize]);
        }

        acc
    });

    if flags > 15 {
        results.reverse();
    }

    results
}
