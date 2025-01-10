#[cfg(test)]
mod tests {
    use lru_cache::cache::memory::MemoryCache;
    use lru_cache::cache::persistent::PersistentCache;
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
        assert!(cache.put("A", String::from("value_a")).is_ok());
        assert!(cache.put("B", String::from("value_b")).is_ok());
        assert!(cache.put("C", String::from("value_c")).is_ok());
        assert!(cache.put("D", String::from("value_d")).is_ok());
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

        assert!(cache.put("A", String::from("value_a")).is_ok());
        // Cache == [B, C, A]

        assert!(cache.put("X", String::from("value_x")).is_ok());
        // Cache == [C, A, X]

        let my_value = cache.get(&"B");
        assert_eq!(my_value, None);
        // Cache == [C, A, X]

        let my_value = cache.get(&"D");
        // Cache == [C, A, X]
        assert_eq!(my_value, None);
    }

    #[test]
fn test_persistent_cache_integration() {
    let file_path = "tests/test_cache.txt";
    let mut cache = PersistentCache::new_persistent(3, file_path).expect("Failed to create cache");

    assert_eq!(cache.get(&"key1".to_string()), Some(&"mathis".to_string()));
    assert_eq!(cache.get(&"key2".to_string()), Some(&"emma".to_string()));
    assert_eq!(cache.get(&"key3".to_string()), Some(&"antoine".to_string()));
    assert_eq!(cache.len(), 3);
    
    cache.put("key1".to_string(), "value1".to_string()).unwrap();
    cache.put("key2".to_string(), "value2".to_string()).unwrap();
    cache.put("key3".to_string(), "value3".to_string()).unwrap();


    let mut new_cache = PersistentCache::new_persistent(3, file_path).expect("Failed to create cache");
    assert_eq!(new_cache.get(&"key1".to_string()), Some(&"value1".to_string()));
    assert_eq!(new_cache.get(&"key2".to_string()), Some(&"value2".to_string()));
    assert_eq!(new_cache.len(), 3);

    new_cache.remove(&"key1".to_string());
    assert_eq!(new_cache.get(&"key1".to_string()), None);
    new_cache.put("key4".to_string(), "value4".to_string()).unwrap();

    let mut reloaded_cache = PersistentCache::new_persistent(3, file_path).expect("Failed to reload cache again");
    assert_eq!(reloaded_cache.get(&"key1".to_string()), None);
    assert_eq!(reloaded_cache.get(&"key4".to_string()), Some(&"value4".to_string()));

    cache.put("key1".to_string(), "mathis".to_string()).unwrap();
    cache.put("key2".to_string(), "emma".to_string()).unwrap();
    cache.put("key3".to_string(), "antoine".to_string()).unwrap();

}
}