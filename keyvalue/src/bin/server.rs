pub mod cmds;
pub mod libs;

use std::{error::Error, str::FromStr, sync::Arc};

use cmds::{cmd_types::CMD, global_store::GlobalStore};
use futures_util::sink::SinkExt;
use libs::user::User;
use tokio::net::{TcpListener, TcpStream};
use tokio_websockets::{Message, ServerBuilder, WebsocketStream};

use crate::cmds::{
    ops::{
        execute_begin, execute_commit, execute_count, execute_delete, execute_end, execute_get,
        execute_rollback, execute_set,
    },
    transactions::TransactionStack,
};

async fn transation_handler(
    client: &mut User,
    cmd: &str,
    gs: Arc<GlobalStore>,
) -> Result<String, String> {
    let transaction_stack = client.get_state();
    let args: Vec<_> = cmd.split(' ').collect();
    let operation = CMD::from_str(args[0])?;
    match operation {
        CMD::BEGIN => {
            println!("Process {:?}", args);
            execute_begin(transaction_stack)
        }
        CMD::SET => {
            println!("Process {:?}", args);
            execute_set(cmd, transaction_stack, gs).await
        }
        CMD::GET => {
            println!("Process {:?}", args);
            execute_get(cmd, transaction_stack, gs).await
        }
        CMD::COUNT => {
            println!("Process {:?}", args);
            execute_count(cmd, transaction_stack)
        }
        CMD::DELETE => {
            println!("Process {:?}", args);
            execute_delete(cmd, transaction_stack, gs).await
        }
        CMD::COMMIT => {
            println!("Process {:?}", args);
            execute_commit(transaction_stack, gs).await
        }
        CMD::ROLLBACK => {
            println!("Process {:?}", args);
            execute_rollback(transaction_stack)
        }
        CMD::END => {
            println!("Process {:?}", args);
            execute_end(transaction_stack)
        }
    }
}

async fn handle_connection(
    client: &mut User,
    mut ws_stream: WebsocketStream<TcpStream>,
    gs: Arc<GlobalStore>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    ws_stream
        .send(Message::text("Welcome to the KeyValueStore".into()))
        .await?;
    loop {
        let gs = Arc::clone(&gs);
        tokio::select! {
         msg = ws_stream.next() => {
            if let Some(Ok(value)) = msg {
                let result = transation_handler(client, value.as_text().unwrap(), gs).await;
                match result {
                    Ok(res) => ws_stream.send(Message::text(res)).await?,
                    Err(err) => ws_stream.send(Message::text(err)).await?
                };
            }
         }
        }
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // let (bcast_tx, _) = channel(1000);
    let listener = TcpListener::bind("127.0.0.1:2000").await?;
    println!("listening on port 2000");
    let mut client_count = 1;
    let global_store = Arc::from(GlobalStore::new());
    loop {
        let (socket, addr) = listener.accept().await?;
        let mut client = User::new(client_count, Box::new(TransactionStack::new()));
        let gs = Arc::clone(&global_store);
        println!("New connection from {addr:?} {:?}", client.id);
        client_count += 1;
        // let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let ws_stream = ServerBuilder::new().accept(socket).await?;
            handle_connection(&mut client, ws_stream, gs).await
        });
    }
}
