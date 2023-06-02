use std::ops::Deref;

type Link<Item> = Option<std::rc::Rc<std::cell::RefCell<Node<Item>>>>;
pub struct Node<Item> {
    value: Item,
    prev: Link<Item>,
    next: Link<Item>,
}

pub struct List<Item> {
    head: Link<Item>,
    tail: Link<Item>,
}

impl<Item> Node<Item> {
    pub fn new(value: Item) -> std::rc::Rc<std::cell::RefCell<Self>> {
        std::rc::Rc::new(std::cell::RefCell::new(Self {
            value,
            prev: None,
            next: None,
        }))
    }
}

impl<Item> List<Item> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push_front(&mut self, elem: Item) {
        let new_node = Node::new(elem);
        match self.head.take() {
            Some(node) => {
                node.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(node);
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    pub fn push_back(&mut self, elem: Item) {
        let new_node = Node::new(elem);
        match self.tail.take() {
            Some(node) => {
                node.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(node);
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<Item> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            std::rc::Rc::try_unwrap(old_head)
                .ok()
                .unwrap()
                .into_inner()
                .value
        })
    }

    pub fn pop_back(&mut self) -> Option<Item> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            };

            std::rc::Rc::try_unwrap(old_tail)
                .ok()
                .unwrap()
                .into_inner()
                .value
        })
    }

    pub fn peek_front(&self) -> Option<std::cell::Ref<Item>> {
        self.head
            .as_ref()
            .map(|node| std::cell::Ref::map(node.borrow(), |node| &node.value))
    }

    pub fn peek_front_mut(&self) -> Option<std::cell::RefMut<Item>> {
        self.head
            .as_ref()
            .map(|node| std::cell::RefMut::map(node.borrow_mut(), |n| &mut n.value))
    }

    pub fn peek_back(&self) -> Option<std::cell::Ref<Item>> {
        self.tail
            .as_ref()
            .map(|node| std::cell::Ref::map(node.borrow(), |n| &n.value))
    }

    pub fn peek_back_mut(&self) -> Option<std::cell::RefMut<Item>> {
        self.tail
            .as_ref()
            .map(|node| std::cell::RefMut::map(node.borrow_mut(), |n| &mut n.value))
    }

    pub fn into_iter(self) -> IntoIter<Item> {
        IntoIter { list: self }
    }

    // It is difficult to implement this, let's move to the next task
    // pub fn iter(&self) -> Iter<'_, Item> {
    //     Iter {
    //         current: self.head.as_ref().map(|x| x.borrow()),
    //     }
    // }
}

impl<Item> Default for List<Item> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
}

impl<Item> Drop for List<Item> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {
            println!("Dropping List");
        }
    }
}

pub struct IntoIter<Item> {
    list: List<Item>,
}

impl<Item> Iterator for IntoIter<Item> {
    type Item = Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<Item> DoubleEndedIterator for IntoIter<Item> {
    fn next_back(&mut self) -> Option<Item> {
        self.list.pop_back()
    }
}

// It is difficult to implement this, so let' move to the next task
// pub struct Iter<'a, Item> {
//     current: Option<std::cell::Ref<'a, Node<Item>>>,
// }
//

// impl<'a, Item> Iterator for Iter<'a, Item> {
//     type Item = &'a Item;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.current.take().map(|x| &x.deref().value)
//     }
// }

#[cfg(test)]
pub mod tests {
    #[test]
    fn front_pop() {
        let mut ll = super::List::new();
        ll.push_front("AbrarK".to_string());
        ll.push_front("Khan".to_string());
        assert_eq!(ll.pop_front(), Some("Khan".to_string()));
        assert_eq!(ll.pop_front(), Some("AbrarK".to_string()));
    }

    #[test]
    fn front_peek() {
        let mut ll = super::List::new();
        ll.push_front("AbrarK".to_string());
        ll.push_front("Khan".to_string());
        assert_eq!(&*ll.peek_front().unwrap(), &"Khan".to_string());
        ll.pop_front();
        assert_eq!(&*ll.peek_front().unwrap(), &"AbrarK".to_string());
    }

    #[test]
    fn push_back() {
        let mut ll = super::List::new();
        ll.push_back("AbrarK".to_string());
        ll.push_back("Khan".to_string());
        assert_eq!(ll.pop_front(), Some("AbrarK".to_string()));
        assert_eq!(ll.pop_front(), Some("Khan".to_string()));
    }

    #[test]
    fn peek_back() {
        let mut ll = super::List::new();
        ll.push_front("AbrarK".to_string());
        ll.push_front("Khan".to_string());
        assert_eq!(&*ll.peek_back().unwrap(), &"AbrarK".to_string());
        ll.pop_back();
        assert_eq!(&*ll.peek_back().unwrap(), &"Khan".to_string());
    }

    #[test]
    fn into_iter() {
        let mut dll = super::List::new();
        // 10
        dll.push_front(10);
        // 10 -> 11
        dll.push_back(11);
        // 9 -> 10 -> 11
        dll.push_front(9);
        let mut iter = dll.into_iter();
        assert_eq!(iter.next(), Some(9));
        assert_eq!(iter.next_back(), Some(11));
        assert_eq!(iter.next(), Some(10));
    }
}
