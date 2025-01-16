
#[cfg(test)]
mod tests {
    use lru_cache::{cache::memory::MemoryCache, core::traits::Cache};

    #[test]
    fn test_memory_cache_basic_operations() {
        let mut cache = MemoryCache::new(3);
        
        cache.put(1, "one".to_string()).unwrap();
        assert_eq!(cache.get(&1).map(|v| v.to_string()), Some("one".to_string()));
        
        cache.put(2, "two".to_string()).unwrap();
        cache.put(3, "three".to_string()).unwrap();
        cache.put(4, "four".to_string()).unwrap();
        
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.len(), 3);
    }

    #[test]
    fn test_memory_cache_update_existing() {
        let mut cache = MemoryCache::new(2);
        
        cache.put(1, "one".to_string()).unwrap();
        cache.put(1, "updated".to_string()).unwrap();
        
        assert_eq!(cache.get(&1).map(|v| v.to_string()), Some("updated".to_string()));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_memory_cache_remove() {
        let mut cache = MemoryCache::new(2);
        
        cache.put(1, "one".to_string()).unwrap();
        cache.put(2, "two".to_string()).unwrap();
        
        let removed = cache.remove(&1);
        assert_eq!(removed, Some("one".to_string()));
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_memory_cache_clear() {
        let mut cache = MemoryCache::new(2);
        
        cache.put(1, "one".to_string()).unwrap();
        cache.put(2, "two".to_string()).unwrap();
        
        cache.clear();
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), None);
    }

    #[test]
    fn test_memory_cache_zero_capacity() {
        let mut cache = MemoryCache::new(0);
        
        let result = cache.put(1, "one".to_string());
        assert!(result.is_err());
    }
}