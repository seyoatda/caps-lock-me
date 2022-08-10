use std::{collections::HashMap, sync::Mutex};

use num_enum::TryFromPrimitive;
use once_cell::sync::Lazy;

#[derive(Debug, Eq, PartialEq, Hash, TryFromPrimitive)]
#[repr(u32)]
pub enum VirtualKey {
    Backspace = 0x08,
    Tab = 0x09,
    Enter = 0x0D,
    Shift = 0x10,
    Ctrl = 0x11,
    Alt = 0x12,
    Pause = 0x13,
    CapsLock = 0x14,
    Esc = 0x1B,
    Space = 0x20,
    PageUp = 0x21,
    PageDown = 0x22,
    End = 0x23,
    Home = 0x24,
    LeftArrow = 0x25,
    UpArrow = 0x26,
    RightArrow = 0x27,
    DownArrow = 0x28,

    Key0 = 0x30,
    Key1 = 0x31,
    Key2 = 0x32,
    Key3 = 0x33,
    Key4 = 0x34,
    Key5 = 0x35,
    Key6 = 0x36,
    Key7 = 0x37,
    Key8 = 0x38,
    Key9 = 0x39,

    KeyA = 0x41,
    KeyB = 0x42,
    KeyC = 0x43,
    KeyD = 0x44,
    KeyE = 0x45,
    KeyF = 0x46,
    KeyG = 0x47,
    KeyH = 0x48,
    KeyI = 0x49,
    KeyJ = 0x4A,
    KeyK = 0x4B,
    KeyL = 0x4C,
    KeyM = 0x4D,
    KeyN = 0x4E,
    KeyO = 0x4F,
    KeyP = 0x50,
    KeyQ = 0x51,
    KeyR = 0x52,
    KeyS = 0x53,
    KeyT = 0x54,
    KeyU = 0x55,
    KeyV = 0x56,
    KeyW = 0x57,
    KeyX = 0x58,
    KeyY = 0x59,
    KeyZ = 0x5A,

    #[num_enum(default)]
    Unkown = 0x0,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum KeyAction {
    Press = 0x100,
    Release = 0x101,

    #[num_enum(default)]
    Unkown = 0x0,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum KeyStatus {
    Pressed,
    Released,
    #[num_enum(default)]
    Unkown,
}

pub type KeyStatusMap = HashMap<VirtualKey, KeyStatus>;

pub static KEY_STATUS_MAP: Lazy<Mutex<KeyStatusMap>> =
    Lazy::new(|| Mutex::new(KeyStatusMap::new()));

pub type KeySet = Vec<VirtualKey>;    
pub fn when_key_pressed(key:VirtualKey){

}
pub fn after_key_pressed(){}

pub fn when_multi_key_pressed(keys:KeySet){}
pub fn after_multi_key_pressed(keys:KeySet){

}
