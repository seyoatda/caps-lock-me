pub mod key_enums;
pub mod key_func;
mod win_func;
use key_enums::{Key, VirtualKey as Vk};
use key_func::*;
use win_func::*;
fn main() {
    Vk::Key0.press();

    
    when_keys_pressed(&[Vk::CapsLock, Vk::KeyH], || {
        Vk::LeftArrow.press();
    });
    listen_event();
}
