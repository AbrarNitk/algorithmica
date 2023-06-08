type Link<Key, Value> = Option<std::ptr::NonNull<Node<Key, Value>>>;

struct Node<Key, Value> {
    key: Key,
    value: Value,
    next: Link<Key, Value>,
    prev: Link<Key, Value>,
}

impl<Key, Value> Node<Key, Value> {
    fn new(key: Key, value: Value) -> Box<Self> {
        Box::new(Node {
            key,
            value,
            next: None,
            prev: None,
        })
    }
}

struct DoublyLinkedList<Key, Value> {
    head: Link<Key, Value>,
    tail: Link<Key, Value>,
    len: usize,
}

impl<Key, Value> DoublyLinkedList<Key, Value> {
    fn new() -> Self {
        Default::default()
    }

    fn push_front(&mut self, key: Key, value: Value) -> std::ptr::NonNull<Node<Key, Value>> {
        // SAFETY: We know Box never returns the null pointer
        let node =
            unsafe { std::ptr::NonNull::new_unchecked(Box::into_raw(Node::new(key, value))) };
        if let Some(old_head) = self.head {
            unsafe {
                (*old_head.as_ptr()).prev = Some(node);
                (*node.as_ptr()).next = Some(old_head);
            }
        } else {
            self.tail = Some(node);
        }
        self.head = Some(node);
        self.len += 1;
        node
    }

    fn pop_back(&mut self) -> Option<(Key, Value)> {
        self.tail.map(|old_tail| {
            // SAFETY: We know that `old_tail` is never going to be None
            let boxed_node = unsafe { Box::from_raw(old_tail.as_ptr()) };
            self.tail = boxed_node.prev;
            if let Some(tail) = self.tail {
                unsafe {
                    (*tail.as_ptr()).next = None;
                }
            } else {
                self.head = None;
            }
            self.len -= 1;
            (boxed_node.key, boxed_node.value)
        })
    }

    fn move_to_front(&mut self, node_ptr: std::ptr::NonNull<Node<Key, Value>>) {
        let mut node = unsafe { &mut (*node_ptr.as_ptr()) };
        if node.prev.is_none() {
            return;
        }
        if node.next.is_none() {
            self.tail = node.prev;
            if let Some(new_tail) = self.tail {
                unsafe {
                    (*new_tail.as_ptr()).next = None;
                }
            }
        } else {
            unsafe {
                if let Some(node_prev) = node.prev {
                    (*node_prev.as_ptr()).next = node.next;
                }

                if let Some(node_next) = node.next {
                    (*node_next.as_ptr()).prev = node.prev;
                };
            }
        }
        node.prev = None;
        if let Some(old_head) = self.head {
            unsafe {
                (*old_head.as_ptr()).prev = Some(node_ptr);
                node.next = Some(old_head);
                self.head = Some(node_ptr);
            }
        };
    }

    fn remove(&mut self, node_ptr: std::ptr::NonNull<Node<Key, Value>>) -> (Key, Value) {
        // SAFETY: std::ptr will always exists and can't be null
        let boxed_node = unsafe { Box::from_raw(node_ptr.as_ptr()) };
        // handling if node is head node
        if boxed_node.prev.is_none() {
            self.head = boxed_node.next;
            if let Some(new_head) = self.head {
                unsafe {
                    (*new_head.as_ptr()).prev = None;
                }
            } else {
                self.tail = None;
            }
            self.len -= 1;
            return (boxed_node.key, boxed_node.value);
        }

        // handling if node is a tail node
        if boxed_node.next.is_none() {
            self.tail = boxed_node.prev;
            if let Some(new_tail) = self.tail {
                unsafe {
                    (*new_tail.as_ptr()).next = None;
                }
            } else {
                self.head = None;
            }
            self.len -= 1;
            return (boxed_node.key, boxed_node.value);
        }

        // handling if node is in between
        unsafe {
            if let Some(prev_node) = boxed_node.prev {
                (*prev_node.as_ptr()).next = boxed_node.next;
            };
            if let Some(next_node) = boxed_node.next {
                (*next_node.as_ptr()).prev = boxed_node.prev;
            };
        }
        self.len -= 1;
        (boxed_node.key, boxed_node.value)
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<Key, Value> Default for DoublyLinkedList<Key, Value> {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }
}

struct KeyRef<K> {
    key: *const K,
}

impl<K: std::hash::Hash> std::hash::Hash for KeyRef<K> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        unsafe { (*self.key).hash(state) }
    }
}

