pub fn square(s: u32) -> u64 {
    let base: u64 = 2;
    let exp: u32 = s - 1;

    base.pow(exp)
}

pub fn total() -> u64 {
    u64::MAX
}
