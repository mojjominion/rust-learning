use futures_util::SinkExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[rustfmt::skip]
#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let mut ws_stream = ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
        .connect()
        .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        println!("Received {:#?}", msg.as_text());
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
                        ws_stream.send(Message::text(msg)).await?;
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
