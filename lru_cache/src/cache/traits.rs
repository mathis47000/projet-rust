use std::hash::Hash;
use std::fmt::Debug;

/// Core trait for cacheable items
pub trait Cacheable: Clone + Debug + PartialEq {}
impl<T: Clone + Debug + PartialEq> Cacheable for T {}

/// Cache interface defining core operations
pub trait Cache<K, V> 
where 
    K: Eq + Hash + Clone,
    V: Cacheable,
{
    /// Insert a key-value pair into the cache
    fn put(&mut self, key: K, value: V);

    /// Retrieve a value from the cache
    fn get(&mut self, key: &K) -> Option<&V>;

    /// Remove a key from the cache
    fn remove(&mut self, key: &K) -> Option<V>;

    /// Get current cache size
    fn len(&self) -> usize;

    /// Check if cache is empty
    fn is_empty(&self) -> bool;
}