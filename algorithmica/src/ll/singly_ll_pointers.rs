type Link<Item> = *mut Node<Item>;
struct Node<Item> {
    value: Item,
    next: Link<Item>,
}

pub struct List<Item> {
    head: Link<Item>,
}

pub struct Iter<'a, Item> {
    current: Option<&'a Node<Item>>,
}

pub struct IterMut<'a, Item> {
    current: Option<&'a mut Node<Item>>,
}

impl<Item> List<Item> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push_front(&mut self, value: Item) {
        let new_node = Box::into_raw(Box::new(Node {
            value,
            next: std::ptr::null_mut(),
        }));

        if self.head.is_null() {
            self.head = new_node;
        } else {
            unsafe {
                let node = &mut *new_node;
                node.next = self.head;
                self.head = node;
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<Item> {
        if self.head.is_null() {
            None
        } else {
            unsafe {
                let box_node = Box::from_raw(self.head);
                self.head = box_node.next;
                Some(box_node.value)
            }
        }
    }

    pub fn peek(&self) -> Option<&Item> {
        unsafe { self.head.as_ref().map(|x| &x.value) }
    }

    pub fn peek_mut(&mut self) -> Option<&mut Item> {
        unsafe { self.head.as_mut().map(|x| &mut x.value) }
    }

    pub fn iter<'a>(&self) -> Iter<'a, Item> {
        Iter {
            current: unsafe { self.head.as_ref() },
        }
    }

    pub fn iter_mut<'a>(&mut self) -> IterMut<'a, Item> {
        IterMut {
            current: unsafe { self.head.as_mut() },
        }
    }
}

impl<Item> Drop for List<Item> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<Item> Default for List<Item> {
    fn default() -> Self {
        Self {
            head: std::ptr::null_mut(),
        }
    }
}

impl<'a, Item> Iterator for Iter<'a, Item> {
    type Item = &'a Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|x| {
            self.current = unsafe { x.next.as_ref() };
            &x.value
        })
    }
}

impl<'a, Item> Iterator for IterMut<'a, Item> {
    type Item = &'a mut Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|x| {
            self.current = unsafe { x.next.as_mut() };
            &mut x.value
        })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn push() {
        let mut ll = super::List::new();
        ll.push_front(1);
        ll.push_front(2);
    }

    #[test]
    fn peek_1() {
        let ll = super::List::<i32>::new();
        assert_eq!(ll.peek(), None)
    }

    #[test]
    fn peek_2() {
        let mut ll = super::List::<i32>::new();
        ll.push_front(1);
        assert_eq!(ll.peek(), Some(&1))
    }

    #[test]
    fn pop() {
        let mut ll = super::List::new();
        ll.push_front(1);
        ll.push_front(2);
        assert_eq!(ll.pop_front(), Some(2));
        assert_eq!(ll.pop_front(), Some(1));
    }

    #[test]
    fn iter() {
        let mut ll = super::List::new();
        ll.push_front(1);
        ll.push_front(2);
        let mut iter = ll.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut ll = super::List::new();
        ll.push_front(1);
        ll.push_front(2);
        for x in ll.iter_mut() {
            *x += 1;
        }
        let mut iter = ll.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }
}
