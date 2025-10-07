use crate::SERVER_ADDRESS;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt, TryStreamExt, future};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::{Bytes, Message, Utf8Bytes};
use tracing::{error, info};

pub async fn run_websocket_server() -> Result<()> {
    let listener = TcpListener::bind(SERVER_ADDRESS).await?;
    while let Ok((stream, _)) = listener.accept().await {
        info!("Accepted new connection");
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                error!("Connection error: {e}");
            }
        });
    }
    Ok(())
}

async fn handle_connection(stream: TcpStream) -> Result<()> {
    let addr = stream.peer_addr()?;
    info!("Connection from peer addr: {}", addr);
    let ws_stream = accept_async(stream).await?;
    info!("Upgraded to WebSocket connection: {}", addr);
    let (write, mut read) = ws_stream.split();
    // let write = Arc::new(Mutex::new(write));
    // while let Some(msg) = read.next().await {
    //     let msg = msg?;
    //     let out = match msg {
    //         Message::Text(text) => {
    //             Message::Text(text.as_str().chars().rev().collect::<String>().into())
    //         }
    //         Message::Binary(b) => Message::Binary(b.iter().copied().rev().collect()),
    //         Message::Ping(_) => Message::Text(Utf8Bytes::from_static("Ping")),
    //         Message::Pong(_) => Message::Text(Utf8Bytes::from_static("Pong")),
    //         Message::Close(_) => Message::Binary(Bytes::from_static(b"closed")),
    //         Message::Frame(_) => unreachable!(),
    //     };
    //     let mut write = write.lock().await;
    //     write.send(out).await?;
    // }
    //// message coming delayed with this approach. Was working before reverse
    read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .map_ok(|msg| match msg {
            Message::Text(text) => {
                Message::Text(text.as_str().chars().rev().collect::<String>().into())
            }
            Message::Binary(b) => Message::Binary(b.iter().copied().rev().collect()),
            _ => unreachable!(),
        })
        .forward(write)
        .await?;
    Ok(())
}
