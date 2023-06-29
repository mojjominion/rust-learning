use crate::cmds::store::TransactionStack;

#[derive(Clone)]
pub(crate) struct User {
    pub id: usize,
    pub transaction_stack: Box<TransactionStack>,
}

impl User {
    pub fn new(id: usize, transaction_stack: Box<TransactionStack>) -> Self {
        Self {
            id,
            transaction_stack,
        }
    }

    pub fn get_state(&mut self) -> (&mut TransactionStack) {
        self.transaction_stack.as_mut()
    }
}
