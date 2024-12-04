use lru_cache::LRUCache;
use lru_cache::traits::Cache;

fn main() {
    // Create an LRU cache with capacity 3
    let mut cache = LRUCache::new(3);
    
    // Put some key-value pairs
    cache.put("apple", 1);
    cache.put("banana", 2);
    cache.put("cherry", 3);
    
    // Accessing a key moves it to most recently used
    assert_eq!(cache.get(&"apple"), Some(&1));
    
    // Adding a 4th item will evict the least recently used item
    cache.put("date", 4);
    assert_eq!(cache.get(&"banana"), None);
}