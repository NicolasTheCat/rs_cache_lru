#[cfg(test)]
mod unit_tests {
    use crate::cache_lru::Cache;
    use crate::cache_trait::CacheTrait;

    #[test]
    fn get_test_string() {
        let mut cache_string: Cache<String> = Cache::new(3);
        cache_string.put("A_string", String::from("Test"));
        assert_eq!(cache_string.get("A_string").unwrap(), "Test");
        let my_value = cache_string.get("NONE");
        assert_eq!(my_value, None);
    }

    #[test]
    fn get_test_usize() {
        let mut cache_usize: Cache<usize> = Cache::new(3);
        let value: usize = 54;
        cache_usize.put("A_usize", value);
        assert_eq!(cache_usize.get("A_usize").unwrap(), &value);
        let my_value = cache_usize.get("NONE");
        assert_eq!(my_value, None);
    }

    #[test]
    fn put_test() {
        let mut cache = Cache::new(3);
        cache.put("A", 1);
        assert_eq!(cache.get("A").unwrap(), &1);
        cache.put("A", 2);
        assert_eq!(cache.get("A").unwrap(), &2);

        cache.put("B", 1);
        cache.put("C", 1);
        cache.put("D", 1);
        assert_eq!(cache.get("B").unwrap(), &1);
        assert_eq!(cache.get("A"), None);
    }
}
