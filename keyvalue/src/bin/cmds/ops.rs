#![allow(dead_code)]

use std::sync::Arc;

use crate::cmds::store::MutationTrait;

use super::store::{GlobalStore, KeyValueStore, TransactionStack};

#[derive(Debug)]
struct AppArgs(String);

static BAD_ARGS: &str = "BAD ARGS";
static NO_ACTIVE_TRANSACTION: &str = "NO ACTIVE TRANSACTION";
static TRANSACTION_BEGUN: &str = "TRANSACTION STARTED";
static TRANSACTION_ENDED: &str = "TRANSACTION ENDED";
static TRANSACTION_COMMITED: &str = "TRANSACTION COMMITED";
static TRANSACTION_ROLLED: &str = "TRANSACTION ROLLED";

pub(crate) fn execute_begin(transactions: &mut TransactionStack) -> String {
    let size = transactions.push_transation();
    format!("{}, Total Active transactions: {}", TRANSACTION_BEGUN, size)
}

pub(crate) fn execute_end(transactions: &mut TransactionStack) -> String {
    match transactions.peek() {
        Some(_) => {
            let size = transactions.pop_transation();
            format!("{}, Total Active transactions: {}", TRANSACTION_ENDED, size)
        }
        None => NO_ACTIVE_TRANSACTION.to_string(),
    }
}

pub(crate) async fn execute_commit(
    transactions: &mut TransactionStack,
    global_store: Arc<GlobalStore>,
) -> String {
    match transactions.peek() {
        Some(transaction) => {
            for (key, value) in transaction.store.map {
                global_store.set(key.as_str(), value.as_str()).await;
            }
            let size = transactions.pop_transation();
            format!(
                "{}, Total Active transactions: {}",
                TRANSACTION_COMMITED, size
            )
        }
        _ => NO_ACTIVE_TRANSACTION.to_string(),
    }
}

pub(crate) fn execute_rollback(transactions: &mut TransactionStack) -> String {
    match transactions.peek() {
        Some(_) => {
            let size = transactions.pop_transation();
            format!(
                "{}, Total Active transactions: {}",
                TRANSACTION_ROLLED, size
            )
        }
        None => NO_ACTIVE_TRANSACTION.to_string(),
    }
}

async fn set_to_global(key: &str, new_value: &str, global_store: Arc<GlobalStore>) -> String {
    let result = global_store.set(key, new_value).await;

    match result {
        Some(old_value) => String::from(format!(
            "Global Store:: The value for {key:?} is updated from {old_value:?} to {new_value}"
        )),
        None => String::from(format!(
            "Global Store:: The value for {key:?} is set to {new_value:?}"
        )),
    }
}

pub(crate) async fn execute_set(
    cmd: &str,
    transactions: &mut TransactionStack,
    global_store: Arc<GlobalStore>,
) -> String {
    let args: Vec<_> = cmd.split(" ").collect();

    match args[1..] {
        [key, new_value] => match transactions.peek() {
            Some(_) => {
                let result = transactions.get_top_mut().store.set(key, new_value);
                match result {
                    // If key did exist
                    Some(old_value) => String::from(format!(
                        "The value for {key:?} is updated from {old_value:?} to {new_value}"
                    )),
                    // If key didn't exist
                    None => String::from(format!("The value for {key:?} is set to {new_value:?}")),
                }
            }
            None => set_to_global(key, new_value, global_store).await,
        },
        _ => BAD_ARGS.to_string(),
    }
}

async fn get_from_global(key: &str, global_store: Arc<GlobalStore>) -> String {
    let result = global_store.get(key).await;
    match result {
        Some(value) => value.to_string(),
        _ => String::from(format!("No entry found for key {key:?}")),
    }
}

pub(crate) async fn execute_get(
    cmd: &str,
    transactions: &mut TransactionStack,
    global_store: Arc<GlobalStore>,
) -> String {
    let args: Vec<_> = cmd.split(" ").collect();

    match args[1..] {
        [key, ..] => match transactions.peek() {
            Some(mut transaction) => match transaction.store.get(key) {
                Some(value) => value.to_string(),
                _ => get_from_global(key, global_store).await,
            },
            _ => get_from_global(key, global_store).await,
        },
        _ => BAD_ARGS.to_string(),
    }
}

pub(crate) fn execute_count(cmd: &str, transactions: &mut TransactionStack) -> String {
    let args: Vec<_> = cmd.split(" ").collect();

    match args[1..] {
        [key, ..] => match transactions.peek() {
            Some(transaction) => {
                let result = transaction.store.get_count();
                String::from(format!("Len for {key:?} is {:?}", result.to_string()))
            }
            _ => NO_ACTIVE_TRANSACTION.to_string(),
        },
        _ => BAD_ARGS.to_string(),
    }
}

async fn delete_from_global(key: &str, global_store: Arc<GlobalStore>) -> String {
    let result = global_store.delete(key).await;
    match result {
        Some(value) => String::from(format!(
            "Global Store:: The entry [ {key:?}:{value:?} ] is deleted"
        )),
        None => String::from("Global Store:: Error while deleting entry for key {key:?}"),
    }
}

pub(crate) async fn execute_delete(
    cmd: &str,
    transactions: &mut TransactionStack,
    global_store: Arc<GlobalStore>,
) -> String {
    let args: Vec<_> = cmd.split(" ").collect();

    match args[1..] {
        [key, ..] => match transactions.peek() {
            Some(_) => {
                let result = transactions.get_top_mut().store.delete(key);
                match result {
                    Some(value) => {
                        String::from(format!("The entry [ {key:?}:{value:?} ] is deleted"))
                    }
                    _ => String::from("Error while deleting entry for key {key:?}"),
                }
            }
            None => delete_from_global(key, global_store).await,
        },
        _ => BAD_ARGS.to_string(),
    }
}
