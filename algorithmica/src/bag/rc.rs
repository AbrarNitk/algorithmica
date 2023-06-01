type Link<Item> = Option<std::rc::Rc<Node<Item>>>;
pub struct Node<Item> {
    value: Item,
    next: Link<Item>,
}

pub struct Bag<Item> {
    head: Link<Item>,
    n: usize,
}

impl<Item> Default for Bag<Item> {
    fn default() -> Self {
        Self { head: None, n: 0 }
    }
}

pub struct Iter<'a, Item> {
    current: Option<&'a Node<Item>>,
}

impl<Item> Bag<Item> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, value: Item) {
        self.n += 1;
        self.head = Some(std::rc::Rc::new(Node {
            value,
            next: self.head.clone(),
        }))
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn iter(&self) -> Iter<Item> {
        Iter {
            current: self.head.as_deref(),
        }
    }
}

impl<'a, Item> Iterator for Iter<'a, Item> {
    type Item = &'a Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|x| {
            let item = &x.value;
            self.current = x.next.as_deref();
            item
        })
    }
}

impl<Item> Drop for Bag<Item> {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(node) = cur {
            if let Ok(mut node) = std::rc::Rc::try_unwrap(node) {
                cur = node.next.take()
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod bag_tests {
    #[test]
    fn test_2() {
        let mut bag = super::Bag::new();
        bag.add(5);
        bag.add(10);
        bag.add(20);
        let mut bag_iter = bag.iter();
        assert_eq!(bag_iter.next(), Some(&20));
        assert_eq!(bag_iter.next(), Some(&10));
        assert_eq!(bag_iter.next(), Some(&5));
        assert_eq!(bag_iter.next(), None);
        assert_eq!(bag.size(), 3)
    }
}
