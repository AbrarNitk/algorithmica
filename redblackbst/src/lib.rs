use std::cmp::{Ord, Ordering};

#[derive(PartialEq)]
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
        if let Some(root) = self.root.as_mut() {
            root.color = Color::BLACK;
        }
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
                if is_red(&leaf.left) && !is_red(&leaf.right) {
                    leaf = rotate_left(leaf);
                }
                if is_red(&leaf.left) && is_red(&leaf.left.as_ref().as_ref().unwrap().left) {
                    leaf = rotate_right(leaf);
                }
                if is_red(&leaf.left) && is_red(&leaf.right) {
                    leaf = flip_colors(leaf);
                }
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
    h.color = Color::RED;
    x.left = Box::new(Some(h));
    x
}

fn rotate_right<Key: Ord, Value>(mut h: Node<Key, Value>) -> Node<Key, Value> {
    let mut x = h.left.take().unwrap();
    h.left = x.right;
    x.color = h.color;
    h.color = Color::RED;
    x.right = Box::new(Some(h));
    x
}

fn flip_colors<Key: Ord, Value>(mut h: Node<Key, Value>) -> Node<Key, Value> {
    if let Some(ref mut left) = h.left.as_mut() {
        left.color = Color::BLACK;
    }

    if let Some(right) = h.right.as_mut() {
        right.color = Color::BLACK;
    }
    h.color = Color::RED;
    h
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
