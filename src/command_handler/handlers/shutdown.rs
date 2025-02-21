use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ActionType {
    Immediately,
    Scheduled,
    Abort,
}

#[derive(Debug, Deserialize)]
struct ShutdownParams {
    action_type: ActionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

pub fn shutdown_handler(data: Value) -> Result<(), Box<dyn Error>> {
    let params: ShutdownParams = serde_json::from_value(data)?;
    println!("Handling shutdown command: {:?}", params);
    
    let msg = params.message.unwrap_or("via pc-remotemanager".to_string());
    match params.action_type {
        ActionType::Immediately => {
            println!("Shutting down immediately");
            Command::new("shutdown")
                .args(["/s", "/t", "0", "/c", &msg])
                .spawn()?;
        },
        ActionType::Scheduled => {
            if let Some(interval) = params.interval {
                println!("Scheduling shutdown in {} seconds", interval);
                Command::new("shutdown")
                    .args(["/s", "/t", &interval.to_string(), "/c", &msg])
                    .spawn()?;
            }
        },
        ActionType::Abort => {
            println!("Clearing scheduled shutdown");
            Command::new("shutdown")
                .args(["/a"])
                .spawn()?;
        }
    }

    Ok(())
}