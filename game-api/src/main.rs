mod skyjo;

use futures_util::stream::{SplitSink, SplitStream};
use futures_util::SinkExt;
use futures_util::{future, stream::TryStreamExt, StreamExt};
use skyjo::Game;
use skyjo::PlayerDeck::*;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let ws_server_handle = tokio::spawn(create_websocket_server());

    let game = Game::new(3);

    //println!("{:?}", game);

    //let game = game.start();
    //let card = game.players[0].reveal(Slot_1_1);
    //card = game.players[0].reveal(Slot_1_2);
    //card = game.players[1].reveal(Slot_2_2);
    //card = game.players[1].reveal(Slot_3_3);
    //card = game.players[2].reveal(Slot_1_2);
    //card = game.players[2].reveal(Slot_1_1);

    ws_server_handle.await;
}

async fn create_websocket_server() {
    let addr = "0.0.0.0:4000";
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", &addr);

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(raw_stream: TcpStream) {
    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");

    let (mut writer, reader) = ws_stream.split();

    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(handle_reader(reader, tx));
    tokio::spawn(handle_writer(writer, rx));
}

async fn handle_reader(mut reader: SplitStream<WebSocketStream<TcpStream>>, tx: Sender<Command>) {
    while let Some(Ok(message)) = reader.next().await {
        println!("{}", message);
        if let Ok("hello") = message.to_text() {
            tx.send(Command::Hello).await;
        }
    }
}

enum Command {
    Hello,
}

async fn handle_writer(
    mut writer: SplitSink<WebSocketStream<TcpStream>, Message>,
    mut rx: Receiver<Command>,
) {
    while let Some(command) = rx.recv().await {
        match command {
            Command::Hello => {
                writer.send(Message::Text("world".to_string())).await;
            }
        }
    }
}
