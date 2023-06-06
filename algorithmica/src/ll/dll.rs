type Link<Item> = Option<std::ptr::NonNull<Node<Item>>>;
pub struct Node<Item> {
    prev: Link<Item>,
    value: Item,
    next: Link<Item>,
}

pub struct LinkList<Item> {
    head: Link<Item>,
    tail: Link<Item>,
    len: usize,
}

impl<Item> Node<Item> {
    fn new(value: Item) -> Box<Self> {
        Box::new(Node {
            prev: None,
            value,
            next: None,
        })
    }
}

impl<Item> LinkList<Item> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push_front(&mut self, value: Item) {
        // SAFETY: Box never return null pointer
        let new_node = unsafe { std::ptr::NonNull::new_unchecked(Box::into_raw(Node::new(value))) };
        match self.head.take() {
            // SAFETY: head will never point to null pointer
            Some(old_node) => unsafe {
                (*new_node.as_ptr()).next = Some(old_node);
                (*old_node.as_ptr()).prev = Some(new_node);
            },
            None => {
                self.tail = Some(new_node);
            }
        }
        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn push_back(&mut self, value: Item) {
        // SAFETY: Box never return null pointer
        let new_node = unsafe { std::ptr::NonNull::new_unchecked(Box::into_raw(Node::new(value))) };
        match self.tail.take() {
            // SAFETY: tail will never point to null pointer
            Some(old_node) => unsafe {
                (*old_node.as_ptr()).next = Some(new_node);
                (*new_node.as_ptr()).prev = Some(old_node);
            },
            None => {
                self.head = Some(new_node);
            }
        }
        self.tail = Some(new_node);
        self.len += 1;
    }
}

impl<Item> Default for LinkList<Item> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn push_front() {}
}
