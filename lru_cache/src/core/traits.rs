use std::collections::HashMap;

use crate::error::CacheError;

use super::node::Node;

pub trait Cache<K: std::clone::Clone, V: std::clone::Clone> {

    fn get(&mut self, key: &K) -> Option<&V>;

    fn get_all(&self) -> &HashMap<K, Node<K, V>>;
    
    fn put(&mut self, key: K, value: V) -> Result<(), CacheError>;
    
    fn remove(&mut self, key: &K) -> Option<V>;
    
    fn clear(&mut self);
    
    fn len(&self) -> usize;
    
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn capacity(&self) -> usize;
}