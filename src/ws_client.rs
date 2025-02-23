use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, tungstenite::Error as WsError};
use futures_util::StreamExt;
use url::Url;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::command_handler::CommandRouter;
use tokio::time::{sleep, Duration};

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

enum ClientError {
    ConnectionError(Box<dyn std::error::Error>),
    RuntimeError(Box<dyn std::error::Error>),
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

    async fn try_connect(&self) -> Result<(), ClientError> {
        let url = Url::parse(&self.server_url)
            .map_err(|e| ClientError::RuntimeError(Box::new(e)))?;
        
        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| match e {
                WsError::Http(_) | WsError::Io(_) | WsError::Protocol(_) => ClientError::ConnectionError(Box::new(e)),
                _ => ClientError::RuntimeError(Box::new(e))
            })?;
            
        let (mut write, mut read) = ws_stream.split();
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
                    Err(e) => {
                        eprintln!("WebSocket error: {}", e);
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        let max_retry_delay = Duration::from_secs(30);
        let mut retry_delay = Duration::from_secs(1);

        loop {
            println!("Attempting to connect to WebSocket server...");
            match self.try_connect().await {
                Ok(_) => {
                    println!("Successfully connected to WebSocket server");
                    return Ok(());
                },
                Err(ClientError::ConnectionError(e)) => {
                    eprintln!("Connection failed: {}. Retrying in {} seconds...", e, retry_delay.as_secs());
                    sleep(retry_delay).await;
                    retry_delay = std::cmp::min(retry_delay * 2, max_retry_delay);
                },
                Err(ClientError::RuntimeError(e)) => {
                    return Err(e);
                }
            }
        }
    }
}
