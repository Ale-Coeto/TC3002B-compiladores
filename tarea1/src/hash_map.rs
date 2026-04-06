//! HashMap implementation using separate chaining for collision handling.
//! Uses a `Vec<Vec<(K, T)>>` where each inner vector is a bucket.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash_index<K: Hash>(key: &K, capacity: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    (hasher.finish() as usize) % capacity
}

/// Hash map
pub struct HashMap<K, T> {
    data: Vec<Vec<(K, T)>>,
    size: usize,
    capacity: usize,
}

impl<K: Hash + PartialEq, T> HashMap<K, T> {
    /// Creates a new empty hash map.
    /// Initial capacity is 16.
    pub fn new() -> Self {
        let capacity = 16;
        Self {
            data: (0..capacity).map(|_| Vec::new()).collect(),
            size: 0,
            capacity,
        }
    }

    /// Resizes the table to double capacity and rehashes all pairs.
    /// Time complexity: O(n), where n is the number of stored pairs.
    fn resize(&mut self) {
        self.capacity = self.capacity * 2;
        let mut new_data: Vec<Vec<(K, T)>> = (0..self.capacity).map(|_| Vec::new()).collect();

        for list in self.data.drain(..) {
            for pair in list {
                let new_index = hash_index(&pair.0, self.capacity);
                new_data[new_index].push((pair.0, pair.1));
            }
        }

        self.data = new_data;
    }

    /// Inserts a key-value pair into the map.
    /// Replaces the value if the key already exists.
    /// Time complexity: average O(1), worst-case O(n)
    /// # Arguments
    /// * `key` - The key to insert.
    /// * `value` - The value associated with the key.
    pub fn insert(&mut self, key: K, value: T) {
        if (self.size as f64 / self.capacity as f64) > 0.75 {
            self.resize();
        }
        let index = hash_index(&key, self.capacity);
        for pair in &mut self.data[index] {
            if pair.0 == key {
                pair.1 = value;
                return;
            }
        }
        self.data[index].push((key, value));
        self.size += 1;
    }

    /// Returns a reference to the value associated with a key.
    /// Time complexity: average O(1), worst-case O(n)
    /// # Arguments
    /// * `key` - The key to look up.
    /// # Returns
    /// * `Some(&T)` - A reference to the value if key exists.
    /// * `None` - If key does not exist.
    pub fn get(&self, key: &K) -> Option<&T> {
        let index = hash_index(&key, self.capacity); 
        for pair in &self.data[index] {
            if pair.0 == *key {
                return Some(&pair.1);
            }
        }

        None
    }

    /// Removes a key-value pair from the map if the key exists.
    /// Time complexity: average O(1), worst-case O(n)
    /// # Arguments
    /// * `key` - The key to remove.
    pub fn remove(&mut self, key: K) {
        let index = hash_index(&key, self.capacity);
        let index_list = &mut self.data[index];
        for i in 0..index_list.len() {
            if index_list[i].0 == key {
                index_list.remove(i);
                self.size -= 1;
                return;
            }
        }
    }

    /// Checks whether a key exists in the map.
    /// Time complexity: average O(1), worst-case O(n)
    /// # Arguments
    /// * `key` - The key to check.
    /// # Returns
    /// * `true` - If the key exists.
    /// * `false` - If the key does not exist.
    pub fn has_key(&self, key: &K) -> bool {
        let index = hash_index(&key, self.capacity);
        for pair in &self.data[index] {
            if pair.0 == *key {
                return true;
            }
        }

        false
    }

    /// Returns the number of key-value pairs in the map.
    /// Time complexity: O(1)
    pub fn size(&self) -> usize {
        return self.size;
    }

    /// Checks whether the map is empty.
    /// Time complexity: O(1)
    pub fn is_empty(&self) -> bool {
        return self.size == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mp: HashMap<String, i32> = HashMap::new();

        assert_eq!(mp.size(), 0);
        assert!(mp.is_empty());
    }

    #[test]
    fn test_insert() {
        let mut mp: HashMap<&str, i32> = HashMap::new();
        mp.insert("Hello", 1);
        mp.insert("World", 1);
        assert_eq!(mp.size(), 2);
        assert_eq!(mp.get(&"Hello"), Some(&1));
        
        mp.insert("Hello", 2);
        assert_eq!(mp.size(), 2);
        assert_eq!(mp.get(&"Hello"), Some(&2));
    }

    #[test]
    fn test_get() {
        let mut mp: HashMap<i32, bool> = HashMap::new();
        mp.insert(10, true);
        mp.insert(1, false);
        assert_eq!(mp.get(&10), Some(&true));
        assert_eq!(mp.get(&1), Some(&false));
        mp.insert(1, true);
        assert_eq!(mp.get(&1), Some(&true));
        assert_eq!(mp.get(&5), None);
    }

    #[test]
    fn test_remove() {
        let mut mp: HashMap<&str, i32> = HashMap::new();
        mp.insert("a", 1);
        mp.insert("b", 2);

        mp.remove("b");
        assert_eq!(mp.size(), 1);
        assert_eq!(mp.get(&"b"), None);
        assert_eq!(mp.get(&"a"), Some(&1));

        mp.remove("missing");
        assert_eq!(mp.size(), 1);
    }

    #[test]
    fn test_has_key() {
        let mut mp: HashMap<i32, &str> = HashMap::new();
        mp.insert(10, "ten");
        mp.insert(20, "twenty");

        assert!(mp.has_key(&10));
        assert!(mp.has_key(&20));
        assert!(!mp.has_key(&30));

        mp.remove(10);
        assert!(!mp.has_key(&10));
    }

    #[test]
    fn test_size() {
        let mut mp: HashMap<char, i32> = HashMap::new();
        assert_eq!(mp.size(), 0);

        mp.insert('a', 1);
        mp.insert('b', 2);
        assert_eq!(mp.size(), 2);

        mp.insert('a', 99);
        assert_eq!(mp.size(), 2);
    }

    #[test]
    fn test_is_empty() {
        let mut mp: HashMap<&str, bool> = HashMap::new();
        assert!(mp.is_empty());

        mp.insert("x", true);
        assert!(!mp.is_empty());

        mp.remove("x");
        assert!(mp.is_empty());

        mp.remove("x");
        assert!(mp.is_empty());
    }

    #[test]
    fn test_resize() {
        let mut mp: HashMap<i32, i32> = HashMap::new();

        for i in 0..20 {
            mp.insert(i, i * 10);
        }

        assert_eq!(mp.size(), 20);
        for i in 0..20 {
            assert_eq!(mp.get(&i), Some(&(i * 10)));
        }
    }
}