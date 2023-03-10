use std::mem::{replace, take};

use crate::{hash, HashTable};

#[derive(Debug, Clone, PartialEq)]
pub struct OpenTable {
    store: Vec<OpenCell>,
    size: usize,
}

#[derive(Debug, Default, Clone, PartialEq)]
enum OpenCell {
    #[default]
    Empty,
    Taken(String, i64),
    Erased(String, i64),
}

impl OpenTable {
    pub fn new() -> Self {
        Self {
            store: vec![Default::default(); 7],
            size: 0,
        }
    }

    fn capacity(&self) -> usize {
        self.store.len()
    }

    fn probing(&self, hash: usize, probe: usize) -> usize {
        probing(hash, probe, self.capacity())
    }

    fn find_index(&self, key: &str) -> Option<usize> {
        let hash = hash(key);
        let mut probe = 0;
        loop {
            let index = self.probing(hash, probe);
            match self.store[index] {
                OpenCell::Empty => break,
                OpenCell::Taken(ref k, _) if k == key => {
                    return Some(index);
                }
                _ => probe += 1,
            }
        }
        None
    }

    fn need_to_resize(&self) -> bool {
        self.size >= self.capacity() / 2
    }

    fn resize(&mut self) {
        let mut new_table = Self {
            store: vec![Default::default(); self.capacity() * 2 + 1],
            size: 0,
        };
        for cell in take(&mut self.store) {
            match cell {
                OpenCell::Empty | OpenCell::Erased(_, _) => continue,
                OpenCell::Taken(k, v) => {
                    new_table.insert(k, v);
                }
            }
        }
        *self = new_table;
    }
}

impl Default for OpenTable {
    fn default() -> Self {
        Self::new()
    }
}

impl HashTable for OpenTable {
    fn insert(&mut self, key: String, value: i64) -> Option<i64> {
        if self.need_to_resize() {
            self.resize();
        }
        let hash = hash(&key);
        let mut probe = 0;
        let mut first_erased = None;
        let first_empty = loop {
            let index = self.probing(hash, probe);
            match self.store[index] {
                OpenCell::Empty => break index,
                OpenCell::Erased(_, _) => {
                    first_erased = first_erased.or(Some(index));
                    probe += 1;
                }
                OpenCell::Taken(ref k, ref mut v) if k == &key => {
                    return Some(replace(v, value));
                }
                _ => probe += 1,
            }
        };
        self.size += 1;
        let index = first_erased.unwrap_or(first_empty);
        self.store[index] = OpenCell::Taken(key, value);
        None
    }

    fn remove(&mut self, key: String) -> Option<i64> {
        let index = self.find_index(&key)?;
        if let OpenCell::Taken(key, value) = take(&mut self.store[index]) {
            self.store[index] = OpenCell::Erased(key, value);
            self.size -= 1;
            Some(value)
        } else {
            unreachable!()
        }
    }

    fn get(&self, key: &str) -> Option<&i64> {
        let index = self.find_index(key)?;
        if let OpenCell::Taken(_, value) = &self.store[index] {
            Some(value)
        } else {
            unreachable!()
        }
    }

    fn get_mut(&mut self, key: &str) -> Option<&mut i64> {
        fn lazy_deletion(store: &mut [OpenCell], key: &str) -> (Option<usize>, Option<usize>) {
            let mut erased = None;
            let hash = hash(key);
            let mut probe = 0;
            loop {
                let index = probing(hash, probe, store.len());
                match store[index] {
                    OpenCell::Empty => break (None, erased),
                    OpenCell::Taken(ref k, _) if k == key => break (Some(index), erased),
                    OpenCell::Taken(_, _) => probe += 1,
                    OpenCell::Erased(_, _) => {
                        erased = erased.or(Some(index));
                        probe += 1;
                    }
                }
            }
        }

        fn make_some_value_from_taken(taken: &mut OpenCell) -> Option<&mut i64> {
            if let OpenCell::Taken(_, value) = taken {
                Some(value)
            } else {
                unreachable!()
            }
        }

        match lazy_deletion(&mut self.store, key) {
            (None, _) => None,
            (Some(index), None) => make_some_value_from_taken(&mut self.store[index]),
            (Some(index), Some(erased)) => {
                self.store.swap(index, erased);
                make_some_value_from_taken(&mut self.store[erased]) //erased is taken after swap
            }
        }
    }
}

fn probing(hash: usize, probe: usize, capacity: usize) -> usize {
    (hash + probe * probe) % capacity // Quadratic probing
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
            .map(|line| {
                let line = line.unwrap();
                let mut data = line.split(' ');
                let key = data.next().unwrap().to_string();
                let value = data.next().unwrap().parse::<i64>().unwrap();
                (key, value)
            })
            .collect()
    }

    #[test]
    fn test_open_table() {
        let mut table = OpenTable::new();
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
