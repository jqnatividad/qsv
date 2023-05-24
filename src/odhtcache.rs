// blatantly copied from https://github.com/race604/dedup/blob/master/src/cache.rs
use std::collections::HashSet;

use log::debug;
use odht::{Config, FxHashFn, HashTableOwned};

struct ExtDedupConfig;

const CHUNK_SIZE: usize = 127;

impl Config for ExtDedupConfig {
    type EncodedKey = [u8; CHUNK_SIZE + 1];
    type EncodedValue = [u8; 1];
    type H = FxHashFn;
    type Key = [u8; CHUNK_SIZE + 1];
    type Value = bool;

    #[inline]
    fn encode_key(k: &Self::Key) -> Self::EncodedKey {
        *k
    }

    #[inline]
    fn encode_value(v: &Self::Value) -> Self::EncodedValue {
        [*v as u8; 1]
    }

    #[inline]
    fn decode_key(k: &Self::EncodedKey) -> Self::Key {
        *k
    }

    #[inline]
    fn decode_value(v: &Self::EncodedValue) -> Self::Value {
        v[0] == 1
    }
}

pub struct ExtDedupCache {
    memo:       HashSet<String>,
    disk:       Option<HashTableOwned<ExtDedupConfig>>,
    memo_limit: u64,
    memo_size:  u64,
}

impl ExtDedupCache {
    pub fn new(memo_limit: u64) -> Self {
        Self {
            memo:       HashSet::new(),
            disk:       None,
            memo_limit: if memo_limit == 0 {
                u64::MAX
            } else {
                memo_limit
            },
            memo_size:  0,
        }
    }

    #[inline]
    pub fn insert(&mut self, item: &str) -> bool {
        if self.memo_size >= self.memo_limit {
            self.dump_to_disk();
        }

        let mut res = self.memo.insert(item.to_owned());
        if res {
            self.memo_size += item.len() as u64;
            if self.disk.is_some() {
                res = self.insert_on_disk(item);
                // debug!("Insert on disk: {res}");
            }
        }

        res
    }

    #[inline]
    pub fn contains(&self, item: &str) -> bool {
        if self.memo.contains(item) {
            return true;
        }

        return if let Some(ref disk) = self.disk {
            ExtDedupCache::item_to_keys(item).all(|key| disk.contains_key(&key))
        } else {
            false
        };
    }

    fn insert_on_disk(&mut self, item: &str) -> bool {
        let disk = self.disk.get_or_insert_with(|| {
            debug!("Create new disk cache");
            HashTableOwned::<ExtDedupConfig>::with_capacity(1_000_000, 95)
        });
        let mut res = false;
        for key in ExtDedupCache::item_to_keys(item) {
            res = disk.insert(&key, &true).is_none() || res;
        }
        res
    }

    fn item_to_keys(item: &str) -> impl Iterator<Item = [u8; CHUNK_SIZE + 1]> + '_ {
        let res = item
            .as_bytes()
            .chunks(CHUNK_SIZE)
            .enumerate()
            .map(|(i, chunk)| {
                let mut key = [0_u8; CHUNK_SIZE + 1];
                key[CHUNK_SIZE] = i as u8;
                key[..chunk.len()].copy_from_slice(chunk);
                key
            });
        res
    }

    fn dump_to_disk(&mut self) {
        // debug!("Memory cache is full, dump to disk");
        let keys = self.memo.drain().collect::<Vec<_>>();
        for key in keys {
            self.insert_on_disk(&key);
        }
        self.memo_size = 0;
    }
}

#[cfg(test)]
mod tests {
    use rand::{distributions::Alphanumeric, thread_rng, Rng};

    use super::*;

    #[test]
    fn test_basic_cache() {
        let mut cache = ExtDedupCache::new(0);
        assert!(cache.insert("hello"));
        assert!(cache.insert("world"));

        assert!(cache.contains("hello"));
        assert!(cache.contains("world"));
        assert!(!cache.contains("other"));
    }

    #[test]
    fn test_limit_memory() {
        let mut cache = ExtDedupCache::new(1024);
        for _ in 0..100 {
            cache.insert(&rand_string(32));
        }
        assert!(cache.memo.len() < 100);
        assert!(cache.disk.is_some());
        assert!(cache.disk.unwrap().len() > 0);
    }

    fn rand_string(len: usize) -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }
}
