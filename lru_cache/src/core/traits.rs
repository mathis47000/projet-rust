use std::collections::HashMap;

use crate::error::CacheError;

use super::node::Node;

pub trait Cache<K: std::clone::Clone, V: std::clone::Clone> {
    /// Retrieves a value from the cache by its key
    fn get(&mut self, key: &K) -> Option<&V>;

    fn get_all(&self) -> &HashMap<K, Node<K, V>>;
    
    /// Inserts a key-value pair into the cache
    fn put(&mut self, key: K, value: V) -> Result<(), CacheError>;
    
    /// Removes a key-value pair from the cache
    fn remove(&mut self, key: &K) -> Option<V>;
    
    /// Clears all entries from the cache
    fn clear(&mut self);
    
    /// Returns the number of entries in the cache
    fn len(&self) -> usize;
    
    /// Returns true if the cache is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// Returns the maximum capacity of the cache
    fn capacity(&self) -> usize;
}