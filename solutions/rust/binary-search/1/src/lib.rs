pub fn find(array: &[i32], target: i32) -> Option<usize> {
    let mut lo: usize = 0;
    let mut hi: usize = array.len();

    while lo < hi {
        let mid: usize = lo + (hi - lo) / 2;

        match array[mid] {
            _ if array[mid] < target => lo = mid + 1,
            _ if array[mid] > target => {
                if mid == 0 {
                    break;
                } else {
                    hi = mid - 1;
                }
            }
            _ => return Some(mid),
        }
    }

    if let Some(val) = array.get(lo) {
        if *val == target {
            return Some(lo);
        }

        None
    } else {
        None
    }
}
