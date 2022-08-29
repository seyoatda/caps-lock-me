use std::collections::HashMap;
use std::sync::Mutex;
use std::thread;

use chrono::Local;
use once_cell::sync::Lazy;
use windows::Win32::UI::Input::KeyboardAndMouse::{KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP};

use super::key_enums::*;
use crate::win_func::*;

impl Key for VirtualKey {
    fn is_pressed(&self) -> bool {
        let map = KEY_STATUS_MAP.lock().unwrap();
        let key_status = map.get(self).unwrap_or(&KeyStatus::Released);
        return key_status == &KeyStatus::Pressed;
    }

    fn is_released(&self) -> bool {
        !self.is_pressed()
    }

    fn press(&self) {
        let key = self.clone();
        thread::spawn(move || {
            send_key_input(&key, KEYBD_EVENT_FLAGS(0));
            send_key_input(&key, KEYEVENTF_KEYUP);
        });
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

impl VirtualKeySet {
    pub fn any_key_released(&self) -> bool {
        for key in &self.keys {
            if key.is_released() {
                return true;
            }
        }
        return false;
    }
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
    fn is_released(&self) -> bool {
        !self.is_pressed()
    }
    fn press(&self) {}
}

pub type KeyStatusMap = HashMap<VirtualKey, KeyStatus>;

pub static KEY_STATUS_MAP: Lazy<Mutex<KeyStatusMap>> =
    Lazy::new(|| Mutex::new(KeyStatusMap::new()));

pub static KEY_SET_MAP: Lazy<Mutex<KeySetMap>> = Lazy::new(|| Mutex::new(KeySetMap::new()));

pub struct KeySetMap {
    key_set_list: HashMap<VirtualKeySet, fn()>,
    last_key_set_pressed: Option<VirtualKeySet>,
    last_pressed_time: Option<i64>,
}

impl KeySetMap {
    pub fn new() -> KeySetMap {
        KeySetMap {
            key_set_list: HashMap::new(),
            last_key_set_pressed: None,
            last_pressed_time: None,
        }
    }

    /// Returns the go through of this [`KeySetMap`].
    /// if return true then some key was pressed
    pub fn go_through(&mut self) -> bool {
        for key_set in &self.key_set_list {
            println!("go through {:?}", &key_set.0);
            let cur_keys = key_set.0;
            let cur_time = Local::now().timestamp_millis();
            if cur_keys.is_pressed() {
                if self.last_key_set_pressed.is_none() {
                    self.last_key_set_pressed = Some(cur_keys.clone());
                    self.last_pressed_time = Some(cur_time);
                    key_set.1();
                } else {
                    if self.last_key_set_pressed.as_ref().unwrap() == cur_keys {
                        if cur_time - self.last_pressed_time.unwrap() < 500 {
                            println!(
                                "{:?}:last pressed time {}",
                                &cur_keys,
                                self.last_pressed_time.unwrap()
                            );
                            println!("{:?}:cur pressed time {}", &cur_keys, cur_time);
                            return true;
                        } else {
                            println!(" more than 500 ms, keep printing");
                            key_set.1();
                        }
                    } else {
                        println!(
                            "another key was pressed: {:?}, time: {}",
                            &cur_keys, cur_time
                        );
                        self.last_key_set_pressed = Some(cur_keys.clone());
                        self.last_pressed_time = Some(cur_time);
                        key_set.1();
                    }
                }
                return true;
            } else if cur_keys.any_key_released() {
                self.last_key_set_pressed = None;
                self.last_pressed_time = None;
            }
        }
        // capslock as a function key, block other key input when it was pressed
        return false;
    }

    pub fn release(&mut self, key: VirtualKey) {}

    pub fn put(&mut self, key_set: VirtualKeySet, binding: fn()) {
        self.key_set_list.insert(key_set, binding);
    }

    fn clear(&mut self) {
        self.last_key_set_pressed = None;
        self.last_pressed_time = None;
    }
}

pub fn when_keys_pressed(keys: &[VirtualKey], binding: fn() -> ()) {
    let key_set = VirtualKeySet {
        keys: keys.to_vec(),
    };

    println!("put key set {:?}", &key_set);
    KEY_SET_MAP.lock().unwrap().put(key_set, binding);
}

pub fn bind_key_set(keys: &[VirtualKey], key: VirtualKey) {
    when_keys_pressed(keys, || key.press())
}
