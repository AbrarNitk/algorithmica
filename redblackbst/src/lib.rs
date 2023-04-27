use std::cmp::{Ord, Ordering};

#[allow(clippy::upper_case_acronyms)]
#[derive(PartialEq, Debug)]
pub enum Color {
    RED,
    BLACK,
}

#[derive(Debug)]
struct Node<Key: Ord, Value> {
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

#[derive(Debug)]
pub struct RedBlackBST<Key: Ord, Value> {
    root: Option<Node<Key, Value>>,
    size: u64,
}

impl<Key: Ord, Value> Default for RedBlackBST<Key, Value> {
    fn default() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }
}

impl<Key: Ord, Value> RedBlackBST<Key, Value> {
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn put(&mut self, key: Key, value: Value) {
        self.root = Self::insert(self.root.take(), key, value);
        self.size += 1;
        if let Some(root) = self.root.as_mut() {
            root.color = Color::BLACK;
        }
    }

    fn insert(node: Option<Node<Key, Value>>, key: Key, value: Value) -> Option<Node<Key, Value>> {
        match node {
            None => Some(Node::new(key, value, Color::RED)),
            Some(mut leaf) => {
                let mut leaf = match leaf.key.cmp(&key) {
                    Ordering::Greater => {
                        leaf.left = Box::new(Self::insert(leaf.left.take(), key, value));
                        leaf
                    }
                    Ordering::Less => {
                        leaf.right = Box::new(Self::insert(leaf.right.take(), key, value));
                        leaf
                    }
                    Ordering::Equal => {
                        leaf.value = value;
                        leaf
                    }
                };
                if is_red(&leaf.right) && !is_red(&leaf.left) {
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

    pub fn get(&self, key: Key) -> Option<&Value> {
        Self::search(&self.root, key)
    }

    fn search(node: &Option<Node<Key, Value>>, key: Key) -> Option<&Value> {
        match node {
            None => None,
            Some(leaf) => match key.cmp(&leaf.key) {
                Ordering::Less => Self::search(&leaf.left, key),
                Ordering::Greater => Self::search(&leaf.right, key),
                Ordering::Equal => Some(&leaf.value),
            },
        }
    }

    pub fn contains(&self, key: Key) -> bool {
        Self::search(&self.root, key).is_some()
    }

    pub fn height(&self) -> u64 {
        RedBlackBST::bst_height(&self.root)
    }

    fn bst_height(node: &Option<Node<Key, Value>>) -> u64 {
        match node {
            Some(node) => {
                u64::max(
                    RedBlackBST::bst_height(&node.left),
                    RedBlackBST::bst_height(&node.right),
                ) + 1
            }
            None => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn min(&self) -> Option<&Key> {
        if self.is_empty() {
            return Option::<&Key>::None;
        }
        self.get_min(&self.root).map(|node| &node.key)
    }
    #[allow(clippy::only_used_in_recursion)]
    fn get_min<'a>(&self, node: &'a Option<Node<Key, Value>>) -> Option<&'a Node<Key, Value>> {
        node.as_ref().and_then(|data| {
            if data.left.is_some() {
                self.get_min(&data.left)
            } else {
                Some(data)
            }
        })
    }

    pub fn max(&self) -> Option<&Key> {
        if self.is_empty() {
            return Option::<&Key>::None;
        }
        self.get_max(&self.root).map(|node| &node.key)
    }

    #[allow(clippy::only_used_in_recursion)]
    fn get_max<'a>(&self, node: &'a Option<Node<Key, Value>>) -> Option<&'a Node<Key, Value>> {
        node.as_ref().and_then(|data| {
            if data.right.is_some() {
                self.get_max(&data.right)
            } else {
                Some(data)
            }
        })
    }
    //    pub fn floor(&self) -> Option<&Key> {}
    //    pub fn ceiling(&self) -> Option<&Key> {}
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
    fn init_test() {
        let mut bst = super::RedBlackBST::default();
        bst.put(3, "1");
        bst.put(2, "2");
        bst.put(1, "3");
        assert_eq!(bst.size(), 3);
        assert_eq!(bst.get(1), Some(&"3"));
        assert_eq!(bst.get(4), None);
    }
}
