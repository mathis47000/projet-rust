use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};
use bincode;

use super::traits::{Cache, Cacheable};
use crate::error::CacheError;

pub struct PersistentLRUCache<K, V>
where 
    K: Eq + std::hash::Hash + Clone + Serialize + for<'de> Deserialize<'de>,
    V: Cacheable + Serialize + for<'de> Deserialize<'de>,
{
    inner: super::lru::LRUCache<K, V>,
    file_path: String,
}

impl<K, V> PersistentLRUCache<K, V>
where 
    K: Eq + std::hash::Hash + Clone + Serialize + for<'de> Deserialize<'de>,
    V: Cacheable + Serialize + for<'de> Deserialize<'de>,
{
    pub fn new_persistent(capacity: usize, file_path: &str) -> Result<Self, CacheError> {
        let mut cache = Self {
            inner: super::lru::LRUCache::new(capacity),
            file_path: file_path.to_string(),
        };
        
        cache.load()?;
        Ok(cache)
    }

    fn save(&self) -> Result<(), CacheError> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)
            .map_err(CacheError::IOError)?;

        bincode::serialize_into(file, &self.inner.map)
            .map_err(|_| CacheError::SerializationError)
    }

    fn load(&mut self) -> Result<(), CacheError> {
        if !Path::new(&self.file_path).exists() {
            return Ok(());
        }

        let mut file = File::open(&self.file_path)
            .map_err(CacheError::IOError)?;
        
        let loaded_map = bincode::deserialize_from(&mut file)
            .map_err(|_| CacheError::DeserializationError)?;
        
        self.inner.map = loaded_map;
        Ok(())
    }
}

impl<K, V> Cache<K, V> for PersistentLRUCache<K, V>
where 
    K: Eq + std::hash::Hash + Clone + Serialize + for<'de> Deserialize<'de>,
    V: Cacheable + Serialize + for<'de> Deserialize<'de>,
{
    fn put(&mut self, key: K, value: V) {
        self.inner.put(key, value);
        let _ = self.save(); // Optional: handle potential save errors
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let removed = self.inner.remove(key);
        let _ = self.save(); // Optional: handle potential save errors
        removed
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}