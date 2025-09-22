const ACTIONS: [&str; 4] = ["wink", "double blink", "close your eyes", "jump"];
const REVERSE_FLAG: u8 = 0b1_0000;

pub fn actions(flags: u8) -> Vec<&'static str> {
    if flags & REVERSE_FLAG == REVERSE_FLAG {
        get_actions((0..ACTIONS.len()).rev(), flags)
    } else {
        get_actions(0..ACTIONS.len(), flags)
    }
}

fn get_actions<T>(iter: T, flags: u8) -> Vec<&'static str>
where
    T: Iterator<Item = usize>,
{
    iter.filter_map(|idx| {
        if flags & (1 << idx) > 0 {
            Some(ACTIONS[idx])
        } else {
            None
        }
    })
    .collect()
}
