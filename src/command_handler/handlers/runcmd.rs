use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct CommandParams {
    command: String,
}

pub fn command_handler(data: Value) -> Result<(), Box<dyn Error>> {
    let params: CommandParams = serde_json::from_value(data)?;
    println!("Handling command: {:?}", params);
    
    Command::new(params.command).spawn()?;

    Ok(())
}
