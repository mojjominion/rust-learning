#![allow(dead_code)]

use std::sync::Arc;

use super::{global_store::GlobalStore, store::MutationTrait, transactions::TransactionStack};

#[derive(Debug)]
struct AppArgs(String);

static BAD_ARGS: &str = "BAD ARGS";
static NO_ACTIVE_TRANSACTION: &str = "NO ACTIVE TRANSACTION";
static TRANSACTION_BEGUN: &str = "TRANSACTION STARTED";
static TRANSACTION_ENDED: &str = "TRANSACTION ENDED";
static TRANSACTION_COMMITED: &str = "TRANSACTION COMMITED";
static TRANSACTION_ROLLED: &str = "TRANSACTION ROLLED";

pub(crate) fn execute_begin(transactions: &mut TransactionStack) -> Result<String, String> {
    let size = transactions.push_transation();
    Ok(format!(
        "{}, Total Active transactions: {}",
        TRANSACTION_BEGUN, size
    ))
}

pub(crate) fn execute_end(transactions: &mut TransactionStack) -> Result<String, String> {
    match transactions.peek() {
        Some(_) => {
            let size = transactions.pop_transation();
            Ok(format!(
                "{}, Total Active transactions: {}",
                TRANSACTION_ENDED, size
            ))
        }
        None => Err(NO_ACTIVE_TRANSACTION.to_string()),
    }
}

pub(crate) async fn execute_commit(
    transactions: &mut TransactionStack,
    global_store: Arc<GlobalStore>,
) -> Result<String, String> {
    match transactions.peek() {
        Some(transaction) => {
            global_store.set_multi(transaction.store.map).await;
            let size = transactions.pop_transation();
            Ok(format!(
                "{}, Total Active transactions: {}",
                TRANSACTION_COMMITED, size
            ))
        }
        _ => Err(NO_ACTIVE_TRANSACTION.to_string()),
    }
}

pub(crate) fn execute_rollback(transactions: &mut TransactionStack) -> Result<String, String> {
    match transactions.peek() {
        Some(_) => {
            let size = transactions.pop_transation();
            Ok(format!(
                "{}, Total Active transactions: {}",
                TRANSACTION_ROLLED, size
            ))
        }
        None => Err(NO_ACTIVE_TRANSACTION.to_string()),
    }
}

pub(crate) async fn execute_set(
    cmd: &str,
    transactions: &mut TransactionStack,
    global_store: Arc<GlobalStore>,
) -> Result<String, String> {
    let args: Vec<_> = cmd.split(' ').collect();

    match args[1..] {
        [key, new_value] => match transactions.peek() {
            Some(_) => {
                let result = transactions.get_top_mut().store.set(key, new_value);
                match result {
                    // If key did exist
                    Some(old_value) => Ok(format!(
                        "The value for {key:?} is updated from {old_value:?} to {new_value}"
                    )),
                    // If key didn't exist
                    None => Err(format!("The value for {key:?} is set to {new_value:?}")),
                }
            }
            None => global_store.set(key, new_value).await,
        },
        _ => Err(BAD_ARGS.to_string()),
    }
}

pub(crate) async fn execute_get(
    cmd: &str,
    transactions: &mut TransactionStack,
    global_store: Arc<GlobalStore>,
) -> Result<String, String> {
    let args: Vec<_> = cmd.split(' ').collect();

    match args[1..] {
        [key, ..] => match transactions.peek() {
            Some(mut transaction) => match transaction.store.get(key) {
                Some(value) => Ok(value),
                _ => global_store.get(key).await,
            },
            _ => global_store.get(key).await,
        },
        _ => Err(BAD_ARGS.to_string()),
    }
}

pub(crate) fn execute_count(
    cmd: &str,
    transactions: &mut TransactionStack,
) -> Result<String, String> {
    let args: Vec<_> = cmd.split(' ').collect();

    match args[1..] {
        [key, ..] => match transactions.peek() {
            Some(transaction) => {
                let result = transaction.store.get_count();
                Ok(format!("Len for {key:?} is {:?}", result.to_string()))
            }
            _ => Err(NO_ACTIVE_TRANSACTION.to_string()),
        },
        _ => Err(BAD_ARGS.to_string()),
    }
}

pub(crate) async fn execute_delete(
    cmd: &str,
    transactions: &mut TransactionStack,
    global_store: Arc<GlobalStore>,
) -> Result<String, String> {
    let args: Vec<_> = cmd.split(' ').collect();

    match args[1..] {
        [key, ..] => match transactions.peek() {
            Some(_) => {
                let result = transactions.get_top_mut().store.delete(key);
                match result {
                    Some(value) => Ok(format!("The entry [ {key:?}:{value:?} ] is deleted")),
                    _ => Ok("Error while deleting entry for key {key:?}".to_string()),
                }
            }
            None => global_store.delete(key).await,
        },
        _ => Err(BAD_ARGS.to_string()),
    }
}
