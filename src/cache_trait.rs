pub trait CacheTrait<V> {
    fn put(&mut self, key: &str, value: V);
    fn get(&mut self, key: &str) -> Option<&V>;
}
