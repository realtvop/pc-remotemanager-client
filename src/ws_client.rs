use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use url::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WsMessage {
    #[serde(rename = "type")]
    message_type: String,
    task_id: i64,
    data: serde_json::Value,
}

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
                    Ok(msg) => {
                        if let Message::Text(text) = msg {
                            match serde_json::from_str::<WsMessage>(&text) {
                                Ok(ws_msg) => println!("Parsed message: {:?}", ws_msg),
                                Err(e) => eprintln!("Failed to parse message: {}", e),
                            }
                        }
                    },
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        });

        Ok(())
    }
}
