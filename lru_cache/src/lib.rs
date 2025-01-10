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
