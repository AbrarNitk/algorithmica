// Bag is a simple data structure that only support the adding
// elements an iterating through them
// It does not support the remove operation

type Link<Item> = Option<Box<Node<Item>>>;

struct Node<Item> {
    item: Item,
    next: Link<Item>,
}

pub struct Bag<Item> {
    head: Link<Item>,
    n: usize,
}

pub struct Iter<'a, Item> {
    current: Option<&'a Node<Item>>,
}

pub struct IterMut<'a, Item> {
    current: Option<&'a mut Node<Item>>,
}

impl<Item> Default for Bag<Item> {
    fn default() -> Self {
        Self { head: None, n: 0 }
    }
}

impl<Item> Bag<Item> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add(&mut self, item: Item) {
        self.n += 1;
        self.head = Some(Box::new(Node {
            item,
            next: self.head.take(),
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

    pub fn iter_mut(&mut self) -> IterMut<Item> {
        IterMut {
            current: self.head.as_deref_mut(),
        }
    }
}

impl<'a, Item> Iterator for Iter<'a, Item> {
    type Item = &'a Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            let item = &node.item;
            self.current = node.next.as_deref();
            item
        })
    }
}

impl<'a, Item> Iterator for IterMut<'a, Item> {
    type Item = &'a mut Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|x| {
            let item = &mut x.item;
            self.current = x.next.as_deref_mut();
            item
        })
    }
}

#[cfg(test)]
mod bag_tests {
    #[test]
    fn test_1() {
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
