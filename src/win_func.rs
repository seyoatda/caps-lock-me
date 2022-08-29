use crate::key_enums::{KeyAction, KeyStatus, VirtualKey};
use crate::key_func::{KEY_SET_MAP, KEY_STATUS_MAP};
use once_cell::sync::OnceCell;
use std::mem::size_of;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, VIRTUAL_KEY,
};

use windows::Win32::{
    Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM},
    UI::Input::*,
    UI::WindowsAndMessaging::*,
};

static KEYBD_HHOOK: OnceCell<HHOOK> = OnceCell::new();

// set hook to listen keyboard event
fn set_hook(hook_ptr: &OnceCell<HHOOK>) {
    unsafe {
        let hook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(keybd_proc), HINSTANCE::default(), 0)
            .expect("fail at setting hooks for keyboard");
        hook_ptr.set(hook).unwrap();
    }
    println!("hooked!");
}

unsafe extern "system" fn keybd_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code < 0 {
        return CallNextHookEx(HHOOK::default(), code, wparam, lparam);
    }
    let action = KeyAction::try_from(wparam.0 as u32).unwrap();
    let key_stuct = *(lparam.0 as *const KBDLLHOOKSTRUCT);
    let vk_code = key_stuct.vkCode;
    let extra_info = key_stuct.dwExtraInfo;
    let key = VirtualKey::try_from(vk_code).unwrap();
    if extra_info == 0x1234 {
        println!("a simulated key input is send");
        return CallNextHookEx(HHOOK::default(), code, wparam, lparam);
    }
    match action {
        KeyAction::Press => {
            KEY_STATUS_MAP
                .lock()
                .unwrap()
                .insert(key, KeyStatus::Pressed);
        }
        KeyAction::Release => {
            KEY_STATUS_MAP
                .lock()
                .unwrap()
                .insert(key, KeyStatus::Released);
        }
        KeyAction::Unkown => {}
    }
    println!("hook msg: key: {:?}, status:{:?}", &key, &action);
    let some_key_pressed = KEY_SET_MAP.lock().unwrap().go_through();
    if some_key_pressed {
        return LRESULT(1);
    }
    // let other hook continue to handle the event
    return LRESULT(0);
}

// unset hook to listen keyboard event
fn unset_hook(hook_ptr: &OnceCell<HHOOK>) {
    unsafe {
        let success = UnhookWindowsHookEx(*hook_ptr.get().unwrap());
        if success.as_bool() {
            println!("unhooked!");
        }
    }
}

pub fn send_key_input(key: &VirtualKey, flag: KEYBD_EVENT_FLAGS) {
    let key_code = key;
    let key_input = KEYBDINPUT {
        wVk: VIRTUAL_KEY(*key_code as u16),
        wScan: 0,
        dwFlags: flag,
        time: 0,
        dwExtraInfo: 0x1234,
    };
    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: KeyboardAndMouse::INPUT_0 { ki: (key_input) },
    };
    unsafe {
        SendInput(&[input], size_of::<INPUT>() as i32);
    }
}

pub fn listen_event() {
    set_hook(&KEYBD_HHOOK);
    unsafe {
        let mut msg = MSG::default();
        let success = GetMessageW(&mut msg, HWND::default(), 0, 0);
        println!("getMessageW: {}", success.0);
    }
}
