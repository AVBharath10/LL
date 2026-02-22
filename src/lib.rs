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
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.head.as_deref(),
        }
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
    pub fn pop_back(&mut self) -> Option<T> {
        match self.head.as_mut() {
            None => None,
            Some(node) if node.next.is_none() => {
                let val = self.head.take().unwrap();
                Some(val.value)
            }
            Some(_) => {
                let mut curr = self.head.as_mut().unwrap();
                while curr.next.as_ref().unwrap().next.is_some() {
                    curr = curr.next.as_mut().unwrap();
                }
                let last = curr.next.take().unwrap();
                Some(last.value)
            }
        }
    }
}
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut curr = self.head.take();
        while let Some(mut boxed_node) = curr {
            curr = boxed_node.next.take();
        }
    }
}
