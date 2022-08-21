use std::collections::HashMap;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use super::key_enums::*;
use crate::win_func::*;

impl Key for VirtualKey {
    fn is_pressed(&self) -> bool {
        let map = KEY_STATUS_MAP.lock().unwrap();
        let key_status = map.get(self).unwrap_or(&KeyStatus::Released);

        return key_status == &KeyStatus::Pressed;
    }

    fn press(&self) {
        send_key_input(self);
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

    pub fn go_through(&self) {
        for key_set in &self.key_set_list {
            if key_set.0.is_pressed() {
                key_set.1();
            }
        }
    }

    pub fn put(&mut self,key_set:VirtualKeySet,binding:fn()){
        self.key_set_list.insert(key_set, binding);
    }
}

pub fn when_keys_pressed(keys: &[VirtualKey], binding: fn() -> ()) {
    let key_set = VirtualKeySet{keys:keys.to_vec()};
    KEY_SET_MAP.lock().unwrap().put(key_set, binding);
}
