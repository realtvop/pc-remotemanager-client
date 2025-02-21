use std::collections::HashMap;
use serde_json::Value;

pub type HandlerFn = Box<dyn Fn(Value) -> Result<(), Box<dyn std::error::Error>> + Send + Sync>;

pub mod handlers {
    use super::*;

    pub fn handle_abc(data: Value) -> Result<(), Box<dyn std::error::Error>> {
        println!("Handling ABC command with data: {:?}", data);
        Ok(())
    }
}

pub struct CommandRouter {
    handlers: HashMap<String, HandlerFn>,
}

impl CommandRouter {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new()
        }
    }

    pub fn register<F>(&mut self, command_type: &str, handler: F)
    where
        F: Fn(Value) -> Result<(), Box<dyn std::error::Error>> + Send + Sync + 'static
    {
        self.handlers.insert(command_type.to_string(), Box::new(handler));
    }

    pub fn handle(&self, command_type: &str, data: Value) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(handler) = self.handlers.get(command_type) {
            handler(data)
        } else {
            Err("No handler registered for this command type".into())
        }
    }

    pub fn register_default_handlers(&mut self) {
        self.register("abc", handlers::handle_abc);
    }
}
