pub struct List {
    head: Link
}

impl List {
    pub fn new() -> List {
        List {
            head: None,
        }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut node = self.head.take();
        while let Some(mut next_node) = node {
            node = next_node.next.take();
        }
    }
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn verify_create() {
        let xs = List::new();
        assert!(xs.head.is_none());
    }

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