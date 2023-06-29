use std::sync::Arc;

use tokio::sync::Mutex;

use super::store::{KeyValueStore, MutationTrait};

pub(crate) struct GlobalStore {
    pub inner: Arc<Mutex<KeyValueStore>>,
}

impl GlobalStore {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(KeyValueStore::new())),
        }
    }

    pub async fn set(&self, key: &str, new_value: &str) -> String {
        let mut lock = self.inner.lock().await;
        let result = lock.set(key, new_value);

        match result {
            Some(old_value) => String::from(format!(
                "Global Store:: The value for {key:?} is updated from {old_value:?} to {new_value}"
            )),
            None => String::from(format!(
                "Global Store:: The value for {key:?} is set to {new_value:?}"
            )),
        }
    }

    pub async fn get(&self, key: &str) -> String {
        let mut lock = self.inner.lock().await;
        let result = lock.get(key);

        match result {
            Some(value) => value.to_string(),
            None => String::from(format!("Global Store:: No entry found for key {key:?}")),
        }
    }

    pub async fn delete(&self, key: &str) -> String {
        let mut lock = self.inner.lock().await;
        let result = lock.delete(key);

        match result {
            Some(value) => String::from(format!(
                "Global Store:: The entry [ {key:?}:{value:?} ] is deleted"
            )),
            None => String::from("Global Store:: Error while deleting entry for key {key:?}"),
        }
    }

    fn get_all(&self) -> Vec<(&String, &String)> {
        todo!()
    }
}
