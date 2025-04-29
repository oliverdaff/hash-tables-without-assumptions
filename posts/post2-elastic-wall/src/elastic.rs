use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub trait HashingStrategy<K> {
    fn hash(&self, key: &K) -> u64;
}

pub struct DefaultHashStrategy;

impl<K: std::hash::Hash> HashingStrategy<K> for DefaultHashStrategy {
    fn hash(&self, key: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }
}
impl<K> HashingStrategy<K> for ModuloHashStrategy
where
    K: Copy + TryInto<usize>,
    <K as TryInto<usize>>::Error: std::fmt::Debug,
{
    fn hash(&self, key: &K) -> u64 {
        let numeric: usize = (*key)
            .try_into()
            .expect("Key must be convertible to usize for modulo hashing");
        (numeric % self.0) as u64
    }
}
pub struct ModuloHashStrategy(pub usize);

pub enum HashStrategy {
    Default(DefaultHashStrategy),
    Modulo(ModuloHashStrategy),
}

impl<K> HashingStrategy<K> for HashStrategy
where
    K: Hash + Copy + TryInto<usize>,
    <K as TryInto<usize>>::Error: std::fmt::Debug,
{
    fn hash(&self, key: &K) -> u64 {
        match self {
            HashStrategy::Default(inner) => inner.hash(key),
            HashStrategy::Modulo(inner) => inner.hash(key),
        }
    }
}

pub struct ElasticHashTable<K, V, H: HashingStrategy<K>> {
    pub subarrays: Vec<Vec<Option<(K, V)>>>,
    hasher: H,
}

impl<K, V, H: HashingStrategy<K>> ElasticHashTable<K, V, H> {
    pub fn new(num_subarrays: usize, slots_per_subarray: usize, hasher: H) -> Self {
        Self {
            subarrays: (0..num_subarrays)
                .map(|_| {
                    let mut v = Vec::with_capacity(slots_per_subarray);
                    v.resize_with(slots_per_subarray, || None);
                    v
                })
                .collect(),
            hasher,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let hash = self.hasher.hash(&key);
        let subarray_idx = (hash as usize) % self.subarrays.len();
        let subarray = &mut self.subarrays[subarray_idx];

        // Find first empty slot
        if let Some(slot) = subarray.iter_mut().find(|entry| entry.is_none()) {
            *slot = Some((key, value));
        } else {
            panic!("Subarray is full, need rehashing");
        }
    }
}
