/// Yields each item of a and then each item of b
pub fn append<I, J>(a: I, b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    // std::iter::from_fn(|| todo!())
    a.chain(b)
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    // std::iter::from_fn(|| todo!())
    nested_iter.flatten()
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(it: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    // std::iter::from_fn(|| todo!())
    it.filter(predicate)
}

pub fn length<I: Iterator>(iter: I) -> usize {
    iter.count()
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    // std::iter::from_fn(|| todo!())
    iter.map(function)
}

pub fn foldl<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    iter.fold(initial, function)
}

pub fn foldr<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    iter.rev().fold(initial, function)
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(iter: I) -> impl Iterator<Item = I::Item> {
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    iter.rev()
}
