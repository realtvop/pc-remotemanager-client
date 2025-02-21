use serde_json::Value;
use std::error::Error;

pub fn shutdown_handler(data: Value) -> Result<(), Box<dyn Error>> {
    println!("Handling shutdown command");
    Ok(())
}