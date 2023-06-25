pub mod cmds;
pub mod libs;

use std::{error::Error, str::FromStr};

use cmds::cmd_types::CMD;
use futures_util::sink::SinkExt;
use libs::user::User;
use tokio::net::{TcpListener, TcpStream};
use tokio_websockets::{Message, ServerBuilder, WebsocketStream};

use crate::cmds::{
    ops::{
        execute_begin, execute_commit, execute_count, execute_delete, execute_end, execute_get,
        execute_rollback, execute_set,
    },
    store::{KeyValueStore, TransactionStack},
};

async fn transation_handler(client: &mut User, cmd: &str) -> Result<String, String> {
    let args: Vec<_> = cmd.split(" ").collect();
    let operation = CMD::from_str(args[0])?;
    let (transaction_stack, global_store) = client.get_state();

    let operation_result = match operation {
        CMD::BEGIN => {
            println!("Process {:?}", operation);
            execute_begin(transaction_stack)
        }
        CMD::SET => {
            println!("Process {:?}", operation);
            execute_set(cmd, transaction_stack)
        }
        CMD::GET => {
            println!("Process {:?}", operation);
            execute_get(cmd, transaction_stack)
        }
        CMD::COUNT => {
            println!("Process {:?}", operation);
            execute_count(cmd, transaction_stack)
        }
        CMD::DELETE => {
            println!("Process {:?}", operation);
            execute_delete(cmd, transaction_stack)
        }
        CMD::COMMIT => {
            println!("Process {:?}", operation);
            execute_commit(transaction_stack, global_store)
        }
        CMD::ROLLBACK => {
            println!("Process {:?}", operation);
            execute_rollback(transaction_stack)
        }
        CMD::END => {
            println!("Process {:?}", operation);
            execute_end(transaction_stack)
        }
    };

    Ok(String::from(operation_result))
}

async fn handle_connection(
    client: &mut User,
    mut ws_stream: WebsocketStream<TcpStream>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    ws_stream
        .send(Message::text("Welcome to the KeyValueStore".into()))
        .await?;

    loop {
        tokio::select! {
         msg = ws_stream.next() => {
            if let Some(Ok(value)) = msg {
                let result = transation_handler(client, value.as_text().unwrap()).await;

                match result {
                    Ok(res) => ws_stream.send(Message::text(res).into()).await?,
                    Err(err) => ws_stream.send(Message::text(err).into()).await?
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

    loop {
        let (socket, addr) = listener.accept().await?;

        println!("New connection from {addr:?} {:?}", client_count);

        let mut client = User::new(
            client_count,
            Box::new(TransactionStack::new()),
            Box::new(KeyValueStore::new()),
        );

        client_count += 1;

        // let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let ws_stream = ServerBuilder::new().accept(socket).await?;

            handle_connection(&mut client, ws_stream).await
        });
    }
}
