mod ws_client;
mod command_handler;

use std::fs;
use serde::{Deserialize};
use toml;
use ws_client::WebSocketClient;

#[derive(Debug, Deserialize)]
struct Config {
    websocket: WebSocketConfig,
}

#[derive(Debug, Deserialize)]
struct WebSocketConfig {
    server: String,
    timeout: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config_content = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_content)?;

    // Create and connect websocket client
    let ws_client = WebSocketClient::new(config.websocket.server);
    ws_client.connect().await?;

    // Keep the main thread running
    tokio::signal::ctrl_c().await?;
    
    Ok(())
}
