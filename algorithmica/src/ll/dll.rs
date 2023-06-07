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

pub struct Iter<'a, Item> {
    head: Link<Item>,
    tail: Link<Item>,
    len: usize,
    _foo: std::marker::PhantomData<&'a Item>,
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

    pub fn pop_front(&mut self) -> Option<Item> {
        self.head.map(|current_head| unsafe {
            let boxed_node = Box::from_raw(current_head.as_ptr());
            match (*current_head.as_ptr()).next {
                Some(new_head) => {
                    (*new_head.as_ptr()).prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            }
            self.len -= 1;
            boxed_node.value
        })
    }

    pub fn pop_back(&mut self) -> Option<Item> {
        self.tail.map(|current_tail| unsafe {
            let boxed_node = Box::from_raw(current_tail.as_ptr());
            match (*current_tail.as_ptr()).prev {
                Some(new_tail) => {
                    (*new_tail.as_ptr()).next = None;
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head = None;
                }
            }
            self.len -= 1;
            boxed_node.value
        })
    }

    pub fn front(&self) -> Option<&Item> {
        self.head.as_ref().map(|x| unsafe { &(*x.as_ptr()).value })
    }

    pub fn back(&self) -> Option<&Item> {
        self.tail.map(|x| unsafe { &(*x.as_ptr()).value })
    }

    pub fn front_mut(&mut self) -> Option<&mut Item> {
        self.head.map(|x| unsafe { &mut (*x.as_ptr()).value })
    }

    pub fn back_mut(&mut self) -> Option<&mut Item> {
        self.tail.map(|x| unsafe { &mut (*x.as_ptr()).value })
    }

    pub fn clear(&mut self) {
        while let Some(_) = self.pop_back() {}
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter<Item> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len,
            _foo: std::marker::PhantomData,
        }
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

impl<Item> Drop for LinkList<Item> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

impl<Item: Clone> Clone for LinkList<Item> {
    fn clone(&self) -> Self {
        let mut dll = LinkList::<Item>::new();
        for item in self.into_iter() {
            dll.push_back(item.clone());
        }
        dll
    }
}

impl<'a, Item> Iterator for Iter<'a, Item> {
    type Item = &'a Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.head.map(|x| unsafe {
                self.len -= 1;
                self.head = (*x.as_ptr()).next;
                &(*x.as_ptr()).value
            })
        } else {
            None
        }
    }
}

impl<'a, Item> IntoIterator for &'a LinkList<Item> {
    type Item = &'a Item;
    type IntoIter = Iter<'a, Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn push_front() {}

    #[test]
    fn push_back() {}

    #[test]
    fn pop_front() {}

    #[test]
    fn pop_back() {}
}
