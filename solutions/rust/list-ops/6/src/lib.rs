/// Yields each item of a and then each item of b
pub fn append<I, J>(mut frst: I, mut scnd: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    std::iter::from_fn(move || {
        if let Some(val) = frst.next() {
            return Some(val);
        }
        if let Some(val) = scnd.next() {
            return Some(val);
        }

        None
    })
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(mut parent_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    let mut child_iter: Option<I::Item> = None;

    std::iter::from_fn(move || {
        loop {
            if let Some(child_it) = child_iter.as_mut() {
                if let Some(child_val) = child_it.next() {
                    return Some(child_val);
                } else {
                    child_iter = parent_iter.next();
                }
            } else if let Some(next_child_it) = parent_iter.next() {
                child_iter = Some(next_child_it);
            } else {
                break;
            }
        }

        None
    })
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(mut iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    std::iter::from_fn(move || {
        while let Some(item) = iter.next() {
            if predicate(&item) {
                return Some(item);
            }
        }
        None
    })
}

pub fn length<I: Iterator>(iter: I) -> usize {
    let mut count: usize = 0;

    for _ in iter {
        count += 1;
    }

    count
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(mut iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    std::iter::from_fn(move || {
        if let Some(val) = iter.next() {
            return Some(function(val));
        }

        None
    })
}

pub fn foldl<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    let mut acc = initial;
    for val in iter {
        acc = function(acc, val);
    }

    acc
}

pub fn foldr<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    let mut acc = initial;
    while let Some(val) = iter.next_back() {
        acc = function(acc, val);
    }

    acc
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(mut iter: I) -> impl Iterator<Item = I::Item> {
    std::iter::from_fn(move || {
        if let Some(val) = iter.next_back() {
            return Some(val);
        }

        None
    })
}
