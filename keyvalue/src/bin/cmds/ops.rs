#![allow(dead_code)]

use crate::cmds::store::MutationTrait;

use super::store::{KeyValueStore, TransactionStack};

#[derive(Debug)]
struct AppArgs(String);

static NO_ACTIVE_TRANSACTION: &str = "NO ACTIVE TRANSACTION";
static BAD_ARGS: &str = "BAD ARGS";
static TRANSACTION_BEGUN: &str = "TRANSACTION STARTED";
static TRANSACTION_ENDED: &str = "TRANSACTION ENDED";
static TRANSACTION_COMMITED: &str = "TRANSACTION COMMITED";
static TRANSACTION_ROLLED: &str = "TRANSACTION ROLLED";

pub(crate) fn execute_begin(transactions: &mut TransactionStack) -> String {
    transactions.push_transation();
    TRANSACTION_BEGUN.to_string()
}

pub(crate) fn execute_end(transactions: &mut TransactionStack) -> String {
    transactions.pop_transation();
    TRANSACTION_ENDED.to_string()
}

pub(crate) fn execute_commit(
    transactions: &mut TransactionStack,
    global_store: &mut KeyValueStore,
) -> String {
    match transactions.peek() {
        Some(transaction) => {
            let result = transaction.store.map.clone();
            for (key, value) in result {
                global_store.set(key.as_str(), value.as_str());
            }
            transactions.pop_transation();
            TRANSACTION_COMMITED.to_string()
        }
        None => NO_ACTIVE_TRANSACTION.to_string(),
    }
}

pub(crate) fn execute_rollback(transactions: &mut TransactionStack) -> String {
    transactions.pop_transation();
    TRANSACTION_ROLLED.to_string()
}

pub(crate) fn execute_set(cmd: &str, transactions: &mut TransactionStack) -> String {
    let args: Vec<_> = cmd.split(" ").collect();
    println!("{}", cmd.to_string());

    match args[1..] {
        [key, new_value] => match transactions.peek() {
            Some(transaction) => {
                let result = transaction.store.set(key, new_value);

                match result {
                    // If key did exist
                    Some(old_value) => String::from(format!(
                        "The value for {key:?} is updated from {old_value:?} to {new_value}"
                    )),
                    // If key didn't exist
                    None => String::from(format!("The value for {key:?} is set to {new_value:?}")),
                }
            }
            None => NO_ACTIVE_TRANSACTION.to_string(),
        },
        _ => BAD_ARGS.to_string(),
    }
}

pub(crate) fn execute_get(cmd: &str, transactions: &mut TransactionStack) -> String {
    let args: Vec<_> = cmd.split(" ").collect();

    match args[1..] {
        [key, ..] => match transactions.peek() {
            Some(transaction) => {
                let result = transaction.store.get(key);

                match result {
                    Some(value) => String::from(value),
                    _ => String::from(format!("Error while getting entry for key {key:?}")),
                }
            }
            _ => NO_ACTIVE_TRANSACTION.to_string(),
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

pub(crate) fn execute_delete(cmd: &str, transactions: &mut TransactionStack) -> String {
    let args: Vec<_> = cmd.split(" ").collect();

    match args[1..] {
        [key, ..] => match transactions.peek() {
            Some(transaction) => {
                let result = transaction.store.delete(key);
                match result {
                    Some(value) => String::from(value),
                    _ => String::from("Error while deleting entry for key {key:?}"),
                }
            }
            _ => NO_ACTIVE_TRANSACTION.to_string(),
        },
        _ => BAD_ARGS.to_string(),
    }
}
