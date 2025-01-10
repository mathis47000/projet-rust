use std::collections::HashMap;
use std::hash::Hash;
use crate::core::traits::Cache;
use crate::core::node::Node;
use crate::error::CacheError;

pub struct MemoryCache<K, V> 
where 
    K: Clone,
    V: Clone,
{
    capacity: usize,
    storage: HashMap<K, Node<K, V>>,
    head: Option<K>,
    tail: Option<K>,
}

impl<K, V> MemoryCache<K, V>
where 
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            storage: HashMap::with_capacity(capacity),
            head: None,
            tail: None,
        }
    }

    fn move_to_front(&mut self, key: &K) {
        if self.head.as_ref() == Some(key) {
            return;
        }

        let node = self.storage.get(key).expect("Node should exist").clone();
        
        // Update links
        if let Some(prev_key) = node.prev.clone() {
            if let Some(prev_node) = self.storage.get_mut(&prev_key) {
                prev_node.next = node.next.clone();
            }
        }
        
        if let Some(next_key) = node.next.clone() {
            if let Some(next_node) = self.storage.get_mut(&next_key) {
                next_node.prev = node.prev.clone();
            }
        }

        if Some(key.clone()) == self.tail {
            self.tail = node.prev.clone();
        }

        let mut updated_node = node;
        updated_node.prev = None;
        updated_node.next = self.head.clone();
        
        if let Some(old_head) = &self.head {
            if let Some(head_node) = self.storage.get_mut(old_head) {
                head_node.prev = Some(key.clone());
            }
        }

        self.head = Some(key.clone());
        self.storage.insert(key.clone(), updated_node);
    }
}

impl<K, V> Cache<K, V> for MemoryCache<K, V>
where 
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn get(&mut self, key: &K) -> Option<&V> {
        if self.storage.contains_key(key) {
            self.move_to_front(key);
            self.storage.get(key).map(|node| &node.value)
        } else {
            None
        }
    }

    fn put(&mut self, key: K, value: V) -> Result<(), CacheError> {
        if self.capacity == 0 {
            return Err(CacheError::CapacityError("Cache capacity cannot be 0".into()));
        }

        if self.storage.contains_key(&key) {
            let mut node = self.storage.get(&key).unwrap().clone();
            node.value = value;
            self.storage.insert(key.clone(), node);
            self.move_to_front(&key);
            return Ok(());
        }

        let new_node = Node {
            key: key.clone(),
            value,
            prev: None,
            next: self.head.clone(),
        };

        if let Some(ref old_head) = self.head {
            if let Some(head_node) = self.storage.get_mut(old_head) {
                head_node.prev = Some(key.clone());
            }
        }

        self.storage.insert(key.clone(), new_node);
        
        if self.tail.is_none() {
            self.tail = Some(key.clone());
        }
        self.head = Some(key);

        if self.storage.len() > self.capacity {
            if let Some(old_tail) = self.tail.clone() {
                let old_tail_prev = self.storage.get(&old_tail)
                    .and_then(|node| node.prev.clone());
                self.tail = old_tail_prev;
                self.storage.remove(&old_tail);
            }
        }

        Ok(())
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(node) = self.storage.remove(key) {
            // Update the previous node's `next` pointer
            if let Some(ref prev_key) = node.prev {
                if let Some(prev_node) = self.storage.get_mut(prev_key) {
                    prev_node.next = node.next.clone();
                }
            }
    
            // Update the next node's `prev` pointer
            if let Some(ref next_key) = node.next {
                if let Some(next_node) = self.storage.get_mut(next_key) {
                    next_node.prev = node.prev.clone();
                }
            }
    
            // Update `head` if needed
            if Some(key.clone()) == self.head {
                self.head = node.next.clone();
            }
    
            // Update `tail` if needed
            if Some(key.clone()) == self.tail {
                self.tail = node.prev.clone();
            }
    
            // Return the removed node's value
            Some(node.value)
        } else {
            None
        }
    }
    

    fn clear(&mut self) {
        self.storage.clear();
        self.head = None;
        self.tail = None;
    }

    fn len(&self) -> usize {
        self.storage.len()
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
    
    fn get_all(&self) -> &HashMap<K, Node<K, V>> {
        &self.storage
    }
}