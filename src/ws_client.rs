use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use url::Url;

pub struct WebSocketClient {
    server_url: String,
}

impl WebSocketClient {
    pub fn new(server_url: String) -> Self {
        Self { server_url }
    }

    pub async fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = Url::parse(&self.server_url)?;
        let (ws_stream, _) = connect_async(url).await?;
        let (mut write, mut read) = ws_stream.split();

        // Handle incoming messages
        tokio::spawn(async move {
            while let Some(message) = read.next().await {
                match message {
                    Ok(msg) => println!("Received: {}", msg),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        });

        Ok(())
    }
}
