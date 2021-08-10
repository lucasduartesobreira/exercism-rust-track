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
        //unimplemented!()
        /*
         *        let mut curr = &mut self.head;
         *        if curr.is_none() || curr.unwrap().next.is_none() {
         *            return self;
         *        }
         *
         *        let mut prev = &mut None;
         *        let mut next = &mut None;
         *        while curr.is_some() {
         *            next = &mut curr.unwrap().next;
         *            curr.unwrap().next = *prev;
         *            prev = curr;
         *            curr = next;
         *        }
         *
         *        SimpleLinkedList { head: *next }
         */
        /*
         *        let mut vec: Vec<T> = self.into();
         *
         *        if vec.is_empty() {
         *            return SimpleLinkedList { head: None };
         *        }
         *        vec.reverse();
         *        let mut head = Node::new(vec[0]);
         *        let mut next = None;
         *
         *        let mut iter = vec.iter();
         *        for i in vec.iter().advance_by(1) {
         *            next
         *        }
         */
    }
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node { data, next: None }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        unimplemented!()
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut result = vec![];
        let mut head = self.head;

        while head.is_some() {
            let temp_head = head.unwrap();
            result.push(temp_head.data);
            head = temp_head.next;
        }

        result
    }
}
