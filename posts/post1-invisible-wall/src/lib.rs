use std::collections::hash_map::DefaultHasher;
use std::convert::TryInto;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

pub mod visualizer;

pub enum HashStrategy {
    Default,
    Modulo(usize),
}

pub struct HashTable<K, V> {
    pub table: Vec<Option<(K, V)>>,
    hash_strategy: HashStrategy,
}

impl<K, V> HashTable<K, V>
where
    K: Copy + Display + Eq + Hash + TryInto<usize>,
    <K as TryInto<usize>>::Error: std::fmt::Debug,
    V: Display,
{
    pub fn new(size: usize) -> Self {
        Self {
            table: std::iter::repeat_with(|| None).take(size).collect(),
            hash_strategy: HashStrategy::Default,
        }
    }

    pub fn set_hash_strategy(&mut self, strategy: HashStrategy) {
        self.hash_strategy = strategy;
    }

    fn hash(&self, key: &K) -> usize {
        match self.hash_strategy {
            HashStrategy::Default => {
                let mut hasher = DefaultHasher::new();
                key.hash(&mut hasher);
                hasher.finish() as usize
            }
            HashStrategy::Modulo(n) => {
                let numeric: usize = (*key)
                    .try_into()
                    .expect("Key must be convertible to usize for modulo hashing");
                numeric % n
            }
        }
    }

    pub fn insert_greedy(&mut self, key: K, value: V) -> u32 {
        for i in 0..self.table.len() {
            let idx = (self.hash(&key) + i) % self.table.len();
            if self.table[idx].is_none() {
                self.table[idx] = Some((key, value));
                return i as u32;
            }
        }
        panic!("HashTable is full");
    }
}
