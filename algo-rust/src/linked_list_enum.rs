use std::mem;

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List {
            head: Link::Empty
        }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::Elem(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::Elem(nxt) => {
                self.head = nxt.next;
                Some(nxt.elem)
            }
        }
    }
}

enum Link {
    Empty,
    Elem(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

#[cfg(test)]
mod tests {
    use crate::linked_list_enum::{List, Node};

    #[test]
    fn verify_empty_pop() {
        let mut xs = List::new();
        assert_eq!(xs.pop(), None);
    }

    #[test]
    fn verify_single_pop() {
        let mut xs = List::new();
        xs.push(1);
        assert_eq!(xs.pop(), Some(1));
        assert_eq!(xs.pop(), None);
    }

    #[test]
    fn verify_multi_pop() {
        let mut xs = List::new();
        xs.push(1);
        xs.push(2);
        xs.push(3);
        assert_eq!(xs.pop(), Some(3));
        assert_eq!(xs.pop(), Some(2));
        assert_eq!(xs.pop(), Some(1));
        assert_eq!(xs.pop(), None);
    }
}