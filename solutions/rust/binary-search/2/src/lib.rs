pub fn find<T, R>(array: R, target: T) -> Option<usize>
where
    T: Ord,
    R: AsRef<[T]>,
{
    use std::cmp::Ordering::*;

    let values = array.as_ref();
    let mut lo: usize = 0;
    let mut hi: usize = values.len();

    while lo < hi {
        let mid: usize = lo + (hi - lo) / 2;

        match target.cmp(&values[mid]) {
            Equal => return Some(mid),
            Less => hi = mid,
            Greater => lo = mid + 1,
        }
    }

    None
}