impl<K: PartialEq> PartialEq for KeyRef<K> {
    fn eq(&self, other: &KeyRef<K>) -> bool {
        unsafe { (*self.key).eq(&*other.key) }
    }
}

impl<K: Eq> Eq for KeyRef<K> {}
pub struct Lru<Key, Value> {
    map: std::collections::HashMap<KeyRef<Key>, std::ptr::NonNull<Node<Key, Value>>>,
    dll: DoublyLinkedList<Key, Value>,
    cap: usize,
}

impl<Key, Value> Lru<Key, Value> {
    pub fn new(cap: usize) -> Self {
        Lru {
            map: std::collections::HashMap::new(),
            dll: DoublyLinkedList::new(),
            cap,
        }
    }

    pub fn push(&mut self, key: Key, value: Value)
    where
        Key: Eq + std::hash::Hash + std::fmt::Debug,
    {
        if self.map.contains_key(&KeyRef { key: &key }) {
            if let Some(node) = self.map.remove(&KeyRef { key: &key }) {
                let (_key, _) = self.dll.remove(node);
            }
        }

        if self.dll.len() >= self.cap {
            if let Some((key, _v)) = self.dll.pop_back() {
                self.map.remove_entry(&KeyRef { key: &key });
            };
        }
        let key_ref = KeyRef { key: &key };
        let node = self.dll.push_front(key, value);
        self.map.insert(key_ref, node);
    }

    pub fn get(&mut self, key: &Key) -> Option<&Value>
    where
        Key: Eq + std::hash::Hash,
    {
        match self.map.get(&KeyRef { key }) {
            Some(node) => {
                self.dll.move_to_front(*node);
                Some(unsafe { &(*node.as_ptr()).value })
            }
            None => None,
        }
    }

    pub fn size(&self) -> usize {
        assert_eq!(self.map.keys().len(), self.dll.len());
        self.dll.len()
    }

    pub fn is_empty(&self) -> bool {
        if self.dll.is_empty() {
            assert!(self.map.is_empty());
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn base_test() {
        let lru = super::Lru::<String, i32>::new(3);
        assert_eq!(lru.size(), 0);
        assert!(lru.is_empty());
    }

    #[test]
    fn base_push() {
        let mut lru = super::Lru::<String, i32>::new(3);
        lru.push("A".to_string(), 1);
        lru.push("B".to_string(), 2);
        lru.push("C".to_string(), 3);
        assert_eq!(lru.size(), 3);
        lru.push("D".to_string(), 4);
        assert_eq!(lru.size(), 3);
    }

    #[test]
    fn push_same_nodes() {
        let mut lru = super::Lru::<String, i32>::new(3);
        lru.push("A".to_string(), 1);
        lru.push("A".to_string(), 10);
        lru.push("A".to_string(), 20);
        lru.push("A".to_string(), 30);
        assert_eq!(Some(&30), lru.get(&"A".to_string()));
        assert_eq!(None, lru.get(&"B".to_string()));
    }

    #[test]
    fn push_same_nodes_2() {
        let mut lru = super::Lru::<String, i32>::new(3);
        lru.push("A".to_string(), 1);
        lru.push("B".to_string(), 2);
        lru.push("A".to_string(), 10);
        lru.push("C".to_string(), 3);
        assert_eq!(Some(&10), lru.get(&"A".to_string()));
        assert_eq!(Some(&3), lru.get(&"C".to_string()));
        assert_eq!(Some(&2), lru.get(&"B".to_string()));
        assert_eq!(lru.size(), 3);
        lru.push("D".to_string(), 5);
        assert_eq!(lru.size(), 3);
    }
}
