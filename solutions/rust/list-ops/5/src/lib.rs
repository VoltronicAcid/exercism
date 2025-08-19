#![allow(dead_code)]
/// Yields each item of a and then each item of b
pub fn append<I, J>(frst: I, scnd: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    Appender { frst, scnd }
}

struct Appender<I, J> {
    frst: I,
    scnd: J,
}

impl<I, J> Iterator for Appender<I, J>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(val) = self.frst.next() {
            return Some(val);
        }

        if let Some(val) = self.scnd.next() {
            return Some(val);
        }

        None
    }
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(parent_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    Concatter {
        parent_iter,
        child_iter: None,
    }
}

struct Concatter<I>
where
    I: Iterator,
    I::Item: Iterator,
{
    parent_iter: I,
    child_iter: Option<I::Item>,
}

impl<I> Iterator for Concatter<I>
where
    I: Iterator,
    I::Item: Iterator,
{
    type Item = <I::Item as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(child_it) = self.child_iter.as_mut() {
                if let Some(child_val) = child_it.next() {
                    return Some(child_val);
                } else {
                    self.child_iter = self.parent_iter.next();
                }
            } else if let Some(next_child_it) = self.parent_iter.next() {
                self.child_iter = Some(next_child_it);
            } else {
                break;
            }
        }

        None
    }
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    Filterer { iter, predicate }
}

struct Filterer<I, F>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    iter: I,
    predicate: F,
}

impl<I, F> Iterator for Filterer<I, F>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        for val in self.iter.by_ref() {
            if (self.predicate)(&val) {
                return Some(val);
            }
        }

        None
    }
}

pub fn length<I: Iterator>(iter: I) -> usize {
    let mut lengther = Lengther::new(iter);

    lengther.count()
}

struct Lengther<I>
where
    I: Iterator,
{
    iter: I,
    count: usize,
}

impl<I> Lengther<I>
where
    I: Iterator,
{
    pub fn new(iter: I) -> Self {
        Lengther { count: 0, iter }
    }

    pub fn count(&mut self) -> usize {
        for _ in self.iter.by_ref() {
            self.count += 1;
        }

        self.count
    }
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    Mapper { iter, function }
}

struct Mapper<I, F, U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    iter: I,
    function: F,
}

impl<I, F, U> Iterator for Mapper<I, F, U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(val) = self.iter.next() {
            return Some((self.function)(val));
        }

        None
    }
}

pub fn foldl<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    let mut acc = initial;
    for val in iter.by_ref() {
        acc = (function)(acc, val);
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
        acc = (function)(acc, val);
    }

    acc
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(iter: I) -> impl Iterator<Item = I::Item> {
    Reverser { iter }
}

struct Reverser<I>
where
    I: DoubleEndedIterator,
{
    iter: I,
}

impl<I> Iterator for Reverser<I>
where
    I: DoubleEndedIterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(val) = self.iter.next_back() {
            return Some(val);
        }

        None
    }
}
