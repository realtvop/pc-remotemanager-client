use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::StreamExt;
use url::Url;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::command_handler::CommandRouter;

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
    router: Arc<CommandRouter>,
}

impl WebSocketClient {
    pub fn new(server_url: String) -> Self {
        let mut router = CommandRouter::new();
        router.register_default_handlers();
        Self {
            server_url,
            router: Arc::new(router),
        }
    }

    pub async fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = Url::parse(&self.server_url)?;
        let (ws_stream, _) = connect_async(url).await?;
        let (_write, mut read) = ws_stream.split();
        let router = self.router.clone();

        tokio::spawn(async move {
            while let Some(message) = read.next().await {
                match message {
                    Ok(msg) => {
                        if let Message::Text(text) = msg {
                            match serde_json::from_str::<WsMessage>(&text) {
                                Ok(ws_msg) => {
                                    if let Err(e) = router.handle(&ws_msg.message_type, ws_msg.data) {
                                        eprintln!("Handler error: {}", e);
                                    }
                                },
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
