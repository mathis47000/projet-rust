//! # LRU Cache Library
//! 
//! A flexible, generic LRU (Least Recently Used) cache implementation with optional persistent storage.
//! 
//! ## Features
//! - Generic key and value types
//! - Optional persistent storage
//! - Trait-based design for extensibility

mod cache;
mod error;

pub use cache::lru::LRUCache;
pub use cache::traits::{Cache, Cacheable};
pub use error::CacheError;