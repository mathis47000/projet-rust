use crate::core::node::Node;
use crate::core::traits::Cache;
use crate::error::CacheError;
use crate::cache::memory::MemoryCache;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::hash::Hash;

pub struct PersistentCache<K, V>
where
    K: Eq + Hash + Clone + ToString + std::str::FromStr,
    V: Clone + ToString + std::str::FromStr,
{
    memory_cache: MemoryCache<K, V>,
    file_path: String,
}

impl<K, V> PersistentCache<K, V>
where
    K: Eq + Hash + Clone + ToString + std::str::FromStr,
    V: Clone + ToString + std::str::FromStr,
    <K as std::str::FromStr>::Err: std::fmt::Debug,
    <V as std::str::FromStr>::Err: std::fmt::Debug,
{
    pub fn new_persistent(capacity: usize, file_path: &str) -> Result<Self, CacheError> {
        let mut memory_cache = MemoryCache::new(capacity);

        // Charger les données depuis le fichier
        if let Ok(mut file) = File::open(file_path) {
            let mut contents = String::new();
            let _ = file.read_to_string(&mut contents);

            for line in contents.lines() {
                let mut parts = line.splitn(2, '=');
                if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                    let key: K = key.parse().expect("Invalid key format");
                    let value: V = value.parse().expect("Invalid value format");
                    memory_cache.put(key, value)?;
                }
            }
        }

        Ok(Self {
            memory_cache,
            file_path: file_path.to_string(),
        })
    }

    fn save_to_file(&self) -> Result<(), CacheError> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path);

            match file {
                Ok(mut file) => {
                    for (key, node) in self.memory_cache.get_all() {
                        let _ = writeln!(file, "{}={}", key.to_string(), node.value.to_string())
                            .map_err(|err| eprintln!("Failed to write to file: {}", err));
                    }
                }
                Err(err) => {
                    eprintln!("Failed to open file: {}", err);
                }
            }
        Ok(())
    }
}

impl<K, V> Cache<K, V> for PersistentCache<K, V>
where
    K: Eq + Hash + Clone + ToString + std::str::FromStr,
    V: Clone + ToString + std::str::FromStr,
    <K as std::str::FromStr>::Err: std::fmt::Debug,
    <V as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn get(&mut self, key: &K) -> Option<&V> {
        self.memory_cache.get(key)
    }

    fn put(&mut self, key: K, value: V) -> Result<(), CacheError> {
        self.memory_cache.put(key, value)?;
        self.save_to_file()?;
        Ok(())
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let result = self.memory_cache.remove(key);
        if result.is_some() {
            self.save_to_file().ok();
        }
        result
    }

    fn clear(&mut self) {
        self.memory_cache.clear();
        self.save_to_file().ok();
    }

    fn len(&self) -> usize {
        self.memory_cache.len()
    }

    fn capacity(&self) -> usize {
        self.memory_cache.capacity()
    }
    
    fn get_all(&self) -> &HashMap<K, Node<K, V>> {
        &self.memory_cache.get_all()
    }
}
