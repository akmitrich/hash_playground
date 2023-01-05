use std::mem::replace;

use crate::{hash, HashTable};

#[derive(Debug, Default, Clone)]
pub struct ChainTable {
    store: Vec<Vec<(String, i64)>>,
    size: usize,
}

impl ChainTable {
    pub fn new() -> Self {
        Self {
            store: vec![vec![]; 7],
            ..Default::default()
        }
    }

    fn get_index(&self, key: &String) -> usize {
        hash(key) % self.store.len()
    }
}

impl HashTable for ChainTable {
    fn insert(&mut self, key: String, value: i64) -> Option<i64> {
        if let Some(old_value) = self.get_mut(&key) {
            Some(replace(old_value, value))
        } else {
            let index = self.get_index(&key);
            self.store[index].push((key, value));
            self.size += 1;
            None
        }
    }

    fn remove(&mut self, key: String) -> Option<i64> {
        let index = self.get_index(&key);
        let to_remove = self.store[index].iter().position(|x| x.0 == key)?;
        self.size -= 1;
        Some(self.store[index].remove(to_remove).1)
    }

    fn get(&self, key: &String) -> Option<&i64> {
        let index = self.get_index(key);
        for (k, v) in self.store[index].iter() {
            if key == k {
                return Some(v);
            }
        }
        None
    }

    fn get_mut(&mut self, key: &String) -> Option<&mut i64> {
        let index = hash(key) % self.store.len();
        for (k, v) in self.store[index].iter_mut() {
            if key == k {
                return Some(v);
            }
        }
        None
    }
}
