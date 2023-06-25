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
    fn get(&mut self, key: &str) -> Option<&String>;
    fn delete(&mut self, key: &str) -> Option<String>;
}

impl MutationTrait for KeyValueStore {
    fn set(&mut self, key: &str, value: &str) -> Option<String> {
        self.map.insert(key.to_string(), value.to_string())
    }
    fn get(&mut self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    fn delete(&mut self, key: &str) -> Option<String> {
        self.map.remove(key)
    }
}

#[derive(Clone)]
pub struct Transaction {
    pub store: Box<KeyValueStore>,
    pub next: Box<Option<Transaction>>,
}

#[derive(Clone)]
pub struct TransactionStack {
    top: Option<Transaction>,
    size: usize,
}

impl TransactionStack {
    pub fn new() -> Self {
        Self { size: 0, top: None }
    }

    pub fn peek(&mut self) -> Option<&mut Transaction> {
        self.top.as_mut()
    }

    pub fn push_transation(&mut self) {
        let store = match &self.top {
            Some(ts) => ts.store.clone(),
            None => Box::from(KeyValueStore::new()),
        };
        let new_transaction = Transaction {
            store,
            next: Box::from(self.top.clone()),
        };
        self.top = Some(new_transaction);
        self.size += 1;
    }

    pub fn pop_transation(&mut self) {
        match &self.top {
            Some(trans) => {
                let next = trans.next.clone().expect("Error while getting  next");
                self.top = Some(next);
                self.size -= 1;
            }
            _ => {
                // Ignore no traction case at this level
            }
        }
    }
}
