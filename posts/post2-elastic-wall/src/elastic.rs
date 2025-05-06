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

    pub fn insert(&mut self, key: K, value: V) -> u32 {
        let hash = self.hasher.hash(&key);
        let base = (hash as usize) % self.subarrays.len();

        let mut probes = 0;

        // --- Phase 1: Try one ideal slot per subarray
        for offset in 0..self.subarrays.len() {
            probes += 1;
            let subarray_idx = (base + offset) % self.subarrays.len();
            let subarray = &mut self.subarrays[subarray_idx];
            let ideal_slot = (hash as usize) % subarray.len();

            if let Some(None) = subarray.get_mut(ideal_slot) {
                subarray[ideal_slot] = Some((key, value));
                return probes + 1;
            }
        }

        // --- Phase 2: Fallback linear scan from ideal slot per subarray
        for subarray_idx in 0..self.subarrays.len() {
            let subarray = &mut self.subarrays[subarray_idx];
            let start = (hash as usize) % subarray.len();

            for probe in 0..subarray.len() {
                probes += 1;
                let idx = (start + probe) % subarray.len();

                if let Some(None) = subarray.get_mut(idx) {
                    subarray[idx] = Some((key, value));
                    return probes;
                }
            }
        }

        panic!("Hash table full — rehash or resize required.");
    }
}
