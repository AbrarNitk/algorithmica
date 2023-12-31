const R: usize = 256;

pub enum Node<Value> {
    Empty,
    Value(NodeValue<Value>),
}

#[derive(Debug)]
pub struct NodeValue<Value> {
    value: Option<Value>,
    next: Vec<Node<Value>>,
}

#[derive(Debug)]
pub struct Trie<Value> {
    node: Node<Value>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Empty => Ok(()),
            Node::Value(v) => f.debug_struct("Node").field("value", v).finish(),
        }
    }
}

impl<V> Node<V> {
    fn new() -> Self {
        let mut next = Vec::with_capacity(R);
        for _ in 0..R {
            next.push(Node::Empty);
        }
        Self::Value(NodeValue { value: None, next })
    }

    fn set_value(&mut self, value: V) {
        match self {
            Node::Empty => panic!("operation not allowed"),
            Node::Value(ref mut v) => v.value = Some(value),
        }
    }

    fn insert(&mut self, chars: &[u8], value: V, mut index: usize) {
        match self {
            Node::Empty => {
                let mut node = Node::new();
                index += 1;
                if index == chars.len() {
                    node.set_value(value);
                    *self = node;
                    return;
                } else {
                    *self = node;
                    if let Node::Value(ref mut nv) = self {
                        nv.next[chars[index] as usize].insert(chars, value, index);
                    }
                }
            }
            Node::Value(ref mut nv) => {
                index += 1;
                if index == chars.len() {
                    nv.value = Some(value);
                    return;
                } else {
                    nv.next[chars[index] as usize].insert(chars, value, index);
                }
            }
        }
    }

    fn get(&self, chars: &[u8], mut index: usize) -> Option<&V> {
        match self {
            Node::Empty => None,
            Node::Value(v) => {
                index += 1;
                if index == chars.len() {
                    v.value.as_ref()
                } else {
                    v.next[chars[index] as usize].get(chars, index)
                }
            }
        }
    }

    fn delete(&mut self, chars: &[u8], mut index: usize) {
        match self {
            Node::Empty => (),
            Node::Value(ref mut nv) => {
                index += 1;
                if index == chars.len() {
                    nv.value = None;
                    return;
                } else {
                    nv.next[chars[index] as usize].delete(chars, index);
                }
            }
        }
    }
}

impl<V> Trie<V> {
    pub fn new() -> Self {
        Self { node: Node::new() }
    }

    pub fn insert<K: AsRef<str>>(&mut self, key: K, value: V) {
        let key_chars = key.as_ref().chars().map(|c| c as u8).collect::<Vec<_>>();
        if key_chars.is_empty() {
            return;
        }

        match self.node {
            Node::Empty => {
                panic!("should not be invoked")
            }
            Node::Value(ref mut nv) => {
                nv.next[key_chars[0] as usize].insert(key_chars.as_slice(), value, 0);
            }
        }
    }

    pub fn get<K: AsRef<str>>(&self, key: K) -> Option<&V> {
        let key_chars = key.as_ref().chars().map(|c| c as u8).collect::<Vec<_>>();
        if key_chars.is_empty() {
            return None;
        }

        match self.node {
            Node::Empty => None,
            Node::Value(ref nv) => nv.next[key_chars[0] as usize].get(key_chars.as_slice(), 0),
        }
    }

    pub fn delete<K: AsRef<str>>(&mut self, key: K) {
        let key_chars = key.as_ref().chars().map(|c| c as u8).collect::<Vec<_>>();
        if key_chars.is_empty() {
            return;
        }

        match self.node {
            Node::Empty => (),
            Node::Value(ref mut nv) => {
                nv.next[key_chars[0] as usize].delete(key_chars.as_slice(), 0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let mut root = Trie::new();
        root.insert("b", "1bvalue");
        root.insert("b", "2bvalue");
        root.insert("c", "1cvalue");
        root.insert("b", "3bvalue");
        root.insert("c", "2cvalue");
        assert_eq!(Some(&"3bvalue"), root.get("b"));
        assert_eq!(Some(&"2cvalue"), root.get("c"));
    }

    #[test]
    fn test_1() {
        let mut root = Trie::new();
        root.insert("abcdef", "d");
        root.insert("ab", "e");
        assert_eq!(Some(&"d"), root.get("abcdef"));
        assert_eq!(Some(&"e"), root.get("ab"));

        root.insert("abcdef", "f");
        root.insert("ab", "b");
        assert_eq!(Some(&"f"), root.get("abcdef"));
        assert_eq!(Some(&"b"), root.get("ab"));

        root.insert("abc", "c");
        root.insert("abd", "d");
        assert_eq!(Some(&"c"), root.get("abc"));
        assert_eq!(Some(&"d"), root.get("abd"));

        assert_eq!(Some(&"f"), root.get("abcdef"));
        assert_eq!(Some(&"b"), root.get("ab"));

        assert_eq!(None, root.get("abcde"));
        assert_eq!(None, root.get("a"));
    }

    #[test]
    fn test_2() {
        let mut root = Trie::new();
        root.insert("abc", "c");
        root.insert("abd", "d");
        assert_eq!(Some(&"c"), root.get("abc"));
        assert_eq!(Some(&"d"), root.get("abd"));
    }

    #[test]
    fn delete_test() {
        let mut root = Trie::new();

        root.insert("abc", "c");
        root.insert("abd", "d");

        assert_eq!(root.get("abc"), Some(&"c"));
        assert_eq!(root.get("abd"), Some(&"d"));
        assert_eq!(root.get("abd"), Some(&"d"));

        root.delete("abd");
        assert_eq!(root.get("abd"), None);
        assert_eq!(root.get("abc"), Some(&"c"));
        root.delete("abc");
        assert_eq!(root.get("abc"), None);
    }
}
