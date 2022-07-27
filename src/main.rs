use once_cell::sync::Lazy;
use std::borrow::BorrowMut;
use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::{thread::sleep, time::Duration};

use windows::Win32::{
    Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::*,
};
fn main() {

    unsafe {
        let mut hook_ptr = SetWindowsHookExA(WH_KEYBOARD_LL, Some(keybd_proc), HINSTANCE::default(), 0)
        .expect("fail at setting hooks for keyboard");
        KEYBD_HHOOK = Arc::new(hook_ptr);
    }
    sleep(Duration::from_secs(10));
}

static KEYBD_HHOOK: Lazy<Arc<HHOOK>> = Lazy::new(Arc::default);

unsafe extern "system" fn keybd_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    println!("code = {}",code);
    if code < 0 {
        return CallNextHookEx(HHOOK::default(), code, wparam, lparam);
    }
    if lparam.0 == 0x100 {
        let code = (*(wparam.0 as *const KBDLLHOOKSTRUCT)).vkCode;
        println!("pressed code={}", code);
    }
    return LRESULT(0);
}
