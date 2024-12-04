use lru_cache::LRUCache;
use lru_cache::traits::Cache;

#[test]
fn test_lru_cache_basic_operations() {
    let mut cache = LRUCache::new(2);
    
    cache.put("key1", 1);
    cache.put("key2", 2);
    
    assert_eq!(cache.get(&"key1"), Some(&1));
    assert_eq!(cache.len(), 2);
    
    cache.put("key3", 3);  // This should evict "key2"
    
    assert_eq!(cache.get(&"key2"), None);
    assert_eq!(cache.get(&"key3"), Some(&3));
}

#[test]
fn test_cache_with_custom_types() {
    #[derive(Clone, Debug, PartialEq)]
    struct Complex {
        real: f64,
        imag: f64,
    }

    let mut cache = LRUCache::new(3);
    
    let c1 = Complex { real: 1.0, imag: 2.0 };
    let c2 = Complex { real: 3.0, imag: 4.0 };
    
    cache.put("num1", c1.clone());
    cache.put("num2", c2.clone());
    
    assert_eq!(cache.get(&"num1"), Some(&c1));
}