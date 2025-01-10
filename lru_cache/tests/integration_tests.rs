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

    #[test]
    fn test_lru_cache() {
        let mut cache = MemoryCache::new(3); // Taille de 3
        cache.put("A", String::from("value_a"));
        cache.put("B", String::from("value_b"));
        cache.put("C", String::from("value_c"));
        cache.put("D", String::from("value_d"));
        // Premier élément moins récemment utilisé et dernier le plus récent
        // Cache == [B, C, D]

        let my_value = cache.get(&"A");
        assert_eq!(my_value, None);
        let my_value = cache.get(&"D");
        assert_eq!(my_value, Some(&String::from("value_d")));
        // Cache == [B, C, D]

        let my_value = cache.get(&"B");
        assert_eq!(my_value, Some(&String::from("value_b")));
        // Cache == [C, D, B]

        let my_value = cache.get(&"C");
        assert_eq!(my_value, Some(&String::from("value_c")));
        // Cache == [D, B, C]

        let my_value = cache.get(&"X");
        assert_eq!(my_value, None);
        // Cache == [D, B, C]

        cache.put("A", String::from("value_a"));
        // Cache == [B, C, A]

        cache.put("X", String::from("value_x"));
        // Cache == [C, A, X]

        let my_value = cache.get(&"B");
        assert_eq!(my_value, None);
        // Cache == [C, A, X]

        let my_value = cache.get(&"D");
        // Cache == [C, A, X]
        assert_eq!(my_value, None);
    }
}