use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use super::traits::{Cache, Cacheable};

pub struct LRUCache<K, V>
where 
    K: Eq + Hash + Clone,
    V: Cacheable,
{
    capacity: usize,
    map: HashMap<K, V>,
    order: VecDeque<K>,
}

impl<K, V> LRUCache<K, V>
where 
    K: Eq + Hash + Clone,
    V: Cacheable,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: HashMap::with_capacity(capacity),
            order: VecDeque::with_capacity(capacity),
        }
    }
}

impl<K, V> Cache<K, V> for LRUCache<K, V>
where 
    K: Eq + Hash + Clone,
    V: Cacheable,
{
    fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            // Update existing key's position
            self.order.retain(|k| k != &key);
        } else if self.len() >= self.capacity {
            // Remove least recently used item
            if let Some(lru_key) = self.order.pop_front() {
                self.map.remove(&lru_key);
            }
        }

        self.order.push_back(key.clone());
        self.map.insert(key, value);
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(value) = self.map.get(key) {
            // Move key to end (most recently used)
            self.order.retain(|k| k != key);
            self.order.push_back(key.clone());
            Some(value)
        } else {
            None
        }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.order.retain(|k| k != key);
        self.map.remove(key)
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}