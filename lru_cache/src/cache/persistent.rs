use crate::core::traits::Cache;
use crate::error::CacheError;
use crate::cache::memory::MemoryCache;
use std::hash::Hash;

pub struct PersistentCache<K, V>
where 
    K: Clone,
    V: Clone,
{
    memory_cache: MemoryCache<K, V>,
}

impl<K, V> PersistentCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn new(capacity: usize) -> Result<Self, CacheError> {
        Ok(Self {
            memory_cache: MemoryCache::new(capacity),
        })
    }

    // fn save_to_file(&self) -> Result<(), CacheError> {
    //     let mut file = File::create(&self.file_path)
    //         .map_err(|e| CacheError::IOError(e.to_string()))?;
    //     // TODO: Implémentation de la sérialisation
    //     Ok(())
    // }

    // fn load_from_file(&mut self) -> Result<(), CacheError> {
    //     if !Path::new(&self.file_path).exists() {
    //         return Ok(());
    //     }

    //     let file = File::open(&self.file_path)
    //         .map_err(|e| CacheError::IOError(e.to_string()))?;
    //     // TODO: Implémentation de la désérialisation
    //     Ok(())
    // }
}

impl<K, V> Cache<K, V> for PersistentCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn get(&mut self, key: &K) -> Option<&V> {
        self.memory_cache.get(key)
    }

    fn put(&mut self, key: K, value: V) -> Result<(), CacheError> {
        self.memory_cache.put(key, value)?;
        // self.save_to_file()?;
        Ok(())
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let result = self.memory_cache.remove(key);
        // if result.is_some() {
        //     let _ = self.save_to_file();
        // }
        result
    }

    fn clear(&mut self) {
        self.memory_cache.clear();
        // let _ = self.save_to_file();
    }

    fn len(&self) -> usize {
        self.memory_cache.len()
    }

    fn capacity(&self) -> usize {
        self.memory_cache.capacity()
    }
}