type OptNodeLink<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: OptNodeLink<T>,
}

pub struct SimpleLinkedList<T> {
    head: OptNodeLink<T>,
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
        self.head = Some(Box::new(Node {
            value,
            next: self.head.take()
        }));

        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.length -= 1;
            
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
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
