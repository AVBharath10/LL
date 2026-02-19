pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

pub struct Iter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(node) => {
                self.current = node.next.as_deref();
                Some(&node.value)
            }
            None => None,
        }
    }
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value: value,
            next: None,
        }
    }
}
impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }
    pub fn push_front(&mut self, value: T) {
        let old_node = self.head.take();
        let new_node = Box::new(Node {
            value,
            next: old_node,
        });
        self.head = Some(new_node);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;

            node.value
        })
    }
    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }
    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
    }
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut current = self.head.as_ref();
        while current.is_some() {
            let node = current.unwrap();
            len += 1;
            current = node.next.as_ref();
        }
        len
    }
}
