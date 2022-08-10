use std::intrinsics::size_of;

use caps_lock_me::key_enums::{KeyAction, KeyStatus, VirtualKey, KEY_STATUS_MAP};
use once_cell::sync::OnceCell;
use windows::Win32::Foundation::HWND;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, VIRTUAL_KEY,
};
use windows::Win32::{
    Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM},
    UI::Input::*,
    UI::WindowsAndMessaging::*,
};
fn main() {
    set_hook(&KEYBD_HHOOK);
    //unset_hook(&KEYBD_HHOOK);
    println!("{}", KEYBD_HHOOK.get().unwrap().0);

    unsafe {
        let mut msg = MSG::default();
        GetMessageW(&mut msg, HWND::default(), 0, 0);
    }
    send_key_input(&[VirtualKey::Key0]);
}

static KEYBD_HHOOK: OnceCell<HHOOK> = OnceCell::new();

unsafe extern "system" fn keybd_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code < 0 {
        return CallNextHookEx(HHOOK::default(), code, wparam, lparam);
    }
    let action = KeyAction::try_from(wparam.0 as u32).unwrap();
    let code = (*(lparam.0 as *const KBDLLHOOKSTRUCT)).vkCode;
    let key = VirtualKey::try_from(code).unwrap();
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
    println!("{:?}", KEY_STATUS_MAP.lock().unwrap());
    // let other hook continue to handle the event
    return LRESULT(0);
}

// set hook to listen keyboard event
fn set_hook(hook_ptr: &OnceCell<HHOOK>) {
    unsafe {
        let hook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(keybd_proc), HINSTANCE::default(), 0)
            .expect("fail at setting hooks for keyboard");
        hook_ptr.set(hook).unwrap();
    }
    println!("hooked!");
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

pub fn send_key_input(keys: &[VirtualKey]) {
    let key_code = keys.get(0).unwrap();
    let flag = KEYEVENTF_KEYUP;
    let key_input = KEYBDINPUT {
        wVk: VIRTUAL_KEY(*key_code as u16),
        wScan: 0,
        dwFlags: flag,
        time: 0,
        dwExtraInfo: 0,
    };
    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: KeyboardAndMouse::INPUT_0 { ki: (key_input) },
    };
    unsafe {
        SendInput(&[input], size_of::<INPUT>());
    }
}
