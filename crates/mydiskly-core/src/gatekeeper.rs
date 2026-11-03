use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONSTOP, MB_OK};
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WindowsProfile {
    Windows10,
    Windows11,
}

pub fn to_wide_chars(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

pub fn get_windows_build_number() -> u32 {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(key) = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion") {
        if let Ok(build_str) = key.get_value::<String, _>("CurrentBuildNumber") {
            if let Ok(b) = build_str.parse::<u32>() {
                return b;
            }
        }
    }
    10240 // Fallback
}

pub fn enforce_os_gatekeeping() -> WindowsProfile {
    let build_num = get_windows_build_number();

    if build_num < 10240 {
        let msg = to_wide_chars(&format!(
            "MyDiskly requires Windows 10 (Build 10240+) or Windows 11 (Build 22000+).\n\nDetected NT Build: {}\n\nThe application will terminate to prevent system partition anomalies.",
            build_num
        ));
        let title = to_wide_chars("MyDiskly — Unsupported Platform");

        unsafe {
            MessageBoxW(0, msg.as_ptr(), title.as_ptr(), MB_ICONSTOP | MB_OK);
        }
        std::process::exit(1);
    }

    if build_num >= 22000 {
        WindowsProfile::Windows11
    } else {
        WindowsProfile::Windows10
    }
}
