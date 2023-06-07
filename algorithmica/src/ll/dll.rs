type Link<Item> = Option<std::ptr::NonNull<Node<Item>>>;
pub struct Node<Item> {
    prev: Link<Item>,
    value: Item,
    next: Link<Item>,
}

pub struct LinkedList<Item> {
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

pub struct Iter<'a, Item> {
    head: Link<Item>,
    tail: Link<Item>,
    len: usize,
    _foo: std::marker::PhantomData<&'a Item>,
}

pub struct IntoIter<Item> {
    list: LinkedList<Item>,
}

impl<Item> LinkedList<Item> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push_front(&mut self, value: Item) {
        // SAFETY: Box never return null pointer
        let new_node = unsafe { std::ptr::NonNull::new_unchecked(Box::into_raw(Node::new(value))) };
        match self.head {
            // SAFETY: head will never point to null pointer
            Some(old_node) => unsafe {
                (*old_node.as_ptr()).prev = Some(new_node);
                (*new_node.as_ptr()).next = Some(old_node);
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
        match self.tail {
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
            // SAFETY: We are unboxing a valid pointer and only we are accessing it
            // So it is safe to drop this pointer
            let boxed_node = Box::from_raw(current_head.as_ptr());
            self.head = boxed_node.next;
            match self.head {
                Some(new_head) => {
                    (*new_head.as_ptr()).prev = None;
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
            // SAFETY: We are unboxing a valid pointer and only we are accessing it
            // So it is safe to drop this pointer
            let boxed_node = Box::from_raw(current_tail.as_ptr());
            self.tail = boxed_node.prev;
            match self.tail {
                Some(new_tail) => {
                    (*new_tail.as_ptr()).next = None;
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

    pub fn into_iter(self) -> IntoIter<Item> {
        IntoIter { list: self }
    }
}

impl<Item> Default for LinkedList<Item> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }
}

impl<Item> Drop for LinkedList<Item> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

impl<Item: Clone> Clone for LinkedList<Item> {
    fn clone(&self) -> Self {
        let mut dll = LinkedList::<Item>::new();
        for item in self.into_iter() {
            dll.push_back(item.clone());
        }
        dll
    }
}

impl<Item> Extend<Item> for LinkedList<Item> {
    fn extend<T: IntoIterator<Item = Item>>(&mut self, iter: T) {
        for item in iter {
            self.push_back(item);
        }
    }
}

impl<Item> std::iter::FromIterator<Item> for LinkedList<Item> {
    fn from_iter<T: IntoIterator<Item = Item>>(iter: T) -> Self {
        let mut dll = LinkedList::new();
        dll.extend(iter);
        dll
    }
}

impl<Item: std::fmt::Debug> std::fmt::Debug for LinkedList<Item> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
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

impl<'a, Item> IntoIterator for &'a LinkedList<Item> {
    type Item = &'a Item;
    type IntoIter = Iter<'a, Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<Item> IntoIterator for LinkedList<Item> {
    type Item = Item;
    type IntoIter = IntoIter<Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<Item> Iterator for IntoIter<Item> {
    type Item = Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len, Some(self.list.len))
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn push_front() {
        let mut dll = super::LinkedList::new();
        dll.push_front("first-element");
        dll.push_front("second-element");
        dll.push_front("third-element");
        // println!("{:?}", dll);
        assert_eq!(dll.len(), 3);
    }

    #[test]
    fn basic_front() {
        let mut list = super::LinkedList::new();
        // Try to break an empty list
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Try to break a one item list
        list.push_front(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Mess around
        list.push_front(10);
        assert_eq!(list.len(), 1);
        list.push_front(20);
        assert_eq!(list.len(), 2);
        list.push_front(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.len(), 2);
        list.push_front(40);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(40));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn push_back() {
        let mut dll = super::LinkedList::new();
        dll.push_back("first-element");
        dll.push_back("second-element");
        dll.push_back("third-element");
        assert_eq!(dll.len(), 3);
    }

    #[test]
    fn pop_front() {
        let mut dll = super::LinkedList::new();
        dll.push_back("first-element");
        dll.push_back("second-element");
        dll.push_back("third-element");
        assert_eq!(dll.pop_front(), Some("first-element"));
        assert_eq!(dll.pop_front(), Some("second-element"));
        assert_eq!(dll.pop_front(), Some("third-element"));
        assert_eq!(dll.pop_front(), None);
    }

    #[test]
    fn pop_back() {
        let mut dll = super::LinkedList::new();
        dll.push_front("first-element");
        dll.push_front("second-element");
        dll.push_front("third-element");
        assert_eq!(dll.pop_back(), Some("first-element"));
        assert_eq!(dll.pop_back(), Some("second-element"));
        assert_eq!(dll.pop_back(), Some("third-element"));
        assert_eq!(dll.pop_back(), None);
    }

    #[test]
    fn clone() {
        let mut dll = super::LinkedList::new();
        dll.push_front("first-element");
        dll.push_front("second-element");
        dll.push_front("third-element");
        let mut cloned_ll = dll.clone();
        assert_eq!(cloned_ll.pop_back(), Some("first-element"));
        assert_eq!(cloned_ll.pop_back(), Some("second-element"));
        assert_eq!(cloned_ll.pop_back(), Some("third-element"));
        assert_eq!(cloned_ll.pop_back(), None);
    }

    #[test]
    fn front() {
        let mut dll = super::LinkedList::new();
        dll.push_front("first-element");
        dll.push_front("second-element");
        dll.push_front("third-element");
        let front_item = dll.front();
        assert_eq!(front_item, Some(&"third-element"));
    }

    #[test]
    fn front_mut() {
        let mut dll = super::LinkedList::new();
        dll.push_front("first-element");
        dll.push_front("second-element");
        dll.push_front("third-element");
        if let Some(e) = dll.front_mut() {
            *e = "changed-third-element";
        };

        assert_eq!(dll.front(), Some(&"changed-third-element"));
    }
    #[test]
    fn back() {
        let mut dll = super::LinkedList::new();
        dll.push_back("first-element");
        dll.push_back("second-element");
        dll.push_back("third-element");
        let back_item = dll.back();
        assert_eq!(back_item, Some(&"third-element"));
    }

    #[test]
    fn back_mut() {
        let mut dll = super::LinkedList::new();
        dll.push_front("first-element");
        dll.push_front("second-element");
        dll.push_front("third-element");
        if let Some(e) = dll.front_mut() {
            *e = "changed-first-element";
        };
        assert_eq!(dll.front(), Some(&"changed-first-element"));
    }

    #[test]
    fn extend() {
        let mut dll = super::LinkedList::new();
        dll.push_back("first-element");

        let mut dll2 = super::LinkedList::new();
        dll2.push_back("second-element");
        dll2.push_back("third-element");
        dll.extend(dll2);
        let mut it = dll.iter();
        assert_eq!(dll.front(), Some(&"first-element"));
        assert_eq!(it.next(), Some(&"first-element"));
        assert_eq!(it.next(), Some(&"second-element"));
        assert_eq!(it.next(), Some(&"third-element"));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn from_iterator() {
        use std::iter::FromIterator;
        let mut dll = super::LinkedList::new();
        dll.push_back("first-element");
        dll.push_back("second-element");
        dll.push_back("third-element");

        let dll2 = super::LinkedList::from_iter(dll.into_iter());
        let mut it = dll2.iter();
        assert_eq!(it.next(), Some(&"first-element"));
        assert_eq!(it.next(), Some(&"second-element"));
        assert_eq!(it.next(), Some(&"third-element"));
        assert_eq!(it.next(), None);
    }
}
