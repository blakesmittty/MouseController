use enigo::{Enigo, MouseControllable};
use serde::Deserialize;
use tungstenite::{stream, Message};
use futures_util::{StreamExt, SinkExt};
use warp::{filters::ws::WebSocket, Filter};
//use std::{convert::Infallible, os::linux::raw::stat};

#[derive(Deserialize)]
struct Move {
    dx: i32,
    dy: i32,
}


pub async fn ws_handler(ws: WebSocket) {
    let (mut tx, mut rx) = ws.split();
    let mut enigo = Enigo::new();

    while let Some(Ok(msg)) = rx.next().await {
        if msg.is_text() {
            if let Ok(parsed) = serde_json::from_str::<Move>(msg.to_str().unwrap()) {
                enigo.mouse_move_relative(parsed.dx, parsed.dy);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // serving the static files for the touchpad
    let static_files = warp::fs::dir("static/");
    // ws route for input
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(ws_handler)
        })
        .boxed();

        let routes = ws_route.boxed().or(static_files.boxed());

    println!("Server running on http://localhost:8080");
    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080))
        .await;

}
