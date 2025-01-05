#[cfg(test)]
mod tests {
    use lru_cache::cache::memory::MemoryCache;
    use lru_cache::core::traits::Cache;

    #[test]
    fn test_basic_cache_operations() {
        let mut cache = MemoryCache::new(3);
        
        // Test put and get
        assert!(cache.put("A", "value_a").is_ok());
        assert_eq!(cache.get(&"A"), Some(&"value_a"));
        assert_eq!(cache.len(), 1);
        
        // Test capacity
        assert!(cache.put("B", "value_b").is_ok());
        assert!(cache.put("C", "value_c").is_ok());
        assert!(cache.put("D", "value_d").is_ok());
        assert_eq!(cache.get(&"A"), None); // A should be evicted
        assert_eq!(cache.len(), 3);
    }

    #[test]
    fn test_lru_behavior() {
        let mut cache = MemoryCache::new(3);
        
        assert!(cache.put("A", "value_a").is_ok());
        assert!(cache.put("B", "value_b").is_ok());
        assert!(cache.put("C", "value_c").is_ok());
        
        // Access B, making it most recently used
        assert_eq!(cache.get(&"B"), Some(&"value_b"));
        
        // Add new item, should evict least recently used (A)
        assert!(cache.put("D", "value_d").is_ok());
        assert_eq!(cache.get(&"A"), None);
        assert_eq!(cache.get(&"B"), Some(&"value_b"));
        assert_eq!(cache.get(&"C"), Some(&"value_c"));
        assert_eq!(cache.get(&"D"), Some(&"value_d"));
    }

    #[test]
    fn test_remove_and_clear() {
        let mut cache = MemoryCache::new(3);
        
        assert!(cache.put("A", "value_a").is_ok());
        assert!(cache.put("B", "value_b").is_ok());
        
        // Test remove
        assert_eq!(cache.remove(&"A"), Some("value_a"));
        assert_eq!(cache.get(&"A"), None);
        assert_eq!(cache.len(), 1);
        
        // Test clear
        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
        assert_eq!(cache.get(&"B"), None);
    }
}