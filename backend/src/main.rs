use enigo::{Enigo, MouseControllable};
use serde::Deserialize;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tungstenite::{stream, Message};
use futures_util::{StreamExt, SinkExt};

#[derive(Deserialize)]
struct Move {
    dx: i32,
    dy: i32,
}



#[tokio::main]
async fn main() {
    
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("server running on port 8080");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            let (mut ws_sender, mut ws_receiver) = ws_stream.split();
            let mut enigo = Enigo::new();

            while let Some(msg) = ws_receiver.next().await {
                if let Ok(Message::Text(text)) = msg {
                    if let Ok(parsed) = serde_json::from_str::<Move>(&text) {
                        enigo.mouse_move_relative(parsed.dx, parsed.dy);
                    }
                }
            }
        });
    }

}
