use std::collections::HashMap;

use crate::cache_trait::CacheTrait;

/// An LRU Cache meant to store an amount of data for a specific time, until a capacity buffer is expended
pub struct Cache<V> {
    capacity: usize,
    data: HashMap<String, V>,
    order: Vec<String>,
}

impl<V> Cache<V> {
    ///Capacity refers to the number of elements saved within the cache
    ///For example :
    /// ```
    /// # use lru_cache::cache_lru::Cache;
    /// # use lru_cache::cache_trait::CacheTrait;
    /// let mut cache:Cache<String> = Cache::new(3);
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

    fn remove_from_order(&mut self, key: &str) {
        let position = self.order.iter().position(|x| x == &key);
        if position.is_some() {
            let pos = position.unwrap();
            self.order.remove(pos);
        }
    }
}

impl<V> CacheTrait<V> for Cache<V> {
    fn put(&mut self, key: &str, value: V) {
        if self.data.contains_key(key) {
            self.remove_from_order(key)
        } else if self.data.len() == self.capacity {
            let old = self.order.remove(0);
            self.data.remove(&old);
        }

        self.order.push(key.to_string());
        self.data.insert(key.to_string(), value);
    }

    fn get(&mut self, key: &str) -> Option<&V> {
        if self.data.contains_key(key) {
            self.remove_from_order(key);
            self.order.push(key.to_string());
            return self.data.get(key);
        } else {
            return None;
        }
    }
}
