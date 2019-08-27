pub struct List {
    head: Link,
}

enum Link {
    Empty,
    Elem(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}