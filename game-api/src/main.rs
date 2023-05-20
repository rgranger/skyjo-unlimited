mod ws;

use tokio;
use ws::create_websocket_server;

#[tokio::main]
async fn main() {
    let ws_server_handle = tokio::spawn(create_websocket_server());

    ws_server_handle
        .await
        .expect("Websocket server didn't start");
}
