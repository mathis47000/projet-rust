//! # LRU Cache
//! 
//! A Least Recently Used (LRU) cache implementation with support for both in-memory
//! and persistent storage.
//! 
//! ## Features
//! 
//! - Generic key and value types
//! - Fixed capacity with LRU eviction
//! - In-memory storage
//! - Persistent storage option
//! 
//! ## Example
//! 
//! ```rust
//! use lru_cache::cache::memory::MemoryCache;
//! use lru_cache::core::traits::Cache;
//! 
//! let mut cache = MemoryCache::new(3);
//! cache.put("key1", "value1");
//! assert_eq!(cache.get(&"key1"), Some(&"value1"));
//! ```

pub mod cache;
pub mod core;
pub mod error;

// src/error.rs
#[derive(Debug)]
pub enum CacheError {
    IoError(String),
    SerializationError(String),
    CapacityError(String),
}

impl std::fmt::Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheError::IoError(msg) => write!(f, "IO Error: {}", msg),
            CacheError::SerializationError(msg) => write!(f, "Serialization Error: {}", msg),
            CacheError::CapacityError(msg) => write!(f, "Capacity Error: {}", msg),
        }
    }
}

impl std::error::Error for CacheError {}