use std::collections::HashMap;

/// KvStore as a memory store struct
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// constructor, init the hashmap of kvstore
    pub fn new() -> Self {
        println!("new kv store");
        Self {
            map: HashMap::new(),
        }
    }

    /// set map[key]=value; if key in map, then overwrite
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// get map[key]; if key not in map, then None
    pub fn get(&self, key: String) -> Option<String> {
        if self.map.contains_key(&key) {
            return Some(self.map.get(&key).unwrap().clone());
        } else {
            return None;
        }
    }

    /// remove k-v pair from map
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
