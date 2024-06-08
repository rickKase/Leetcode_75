/*
Question 48: LRU Cache

Question:
---------
Design and implement a data structure for Least Recently Used (LRU) cache.
It should support the following operations: get and put.
- get(key): Get the value (will always be positive) of the key if the key exists in the cache, otherwise return -1.
- put(key, value): Set or insert the value if the key is not already present. When the cache reaches its capacity, it should invalidate the least recently used item before inserting a new item.

Example:
---------
LRUCache cache = new LRUCache( 2 ); // capacity 2
cache.put(1, 1);
cache.put(2, 2);
cache.get(1);       // returns 1
cache.put(3, 3);    // evicts key 2
cache.get(2);       // returns -1 (not found)
cache.put(4, 4);    // evicts key 1
cache.get(1);       // returns -1 (not found)
cache.get(3);       // returns 3
cache.get(4);       // returns 4
*/

use std::collections::{HashMap, VecDeque};

pub struct LRUCache {
    capacity: usize,
    map: HashMap<i32, i32>,
    order: VecDeque<i32>, // Most recently used at the back.
}

impl LRUCache {
    /// Creates a new LRUCache with the given capacity.
    pub fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            map: HashMap::new(),
            order: VecDeque::new(),
        }
    }
    
    /// Gets the value for the given key if it exists in the cache.
    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&val) = self.map.get(&key) {
            // Update order: remove key and push it to the back.
            if let Some(pos) = self.order.iter().position(|&x| x == key) {
                self.order.remove(pos);
            }
            self.order.push_back(key);
            val
        } else {
            -1
        }
    }
    
    /// Puts the key-value pair into the cache.
    pub fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            // Update value and refresh order.
            self.map.insert(key, value);
            if let Some(pos) = self.order.iter().position(|&x| x == key) {
                self.order.remove(pos);
            }
            self.order.push_back(key);
        } else {
            // If at capacity, remove the least recently used item.
            if self.map.len() == self.capacity {
                if let Some(lru) = self.order.pop_front() {
                    self.map.remove(&lru);
                }
            }
            self.map.insert(key, value);
            self.order.push_back(key);
        }
    }
}

/// Prints the question title to the CLI.
pub fn print_question() {
    println!("Question 48: LRU Cache");
    println!("----------------------");
    println!("Design a data structure for LRU Cache with get and put operations.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lru_cache() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);    // evicts key 2
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);    // evicts key 1
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }
}
