pub mod key_enums;
pub mod key_func;
mod win_func;
use key_enums::{Key, VirtualKey as Vk};
use key_func::*;
use win_func::*;
fn main() {


    
    when_keys_pressed(&[Vk::CapsLock, Vk::KeyH], || {
        Vk::LeftArrow.press();  
    });
    bind_key_set(&[Vk::CapsLock,Vk::KeyJ], Vk::DownArrow);

    listen_event();
}
