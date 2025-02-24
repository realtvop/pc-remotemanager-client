use std::collections::HashMap;
use serde_json::Value;
mod handlers;

pub type HandlerFn = Box<dyn Fn(Value) -> Result<(), Box<dyn std::error::Error>> + Send + Sync>;

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
        self.register("shutdown", handlers::shutdown_handler);
        self.register("key", handlers::keyboard_handler);
        self.register("input", handlers::input_handler);
    }
}
