pub trait CacheTrait<V> {
    /// Puts new data within the cache under the specified key
    ///
    /// ```
    /// # use lru_cache::cache_lru::Cache;
    /// # use lru_cache::cache_trait::CacheTrait;
    /// let mut cache:Cache<String> = Cache::new(3);
    /// cache.put("A", String::from("value_a"));
    /// ```
    ///
    /// Putting more data than the cache can store will result in deletion of the oldest key/value pair
    /// ```
    /// # use lru_cache::cache_lru::Cache;
    /// # use lru_cache::cache_trait::CacheTrait;
    /// let mut cache:Cache<String> = Cache::new(3);
    /// cache.put("A", String::from("value_a")); // => [A]     "oldest" value to "newest"
    /// cache.put("B", String::from("value_b")); // => [A,B]
    /// cache.put("C", String::from("value_c")); // => [A,B,C]
    /// cache.put("D", String::from("value_d")); // => [B,C,D]
    /// ```
    fn put(&mut self, key: &str, value: V);

    /// Retrieves data from the cache using a specific key
    ///
    /// Can potentially return `None`
    /// ```
    /// # use lru_cache::cache_lru::Cache;
    /// # use lru_cache::cache_trait::CacheTrait;
    /// let mut cache:Cache<String> = Cache::new(3);
    /// cache.put("A", String::from("value_a"));
    /// let my_value = cache.get("A").unwrap();
    /// ```
    /// Retrieving a value "refreshes" it in the priority deletion order
    /// ```
    /// # use lru_cache::cache_lru::Cache;
    /// # use lru_cache::cache_trait::CacheTrait;
    /// let mut cache:Cache<String> = Cache::new(3);
    /// cache.put("A", String::from("value_a")); // => [A]     "oldest" value to "newest"
    /// cache.put("B", String::from("value_b")); // => [A,B]
    /// cache.put("C", String::from("value_c")); // => [A,B,C]
    /// let my_value = cache.get("A").unwrap();  // => [B,C,A]
    ///
    /// ```
    ///
    fn get(&mut self, key: &str) -> Option<&V>;
}
