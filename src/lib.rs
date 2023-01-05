use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub mod chain;
pub mod open;

pub trait HashTable {
    fn insert(&mut self, key: String, value: i64) -> Option<i64>;
    fn remove(&mut self, key: String) -> Option<i64>;
    fn get(&self, key: &String) -> Option<&i64>;
    fn get_mut(&mut self, key: &String) -> Option<&mut i64>;
}

pub(crate) fn hash(key: &str) -> usize {
    let mut s = DefaultHasher::new();
    key.hash(&mut s);
    s.finish() as _
}
