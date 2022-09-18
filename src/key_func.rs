use std::sync::Mutex;
use std::thread;
use std::{collections::HashMap, sync::Arc};
use serde::{Serialize, Deserialize};
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

#[derive(Debug, Eq, PartialEq, Hash, Clone, Deserialize, Serialize)]
pub struct VirtualKeySet {
    pub keys: Vec<VirtualKey>,
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
    fn is_released(&self) -> bool {
        !self.is_pressed()
    }
    fn press(&self) {}
    fn on_pressed(&self, func: &dyn Fn() -> ()) {
        if self.is_pressed() {
            func();
        }
    }
}

pub type KeyStatusMap = HashMap<VirtualKey, KeyStatus>;
pub type KeyHandler = dyn Fn() + Send + Sync + 'static;
pub type KeyHandlerPtr = Arc<KeyHandler>;

pub static KEY_STATUS_MAP: Lazy<Mutex<KeyStatusMap>> =
    Lazy::new(|| Mutex::new(KeyStatusMap::new()));

pub static KEY_SET_MAP: Lazy<Mutex<KeySetMap>> = Lazy::new(|| Mutex::new(KeySetMap::new()));

pub struct KeySetMap {
    key_set_list: HashMap<VirtualKeySet, Arc<KeyHandler>>,
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
        // todo: need to profile this control flow
        for key_set in &self.key_set_list {
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
                            return true;
                        } else {
                            key_set.1();
                        }
                    } else {
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
        return false;
    }

    pub fn release(&mut self, _: VirtualKey) {}

    pub fn put(&mut self, key_set: VirtualKeySet, binding: KeyHandlerPtr) {
        self.key_set_list.insert(key_set, binding);
    }

    fn clear(&mut self) {
        self.last_key_set_pressed = None;
        self.last_pressed_time = None;
    }
}

pub fn when_keys_pressed<F: Fn() + Send + Sync + 'static>(keys: &[VirtualKey], binding: F) {
    let key_set = VirtualKeySet {
        keys: keys.to_vec(),
    };

    println!("put key set {:?}", &key_set);
    KEY_SET_MAP.lock().unwrap().put(key_set, Arc::new(binding));
}

pub fn bind_key_set(keys: &[VirtualKey], key: VirtualKey) {
    when_keys_pressed(keys, move || key.press())
}

/// bind a set of keys to a set of simulated keys
///
/// the simulated keys will be pressed sequencely
pub fn bind_key_sets(keys: &[VirtualKey], simulated_keys: &[VirtualKey]) {
    let simulated_keys = simulated_keys.to_vec();
    when_keys_pressed(keys, move || {
        for key in &simulated_keys {
            key.press();
        }
    })
}
