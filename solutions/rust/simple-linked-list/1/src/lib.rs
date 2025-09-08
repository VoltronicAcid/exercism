struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Node<T>>,
    length: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None, length: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, value: T) {
        self.head = match self.head.take() {
            Some(prev_head) => Some(Node {value, next: Some(Box::new(prev_head))}),
            None => Some(Node {value, next: None}),
        };

        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next_node) = head.next {
                self.head = Some(*next_node);
            }

            self.length -= 1;
            head.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head.as_ref() {
            Some(head) =>  Some(&head.value),
            None => None,
        }
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut reversed = SimpleLinkedList::new();

        while let Some(value) = self.pop() {
            reversed.push(value);
        } 

        reversed
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().fold(SimpleLinkedList::new(), |mut list, value|{
            list.push(value);

            list
        })
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut list: SimpleLinkedList<T>) -> Vec<T> {
        let mut values = Vec::<T>::new();
        let index = 0_usize;

        while let Some(element) = list.pop() {
            values.insert(index, element);
        }

        values
    }
}
