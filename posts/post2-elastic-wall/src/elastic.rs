use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub trait HashingStrategy<K> {
    fn hash(&self, key: &K) -> u64;
}

#[derive(Debug, Clone)]
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
    slots: Vec<Option<(K, V)>>,
    subarray_count: usize,
    slots_per_subarray: usize,
    hasher: H,
    balanced: bool,
}

impl<K, V, H: HashingStrategy<K>> ElasticHashTable<K, V, H> {
    pub fn new(
        subarray_count: usize,
        slots_per_subarray: usize,
        balanced: bool,
        hasher: H,
    ) -> Self {
        let total_slots = subarray_count * slots_per_subarray;
        let slots = std::iter::repeat_with(|| None)
            .take(total_slots)
            .collect::<Vec<Option<(K, V)>>>();

        Self {
            slots,
            subarray_count,
            slots_per_subarray,
            hasher,
            balanced,
        }
    }

    fn slot_index(&self, hash: u64, subarray_idx: usize) -> usize {
        if self.balanced {
            ((hash.rotate_right(subarray_idx as u32)) as usize) % self.slots_per_subarray
        } else {
            (hash as usize) % self.slots_per_subarray
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> usize {
        let hash = self.hasher.hash(&key);
        let base = (hash as usize) % self.subarray_count;
        let mut probe_count = 0;

        // Phase 1: Try ideal slot in each subarray using fallback pattern
        for i in 0..self.subarray_count {
            let subarray_idx = (base + i) % self.subarray_count;
            let slot_idx = self.slot_index(hash, subarray_idx);
            let offset = subarray_idx * self.slots_per_subarray + slot_idx;

            probe_count += 1;
            if self.slots[offset].is_none() {
                self.slots[offset] = Some((key, value));
                return probe_count;
            }
        }

        // Phase 2: Linearly scan within each subarray from rotated starting point
        for i in 0..self.subarray_count {
            let subarray_idx = (base + i) % self.subarray_count;
            let start_idx = self.slot_index(hash, subarray_idx);

            for offset_within in 1..self.slots_per_subarray {
                probe_count += 1;
                let idx = (start_idx + offset_within) % self.slots_per_subarray;
                let offset = subarray_idx * self.slots_per_subarray + idx;

                if self.slots[offset].is_none() {
                    self.slots[offset] = Some((key, value));
                    return probe_count;
                }
            }
        }

        panic!("ElasticHashTable is full");
    }
}
