
mod win_func;
pub mod key_enums;
pub mod key_func;
use key_func::*;
use win_func::*;
use key_enums::{VirtualKey as Vk, Key};
fn main() {

    send_key_input(&Vk::Key0);

    send_key_input(&Vk::Key1);
    send_key_input(&Vk::Key2);

    when_keys_pressed(&[Vk::CapsLock,Vk::KeyH], ||{
        Vk::LeftArrow.press();
    });
    listen_event();
}
