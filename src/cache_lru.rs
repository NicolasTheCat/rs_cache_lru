use std::{collections::HashMap, hash::Hash};

use crate::cache_trait::CacheTrait;

/// An LRU Cache meant to store an amount of data for a specific time, until a capacity buffer is expended
pub struct Cache<K, V> {
    capacity: usize,
    data: HashMap<K, V>,
    order: Vec<K>,
}

impl<K, V> Cache<K, V>
where
    K: Clone + Eq + Hash,
{
    ///Capacity refers to the number of elements saved within the cache
    ///For example :
    /// ```
    /// # use lru_cache::cache_lru::Cache;
    /// # use lru_cache::cache_trait::CacheTrait;
    /// let mut cache:Cache<&str,String> = Cache::new(3);
    /// ```
    /// Creates a new cache that will hold 3 Strings at once before removing the oldest one.
    ///
    /// Make sure to specify the type of the cache value and key
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: HashMap::with_capacity(capacity),
            order: Vec::with_capacity(capacity),
        }
    }

    fn remove_from_order(&mut self, key: &K) {
        let position = self.order.iter().position(|x| x == key);
        if position.is_some() {
            let pos = position.unwrap();
            self.order.remove(pos);
        }
    }
}

impl<K, V> CacheTrait<K, V> for Cache<K, V>
where
    K: Clone + Eq + Hash,
{
    fn put(&mut self, key: K, value: V) {
        if self.data.contains_key(&key) {
            self.remove_from_order(&key)
        } else if self.data.len() == self.capacity {
            let old = self.order.remove(0);
            self.data.remove(&old);
        }

        self.order.push(key.clone());
        self.data.insert(key, value);
    }

    fn get(&mut self, key: K) -> Option<&V> {
        if self.data.contains_key(&key) {
            self.remove_from_order(&key);
            self.order.push(key.clone());
            return self.data.get(&key);
        } else {
            return None;
        }
    }
}
