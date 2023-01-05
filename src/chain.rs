use std::mem::replace;

use crate::{hash, HashTable};

#[derive(Debug, Clone)]
pub struct ChainTable {
    store: Vec<Vec<(String, i64)>>,
    size: usize,
}

impl ChainTable {
    pub fn new() -> Self {
        Self {
            store: vec![vec![]; 7],
            size: 0,
        }
    }

    fn get_store_index(&self, key: &str) -> usize {
        hash(key) % self.store.len()
    }

    fn get_indice(&self, key: &str) -> Option<(usize, usize)> {
        let store_index = self.get_store_index(key);
        self.store[store_index]
            .iter()
            .position(|(k, _)| k == key)
            .map(|minor_index| (store_index, minor_index))
    }
}

impl Default for ChainTable {
    fn default() -> Self {
        Self::new()
    }
}

impl HashTable for ChainTable {
    fn insert(&mut self, key: String, value: i64) -> Option<i64> {
        if let Some(old_value) = self.get_mut(&key) {
            Some(replace(old_value, value))
        } else {
            let index = self.get_store_index(&key);
            self.store[index].push((key, value));
            self.size += 1;
            None
        }
    }

    fn remove(&mut self, key: String) -> Option<i64> {
        let (index, to_remove) = self.get_indice(&key)?;
        let (_, removed) = self.store[index].remove(to_remove);
        self.size -= 1;
        Some(removed)
    }

    fn get(&self, key: &str) -> Option<&i64> {
        let (store_index, minor_index) = self.get_indice(key)?;
        let (_, value) = self.store[store_index].get(minor_index).unwrap(); //minor_index must be valid
        Some(value)
    }

    fn get_mut(&mut self, key: &str) -> Option<&mut i64> {
        let (store_index, minor_index) = self.get_indice(key)?;
        let (_, value) = self.store[store_index].get_mut(minor_index).unwrap(); //minor_index must be valid
        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{self, BufRead},
    };

    use super::*;
    fn read_dict() -> Vec<(String, i64)> {
        let input = File::open("test_dict.txt").expect("Unable to open 'test_dict.txt'");
        io::BufReader::new(input)
            .lines()
            .map(|l| {
                let line = l.unwrap();
                let mut data = line.split(' ');
                let key = data.next().unwrap().to_string();
                let value = data.next().unwrap().parse::<i64>().unwrap();
                (key, value)
            })
            .collect()
    }

    #[test]
    fn test_chain_table() {
        let mut table = ChainTable::new();
        let data = read_dict();
        for (k, v) in data.clone() {
            assert!(table.insert(k, v).is_none());
        }
        assert_eq!(data.len(), table.size);
        let (exist_key, exist_value) = data.get(data.len() / 2).unwrap();
        assert_eq!(exist_value, table.get(exist_key).unwrap());
        let new_value = *exist_value + 105;
        let value = table.get_mut(exist_key).unwrap();
        *value = new_value;
        assert_eq!(&new_value, table.get(exist_key).unwrap());
        assert_eq!(&new_value, table.get_mut(exist_key).unwrap());
        let other_value = new_value + 22;
        assert_eq!(
            new_value,
            table.insert(exist_key.into(), other_value).unwrap()
        );
        assert_eq!(&other_value, table.get(exist_key).unwrap());
        assert_eq!(&other_value, table.get_mut(exist_key).unwrap());
        table.remove(exist_key.into());
        assert!(table.get(exist_key).is_none());
        assert!(table.get_mut(exist_key).is_none());
        assert_eq!(data.len() - 1, table.size);
        assert!(table.store.len() > table.size);
    }
}
