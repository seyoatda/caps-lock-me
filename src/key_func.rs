use std::collections::HashMap;
use std::sync::Mutex;
use std::thread;

use once_cell::sync::Lazy;
use windows::Win32::UI::Input::KeyboardAndMouse::{KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP};

use super::key_enums::*;
use crate::win_func::*;

impl Key for VirtualKey {
    fn is_pressed(&self) -> bool {
        let map = KEY_STATUS_MAP.lock().unwrap();
        let key_status = map.get(self).unwrap_or(&KeyStatus::Released);
        println!("key status: {:?}", &self);
        return key_status == &KeyStatus::Pressed;
    }

    fn press(&self) {
        let key = self.clone();
        thread::spawn(move || {
            send_key_input(&key, KEYBD_EVENT_FLAGS(0));
            send_key_input(&key, KEYEVENTF_KEYUP);
        });
        println!("{:?} send key input", &self);
    }

    fn on_pressed(&self, func: &dyn Fn() -> ()) {
        if self.is_pressed() {
            func();
        }
    }
}
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct VirtualKeySet {
    keys: Vec<VirtualKey>,
}

impl Key for VirtualKeySet {
    fn is_pressed(&self) -> bool {
        for key in &self.keys {
            if !key.is_pressed() {
                return false;
            }
        }
        println!("{:?} is pressed", self.keys);
        return true;
    }
    fn on_pressed(&self, func: &dyn Fn() -> ()) {
        if self.is_pressed() {
            func();
        }
    }
    fn press(&self) {}
}

pub type KeyStatusMap = HashMap<VirtualKey, KeyStatus>;

pub static KEY_STATUS_MAP: Lazy<Mutex<KeyStatusMap>> =
    Lazy::new(|| Mutex::new(KeyStatusMap::new()));

pub static KEY_SET_MAP: Lazy<Mutex<KeySetMap>> = Lazy::new(|| Mutex::new(KeySetMap::new()));

pub struct KeySetMap {
    key_set_list: HashMap<VirtualKeySet, fn()>,
}

impl KeySetMap {
    pub fn new() -> KeySetMap {
        KeySetMap {
            key_set_list: HashMap::new(),
        }
    }

    /// Returns the go through of this [`KeySetMap`].
    /// if return true then some key was pressed
    pub fn go_through(&self) -> bool {
        for key_set in &self.key_set_list {
            println!("go through {:?}", &key_set.0);
            if key_set.0.is_pressed() {
                key_set.1();
                println!("{:?} will will execute", &key_set.1);
                return true;
            }
        }
        return false;
    }

    pub fn put(&mut self, key_set: VirtualKeySet, binding: fn()) {
        self.key_set_list.insert(key_set, binding);
    }
}

pub fn when_keys_pressed(keys: &[VirtualKey], binding: fn() -> ()) {
    let key_set = VirtualKeySet {
        keys: keys.to_vec(),
    };

    println!("put key set {:?}", &key_set);
    KEY_SET_MAP.lock().unwrap().put(key_set, binding);
}
