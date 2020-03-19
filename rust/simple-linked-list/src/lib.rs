use std::iter::FromIterator;

type Link<T> = Option<Box<Node<T>>>;

pub struct SimpleLinkedList<T> {
    head: Link<T>,
}

struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn len(&self) -> usize {
        let mut count: usize = 0;
        let mut node = &(self.head);
        while node.is_some() {
            count += 1;
            node = &(node.as_ref().unwrap().next);
        }
        count
    }

    pub fn push(&mut self, _element: T) {
        let node = Box::new(Node {
            data: _element,
            next: self.head.take(),
        });
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.data)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head {
            None => None,
            Some(ref node) => Some(&node.data),
        }
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut rev_ll = Self::new();
        while self.head.is_some() {
            rev_ll.push(self.pop().unwrap());
        }
        rev_ll
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut ll = SimpleLinkedList::new();
        for elem in _iter {
            ll.push(elem);
        }
        ll
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
    fn into(mut self) -> Vec<T> {
        let mut vec = Vec::new();
        while self.head.is_some() {
            vec.insert(0, self.pop().unwrap());
        }
        vec
    }
}
