use crate::SERVER_ADDRESS;
use anyhow::Result;
use futures_util::{StreamExt, TryStreamExt, future};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
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
    let (write, read) = ws_stream.split();
    // message coming delayed once try return after reverse
    read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .forward(write)
        .await?;
    Ok(())
}
