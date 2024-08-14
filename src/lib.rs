use std::collections::HashMap;
pub struct KvStore {
    store: HashMap<String, String>,
}
impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            store: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }
    pub fn get(&self, key: String) -> Option<String> {
        if let Some(value) = self.store.get(&key) {
            return Some(value.clone());
        } else {
            return None;
        }
    }
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
