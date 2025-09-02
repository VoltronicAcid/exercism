pub struct Matcher<T> {
    matcher: fn(T) -> bool,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<S: ToString>(matcher: fn(T) -> bool, subs: S) -> Matcher<T> {
        Matcher {
            matcher,
            subs: subs.to_string(),
        }
    }
}

pub struct Fizzy<T: ToString + Clone> {
    matchers: Vec<Matcher<T>>,
}

impl<T: ToString + Clone> Fizzy<T> {
    pub fn new() -> Self {
        Fizzy {
            matchers: Vec::new(),
        }
    }

    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);

        self
    }

    pub fn apply<I: Iterator<Item = T>>(self, iter: I) -> impl Iterator<Item = String> {
        iter.map(move |val| {
            let s = self
                .matchers
                .iter()
                .filter(|&matcher| (matcher.matcher)(val.clone()))
                .map(|matcher| matcher.subs.clone())
                .collect::<Vec<String>>()
                .join("");

            match s.len() {
                0 => val.to_string(),
                _ => s.clone(),
            }
        })
    }
}

pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: From<u8> + std::ops::Rem<Output = T> + PartialEq + Default + ToString + Clone,
{
    Fizzy::new()
        .add_matcher(Matcher::new(
            |t: T| t.rem(T::from(3)) == T::default(),
            "fizz",
        ))
        .add_matcher(Matcher::new(
            |t: T| t.rem(T::from(5)) == T::default(),
            "buzz",
        ))
}
