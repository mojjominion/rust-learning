use std::collections::HashMap;

#[allow(unreachable_code, unused, dead_code)]

type TStore = HashMap<String, String>;

#[derive(Clone)]
pub struct KeyValueStore {
    pub map: TStore,
}

impl KeyValueStore {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn get_count(&self) -> usize {
        self.map.len()
    }
}

pub trait MutationTrait {
    fn set(&mut self, key: &str, value: &str) -> Option<String>;
    fn get(&mut self, key: &str) -> Option<String>;
    fn delete(&mut self, key: &str) -> Option<String>;
    fn get_all(&self) -> Vec<(String, String)>;
}

impl MutationTrait for KeyValueStore {
    fn set(&mut self, key: &str, value: &str) -> Option<String> {
        self.map.insert(key.to_string(), value.to_string())
    }
    fn get(&mut self, key: &str) -> Option<String> {
        self.map.get(key).cloned()
    }
    fn get_all(&self) -> Vec<(String, String)> {
        let res: Vec<_> = self
            .map
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();
        res
    }

    fn delete(&mut self, key: &str) -> Option<String> {
        self.map.remove(key)
    }
}
