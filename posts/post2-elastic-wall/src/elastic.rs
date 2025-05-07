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
    variation: bool,
}

impl<K, V, H: HashingStrategy<K>> ElasticHashTable<K, V, H> {
    pub fn new(
        num_subarrays: usize,
        slots_per_subarray: usize,
        variation: bool,
        hasher: H,
    ) -> Self {
        Self {
            subarrays: (0..num_subarrays)
                .map(|_| {
                    let mut v = Vec::with_capacity(slots_per_subarray);
                    v.resize_with(slots_per_subarray, || None);
                    v
                })
                .collect(),
            variation,
            hasher,
        }
    }

    fn slot_index(&self, hash: u64, subarray_idx: usize) -> usize {
        let subarray = &self.subarrays[subarray_idx];
        if self.variation {
            ((hash.rotate_right(subarray_idx as u32)) as usize) % subarray.len()
        } else {
            (hash as usize) % subarray.len()
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> u32 {
        let hash = self.hasher.hash(&key);
        let base = (hash as usize) % self.subarrays.len();

        let mut probes = 0;

        // Phase 1: Try ideal slot in each subarray, vary slot_idx by subarray index
        for i in 0..self.subarrays.len() {
            let subarray_idx = (base + i) % self.subarrays.len();

            let slot_idx = self.slot_index(hash, subarray_idx);
            probes += 1;
            let subarray = &mut self.subarrays[subarray_idx];
            if subarray[slot_idx].is_none() {
                subarray[slot_idx] = Some((key, value));
                return probes;
            }
        }

        // Phase 2: Scan forward from varied starting point per subarray
        for i in 0..self.subarrays.len() {
            let subarray_idx = (base + i) % self.subarrays.len();
            let start_idx = self.slot_index(hash, subarray_idx);

            let subarray = &mut self.subarrays[subarray_idx];
            for offset in 1..subarray.len() {
                probes += 1;
                let idx = (start_idx + offset) % subarray.len();
                if subarray[idx].is_none() {
                    subarray[idx] = Some((key, value));
                    return probes;
                }
            }
        }

        panic!("ElasticHashTable is full");
    }
}
