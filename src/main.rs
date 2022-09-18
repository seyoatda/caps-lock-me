pub mod key_enums;
pub mod key_func;
mod win_func;
mod config;


use std::path::Path;
use key_enums::{Key, VirtualKey as Vk};
use key_func::*;
use win_func::*;
use crate::config::Config;
use crate::config::read_config;

fn main() {
    when_keys_pressed(&[Vk::CapsLock, Vk::KeyH], || {
        Vk::LeftArrow.press();
    });
    init_key_mappings();

    listen_event();
}

fn init_key_mappings() {
    let config = read_config(Path::new("./resources/config.toml"));
    for x in config.key_mappings {
        bind_key_sets(x.origin.keys.as_slice(), x.mapping.keys.as_slice())
    }
}
