use super::store::KeyValueStore;

#[derive(Clone)]
pub struct Transaction {
    pub is_dummy: bool,
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
        Self {
            size: 0,
            top: Some(Transaction {
                next: Box::from(None),
                store: Box::from(KeyValueStore::new()),
                is_dummy: true,
            }),
        }
    }

    pub fn peek(&self) -> Option<Transaction> {
        match &self.top {
            Some(t) if !t.is_dummy => Some(t.clone()),
            _ => None,
        }
    }

    pub fn get_top_mut(&mut self) -> &mut Transaction {
        self.top.as_mut().unwrap()
    }

    pub fn push_transation(&mut self) -> usize {
        let store = match &self.top {
            Some(ts) => ts.store.clone(),
            None => Box::from(KeyValueStore::new()),
        };
        let new_transaction = Transaction {
            store,
            next: Box::from(self.top.clone()),
            is_dummy: false,
        };
        self.top = Some(new_transaction);
        self.size += 1;
        self.size
    }

    pub fn pop_transation(&mut self) -> usize {
        match self.peek() {
            Some(transaction) => {
                self.top = Some(transaction.next.unwrap());
                self.size -= 1;
                self.size
            }
            _ => 0,
        }
    }
}
