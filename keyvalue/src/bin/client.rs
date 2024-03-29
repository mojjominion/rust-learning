use futures_util::SinkExt;
use http::Uri;
use std::{
    io::{self, Write},
    process,
};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpStream,
};
use tokio_websockets::{ClientBuilder, Error, MaybeTlsStream, Message, WebsocketStream};

static EXIT: &str = "EXIT";
fn print_cli_interface() {
    print!("cmd:> ");
    io::stdout().flush().unwrap();
}

pub async fn run(mut ws_stream: WebsocketStream<MaybeTlsStream<TcpStream>>) -> Result<(), Error> {
    let mut stdin = BufReader::new(tokio::io::stdin()).lines();
    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        println!("{}", msg.as_text().unwrap());
                        print_cli_interface();
                    },
                    Some(Err(err)) => {
                        println!("Error {:#?}", err);
                    },
                    None => {}
                }
            },
            line = stdin.next_line() => {
                match line {
                    Ok(Some(msg)) => {
                        if msg.to_uppercase() == EXIT {
                            process::exit(0);
                        }
                        ws_stream.send(Message::text(msg)).await?;
                        print_cli_interface();
                    },
                    Err(err) => {
                        println!("Error while reading stdin {:#?}", err);
                    },
                    _ => {}
                }
            }
        }
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let ws_stream = ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
        .connect()
        .await?;
    run(ws_stream).await
}
