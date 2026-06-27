use std::hash::{DefaultHasher, Hash, Hasher};

pub struct HashMap<Key, Value> {
    table: Vec<Vec<(Key, Value)>>,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        let mut c = Vec::with_capacity(16);
        c.resize_with(16, || vec![]);
        Self { table: c }
    }

    pub fn table_row(&self, key: &K) -> Option<usize>
    where
        K: Hash + Eq,
    {
        if self.table.is_empty() {
            return None;
        }

        let mut hasher = DefaultHasher::default();
        key.hash(&mut hasher);
        Some((hasher.finish() % (self.table.len() as u64)) as usize)
    }

    pub fn insert(&mut self, key: K, value: V)
    where
        K: Hash + Eq,
    {
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
            }
            None => {
                panic!("this is not expected, because all the array elements are assigned")
            }
        }
    }

    pub fn get(&self, key: K) -> Option<&V>
    where
        K: Hash + Eq,
    {
        let row_index = self.table_row(&key);

        self.table
            .get(row_index.unwrap())
            .and_then(|container| container.into_iter().find(|(k, _)| k.eq(&key)))
            .map(|(_, v)| v)
    }

    pub fn contains_key(&self, key: K) -> bool
    where
        K: Hash + Eq,
    {
        self.get(key).is_some()
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
}
