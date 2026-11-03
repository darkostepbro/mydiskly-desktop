use windows_sys::Win32::UI::WindowsAndMessaging::{
    MessageBoxW, MB_ICONERROR, MB_ICONINFORMATION, MB_ICONWARNING, MB_OK,
};
use crate::gatekeeper::to_wide_chars;

pub fn show_info_box(title: &str, message: &str) {
    let t = to_wide_chars(title);
    let m = to_wide_chars(message);
    unsafe {
        MessageBoxW(0, m.as_ptr(), t.as_ptr(), MB_ICONINFORMATION | MB_OK);
    }
}

pub fn show_warning_box(title: &str, message: &str) {
    let t = to_wide_chars(title);
    let m = to_wide_chars(message);
    unsafe {
        MessageBoxW(0, m.as_ptr(), t.as_ptr(), MB_ICONWARNING | MB_OK);
    }
}

pub fn show_error_box(title: &str, message: &str) {
    let t = to_wide_chars(title);
    let m = to_wide_chars(message);
    unsafe {
        MessageBoxW(0, m.as_ptr(), t.as_ptr(), MB_ICONERROR | MB_OK);
    }
}
