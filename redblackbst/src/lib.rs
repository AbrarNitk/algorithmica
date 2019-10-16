use std::cmp::{Ord, Ordering};

#[derive(PartialEq, Copy, Clone)]
pub enum Color {
    RED,
    BLACK,
}

pub struct Node<Key: Ord, Value> {
    key: Key,
    value: Value,
    color: Color,
    left: Box<Option<Node<Key, Value>>>,
    right: Box<Option<Node<Key, Value>>>,
}

impl<Key: Ord, Value> Node<Key, Value> {
    pub fn new(key: Key, value: Value, color: Color) -> Self {
        Node {
            key,
            value,
            color,
            left: Box::new(None),
            right: Box::new(None),
        }
    }
}

pub struct RedBlackBST<Key: Ord, Value> {
    root: Option<Node<Key, Value>>,
    size: u64,
}

impl<Key: Ord, Value> RedBlackBST<Key, Value> {
    pub fn size(&self) -> u64 {
        return self.size;
    }

    pub fn put(&mut self, key: Key, value: Value) {
        self.root = Self::insert(self.root.take(), key, value);
    }

    fn insert(node: Option<Node<Key, Value>>, key: Key, value: Value) -> Option<Node<Key, Value>> {
        match node {
            None => Some(Node::new(key, value, Color::RED)),
            Some(mut leaf) => {
                let mut leaf = match leaf.key.cmp(&key) {
                    Ordering::Less => {
                        leaf.left = Box::new(Self::insert(leaf.left.take(), key, value));
                        leaf
                    }
                    Ordering::Greater => {
                        leaf.right = Box::new(Self::insert(leaf.right.take(), key, value));
                        leaf
                    }
                    Ordering::Equal => {
                        leaf.value = value;
                        leaf
                    }
                };

                Some(leaf)
            }
        }
    }
}

fn is_red<Key: Ord, Value>(node: &Option<Node<Key, Value>>) -> bool {
    match node {
        None => false,
        Some(Node { color, .. }) => color == &Color::RED,
    }
}

fn rotate_left<Key: Ord, Value>(mut h: Node<Key, Value>) -> Node<Key, Value> {
    let mut x = h.right.take().unwrap();
    h.right = x.left;
    x.color = h.color;
    x.left = Box::new(Some(h));
    h.color = Color::RED;
    x
}

fn rotate_right<Key: Ord, Value>(mut h: Node<Key, Value>) -> Node<Key, Value> {
    let mut x = h.left.take().unwrap();
    h.left = x.right;
    x.color = h.color;
    x.right = Box::new(Some(h));
    h.color = Color::RED;
    x
}

//impl<Key: Ord, Value> PartialEq for Node<Key, Value> {
//    fn eq(&self, other: &Node<Key, Value>) -> bool {
//        self == other
//    }
//}
//
//impl<Key: Ord, Value> Ord for Node<Key, Value> {
//    fn cmp(&self, other: &Node<Key, Value>) -> Ordering {
//        Ordering::Equal
//    }
//}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
