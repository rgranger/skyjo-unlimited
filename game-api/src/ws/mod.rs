use std::collections::HashMap;

use futures_util::stream::{SplitSink, SplitStream};
use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use uid::IdU64;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct T(());
type Uid = IdU64<T>;

pub async fn create_websocket_server() {
    let addr = "0.0.0.0:4000";
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", &addr);

    let sockets_hash_map = HashMap::new();

    while let Ok((raw_stream, _)) = listener.accept().await {
        if let Ok(ws_stream) = tokio_tungstenite::accept_async(raw_stream).await {
            let (writer, reader) = ws_stream.split();

            sockets_hash_map.insert(Uid::new(), &writer);
            tokio::spawn(handle_connection(ws_stream));
        }
    }
}

async fn handle_connection(ws_stream: WebSocketStream<TcpStream>) {
    let (tx, rx) = mpsc::channel(32);

    tokio::spawn(handle_reader(reader, tx));
    tokio::spawn(handle_writer(writer, rx));
}

async fn handle_reader(mut reader: SplitStream<WebSocketStream<TcpStream>>, tx: Sender<Message>) {
    while let Some(Ok(message)) = reader.next().await {
        tx.send(handle_message(message)).await;
    }
}

fn handle_message(message: Message) -> Message {
    println!("{}", message);

    if let Ok("hello") = message.to_text() {
        return Message::Text("world");
    }
}

async fn handle_writer(
    mut writer: SplitSink<WebSocketStream<TcpStream>, Message>,
    mut rx: Receiver<Message>,
) {
    while let Some(message) = rx.recv().await {
        writer.send(message).await;
    }
}
