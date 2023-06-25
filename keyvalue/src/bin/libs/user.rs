use crate::cmds::store::{KeyValueStore, TransactionStack};

#[derive(Clone)]
pub(crate) struct User {
    pub id: usize,
    pub global_store: Box<KeyValueStore>,
    pub transaction_stack: Box<TransactionStack>,
}

impl User {
    pub fn new(
        id: usize,
        transaction_stack: Box<TransactionStack>,
        global_store: Box<KeyValueStore>,
    ) -> Self {
        Self {
            id,
            transaction_stack,
            global_store,
        }
    }

    pub fn get_state(&mut self) -> (&mut TransactionStack, &mut KeyValueStore) {
        (self.transaction_stack.as_mut(), self.global_store.as_mut())
    }
}
