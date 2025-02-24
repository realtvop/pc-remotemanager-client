use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Keyboard, Key, Settings,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum KeyAction {
    // single key
    Up,
    Down,
    Left,
    Right,
    Enter,
    Escape,
    VolumeUp,
    VolumeDown,
    VolumeMute,
    PlayPause,
    // hotkey
    AltF4,
    TaskMgr,
    VirtualDesktopLeft,
    VirtualDesktopRight,
    WinD,
}

#[derive(Debug, Deserialize)]
struct KeyParams {
    key: KeyAction,
}
#[derive(Debug, Deserialize)]
struct InputParams {
    text: str,
}

pub fn keyboard_handler(data: Value) -> Result<(), Box<dyn Error>> {
    let params: KeyParams = serde_json::from_value(data)?;
    println!("Handling keyboard command: {:?}", params);
    
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    
    match params.key {
        KeyAction::Up => enigo.key(Key::UpArrow, Click).unwrap(),
        KeyAction::Down => enigo.key(Key::DownArrow, Click).unwrap(),
        KeyAction::Left => enigo.key(Key::LeftArrow, Click).unwrap(),
        KeyAction::Right => enigo.key(Key::RightArrow, Click).unwrap(),
        KeyAction::Enter => enigo.key(Key::Return, Click).unwrap(),
        KeyAction::Escape => enigo.key(Key::Escape, Click).unwrap(),
        KeyAction::VolumeUp => enigo.key(Key::VolumeUp, Click).unwrap(),
        KeyAction::VolumeDown => enigo.key(Key::VolumeDown, Click).unwrap(),
        KeyAction::VolumeMute => enigo.key(Key::VolumeMute, Click).unwrap(),
        KeyAction::PlayPause => enigo.key(Key::MediaPlayPause, Click).unwrap(),
        KeyAction::AltF4 => {
            enigo.key(Key::Alt, Press).unwrap();
            enigo.key(Key::F4, Click).unwrap();
            enigo.key(Key::Alt, Release).unwrap();
        },
        KeyAction::WinD => {
            enigo.key(Key::Meta, Press).unwrap();
            enigo.key(Key::Unicode('d'), Click).unwrap();
            enigo.key(Key::Meta, Release).unwrap();
        },
        KeyAction::TaskMgr => {
            enigo.key(Key::Control, Press).unwrap();
            enigo.key(Key::Shift, Press).unwrap();
            enigo.key(Key::Escape, Click).unwrap();
            enigo.key(Key::Shift, Release).unwrap();
            enigo.key(Key::Control, Release).unwrap();
        },
        KeyAction::VirtualDesktopLeft => {
            enigo.key(Key::Control, Press).unwrap();
            enigo.key(Key::Meta, Press).unwrap();
            enigo.key(Key::LeftArrow, Click).unwrap();
            enigo.key(Key::Meta, Release).unwrap();
            enigo.key(Key::Control, Release).unwrap();
        },
        KeyAction::VirtualDesktopRight => {
            enigo.key(Key::Control, Press).unwrap();
            enigo.key(Key::Meta, Press).unwrap();
            enigo.key(Key::RightArrow, Click).unwrap();
            enigo.key(Key::Meta, Release).unwrap();
            enigo.key(Key::Control, Release).unwrap();
        },
    }

    Ok(())
}

pub fn input_handler(data: Value) -> Result<(), Box<dyn Error>> {
    let params: InputParams = serde_json::from_value(data)?;
    println!("Handling input: {:?}", params);
    
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    
    enigo.text(&params.text).unwrap();

    Ok(())
}