use enigo::{Enigo, KeyboardControllable, Key};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum KeyAction {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Escape,
    AltF4,
    TaskMgr,
}

#[derive(Debug, Deserialize)]
struct KeyParams {
    key: KeyAction,
}

pub fn keyboard_handler(data: Value) -> Result<(), Box<dyn Error>> {
    let params: KeyParams = serde_json::from_value(data)?;
    println!("Handling keyboard command: {:?}", params);
    
    let mut enigo = Enigo::new();
    
    match params.key {
        KeyAction::Up => enigo.key_click(Key::UpArrow),
        KeyAction::Down => enigo.key_click(Key::DownArrow),
        KeyAction::Left => enigo.key_click(Key::LeftArrow),
        KeyAction::Right => enigo.key_click(Key::RightArrow),
        KeyAction::Enter => enigo.key_click(Key::Return),
        KeyAction::Escape => enigo.key_click(Key::Escape),
        KeyAction::AltF4 => {
            enigo.key_down(Key::Alt);
            enigo.key_click(Key::F4);
            enigo.key_up(Key::Alt);
        },
        KeyAction::TaskMgr => {
            enigo.key_down(Key::Control);
            enigo.key_down(Key::Shift);
            enigo.key_click(Key::Escape);
            enigo.key_up(Key::Shift);
            enigo.key_up(Key::Control);
        },
    }

    Ok(())
}