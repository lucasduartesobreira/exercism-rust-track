use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    // Delete this field
    // dummy is needed to avoid unused parameter error during compilation
    //dummy: ::std::marker::PhantomData<T>,
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut next = self.head.as_ref();
        while next.is_some() {
            count += 1;
            next = next.unwrap().next.as_ref();
        }
        count
    }

    pub fn push(&mut self, _element: T) {
        let mut new_node = Node::new(_element);
        let last_head = self.head.take();

        new_node.next = last_head;
        self.head = Some(Box::from(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        let next = self.head.take()?;

        self.head = next.next;

        Some(next.data)
    }

    pub fn peek(&self) -> Option<&T> {
        let head = self.head.as_ref()?;
        Some(&head.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        if self.head.is_none() {
            return Self { head: None };
        }

        let mut current = self.head;
        let mut previous: Option<Box<Node<T>>> = None;
        let mut next: Option<Box<Node<T>>> = None;

        while let Some(mut current_unwraped) = current {
            next = current_unwraped.next;

            current_unwraped.next = previous;

            previous = Some(current_unwraped);

            current = next;
        }

        Self { head: previous }
    }
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node { data, next: None }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(i: I) -> Self {
        let i = i.into_iter();

        let mut result = SimpleLinkedList::new();

        for t in i {
            result.push(t);
        }

        result
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut result = vec![];
        let mut node = self;

        while let Some(data) = node.pop() {
            result.insert(0, data);
        }

        result
    }
}
