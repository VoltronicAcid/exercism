#[derive(Debug, Eq)]
pub struct CustomSet<T> {
    data: Vec<T>,
}

impl<T> CustomSet<T>
where
    T: PartialEq + Clone,
{
    pub fn new(input: &[T]) -> Self {
        let mut data: Vec<T> = Vec::new();

        for val in input {
            if !data.contains(val) {
                data.push(val.clone());
            }
        }

        CustomSet { data }
    }

    pub fn contains(&self, elem: &T) -> bool {
        self.data.contains(elem)
    }

    pub fn add(&mut self, element: T) {
        if !self.data.contains(&element) {
            self.data.push(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.data.iter().all(|elem| other.contains(elem))
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.data.iter().all(|elem| !other.contains(elem))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        CustomSet::new(
            &self
                .data
                .iter()
                .filter(|&elem| other.contains(elem))
                .cloned()
                .collect::<Vec<T>>(),
        )
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        CustomSet::new(
            &self
                .data
                .iter()
                .filter(|&elem| !other.contains(elem))
                .cloned()
                .collect::<Vec<T>>(),
        )
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        CustomSet::new(
            &self
                .data
                .iter()
                .chain(other.data.iter())
                .cloned()
                .collect::<Vec<T>>(),
        )
    }
}

impl<T> PartialEq for CustomSet<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data.len() == other.data.len()
            && self.data.iter().all(|elem| other.data.contains(elem))
            && other.data.iter().all(|elem| self.data.contains(elem))
    }
}
