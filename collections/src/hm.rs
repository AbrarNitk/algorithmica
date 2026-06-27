use std::hash::{DefaultHasher, Hash, Hasher};

pub struct HashMap<Key, Value> {
    table: Vec<Vec<(Key, Value)>>,
    size: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        let mut c = Vec::with_capacity(16);
        c.resize_with(16, || vec![]);
        Self { table: c, size: 0 }
    }

    pub fn table_row(&self, key: &K) -> Option<usize> {
        if self.table.is_empty() {
            return None;
        }

        let mut hasher = DefaultHasher::default();
        key.hash(&mut hasher);
        Some((hasher.finish() % (self.table.len() as u64)) as usize)
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.size > 3 * self.table.len() / 4 {
            self.resize();
        }

        let row_index = self.table_row(&key);
        match self.table.get_mut(row_index.unwrap()) {
            Some(v) => {
                let mut index_to_update = None;
                for (i, (k, _)) in v.iter().enumerate() {
                    if k.eq(&key) {
                        index_to_update = Some(i);
                        break;
                    }
                }
                if let Some(index) = index_to_update {
                    v.insert(index, (key, value));
                } else {
                    v.push((key, value));
                }

                self.size += 1;
            }
            None => {
                panic!("this is not expected, because all the array elements are assigned")
            }
        }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let row_index = self.table_row(&key);

        self.table
            .get(row_index.unwrap())
            .and_then(|container| container.into_iter().find(|(k, _)| k.eq(&key)))
            .map(|(_, v)| v)
    }

    pub fn contains_key(&self, key: K) -> bool {
        self.get(key).is_some()
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        let row_index = self.table_row(&key).unwrap();
        let mut matched_entry_index = None;
        for (i, (k, _)) in self.table[row_index].iter().enumerate() {
            if k.eq(&key) {
                matched_entry_index = Some(i);
            }
        }

        match matched_entry_index {
            Some(index) => Some(self.table[row_index].remove(index).1),
            None => None,
        }
    }

    // resize to the double, if the capacity is 75% full
    fn resize(&mut self) {
        let table_len = self.table.len();
        let new_len = 2 * table_len;

        let mut new_table = Vec::with_capacity(new_len);
        new_table.resize_with(new_len, || vec![]);

        for (k, v) in self.table.iter_mut().flat_map(|row| row.drain(..)) {
            let mut hasher = DefaultHasher::default();
            k.hash(&mut hasher);
            let bucket = (hasher.finish() % new_len as u64) as usize;
            new_table[bucket].push((k, v));
        }
        let _ = std::mem::replace(&mut self.table, new_table);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_test() {
        let mut hm = super::HashMap::new();

        let key_iter = 1..100;
        let value_iter = 1..100;

        for (k, v) in std::iter::zip(key_iter, value_iter) {
            hm.insert(k, v);
            assert_eq!(Some(&v), hm.get(k));
        }
    }

    #[test]
    fn test_with_str() {
        let map = super::HashMap::<&str, &str>::new();
        assert_eq!(map.contains_key("foo"), false);
        assert_eq!(map.get("foo"), None);
    }

    #[test]
    fn test_with_remove_without_value() {
        let mut map = super::HashMap::<&str, &str>::new();
        assert_eq!(map.contains_key("foo"), false);
        assert_eq!(map.get("foo"), None);
        assert_eq!(map.remove("foo"), None);
    }

    #[test]
    fn test_with_remove() {
        let mut map = super::HashMap::<&str, &str>::new();
        assert_eq!(map.contains_key("foo"), false);
        assert_eq!(map.get("foo"), None);
        assert_eq!(map.remove("foo"), None);

        map.insert("key", "value");
        assert_eq!(map.remove("key"), Some("value"));
    }
}
