use anyhow::Result;
use tracing::Level;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use websocket_rs::client::websocket_client;
use websocket_rs::server::run_websocket_server;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    info!("Starting websocket server and client");
    let server_handle = tokio::spawn(run_websocket_server());
    websocket_client().await?;
    server_handle.await??;
    info!("Shutting down websocket server");
    Ok(())
}
