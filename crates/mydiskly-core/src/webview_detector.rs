use std::path::PathBuf;
use std::process::Command;
use windows_sys::Win32::UI::Shell::ShellExecuteW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    MessageBoxW, IDYES, MB_ICONWARNING, MB_YESNO,
};
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
use winreg::RegKey;

use crate::gatekeeper::to_wide_chars;

pub const WEBVIEW2_DOWNLOAD_URL: &str = "https://go.microsoft.com/fwlink/p/?LinkId=2124703";

/// Helper function to search a directory for subdirectories containing `msedgewebview2.exe`.
fn check_app_dir(base_dir: PathBuf) -> Option<PathBuf> {
    if !base_dir.is_dir() {
        return None;
    }
    if base_dir.join("msedgewebview2.exe").is_file() {
        return Some(base_dir);
    }
    if let Ok(entries) = std::fs::read_dir(&base_dir) {
        for entry in entries.flatten() {
            let sub = entry.path();
            if sub.is_dir() && sub.join("msedgewebview2.exe").is_file() {
                return Some(sub);
            }
        }
    }
    None
}

/// Helper function to inspect registry subkeys and verify physical file existence.
fn check_registry_key(root: RegKey) -> Option<PathBuf> {
    let subkeys = [
        r"SOFTWARE\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}",
        r"SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}",
    ];

    for sub in &subkeys {
        if let Ok(key) = root.open_subkey(sub) {
            let loc: Result<String, _> = key.get_value("location");
            let pv: Result<String, _> = key.get_value("pv");
            if let (Ok(loc_str), Ok(pv_str)) = (loc, pv) {
                if !pv_str.trim().is_empty() && pv_str.trim() != "0.0.0.0" {
                    let path_with_pv = PathBuf::from(&loc_str).join(&pv_str);
                    if path_with_pv.join("msedgewebview2.exe").is_file() {
                        return Some(path_with_pv);
                    }
                    let path_direct = PathBuf::from(&loc_str);
                    if path_direct.join("msedgewebview2.exe").is_file() {
                        return Some(path_direct);
                    }
                }
            }
        }
    }
    None
}

/// Physically inspects the system to locate a functional WebView2 runtime.
/// Guards against ghost/phantom registry keys left by stripped or debloated Windows editions.
pub fn find_webview2_runtime_path() -> Option<PathBuf> {
    // 1. Check if WEBVIEW2_BROWSER_EXECUTABLE_FOLDER is already set and physically valid
    if let Ok(env_path) = std::env::var("WEBVIEW2_BROWSER_EXECUTABLE_FOLDER") {
        let p = PathBuf::from(&env_path);
        if p.join("msedgewebview2.exe").is_file() {
            return Some(p);
        }
    }

    // 2. Check LocalAppData (Per-user fixed/standalone runtime installation)
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        let user_edge = PathBuf::from(local_app_data)
            .join("Microsoft")
            .join("EdgeWebView")
            .join("Application");
        if let Some(found) = check_app_dir(user_edge) {
            return Some(found);
        }
    }

    // 3. Check HKCU registry (verified physical path)
    if let Some(found) = check_registry_key(RegKey::predef(HKEY_CURRENT_USER)) {
        return Some(found);
    }

    // 4. Check HKLM registry (verified physical path)
    if let Some(found) = check_registry_key(RegKey::predef(HKEY_LOCAL_MACHINE)) {
        return Some(found);
    }

    // 5. Check Program Files (x86) and Program Files standard directories
    for pf_var in ["ProgramFiles(x86)", "ProgramFiles"] {
        if let Ok(pf) = std::env::var(pf_var) {
            let sys_edge = PathBuf::from(pf)
                .join("Microsoft")
                .join("EdgeWebView")
                .join("Application");
            if let Some(found) = check_app_dir(sys_edge) {
                return Some(found);
            }
        }
    }

    None
}

/// Ensures WebView2 runtime environment is prepared before Tauri initializes.
/// Sets the `WEBVIEW2_BROWSER_EXECUTABLE_FOLDER` environment variable in process memory.
pub fn ensure_webview2_environment() -> bool {
    if let Some(runtime_path) = find_webview2_runtime_path() {
        std::env::set_var("WEBVIEW2_BROWSER_EXECUTABLE_FOLDER", &runtime_path);
        true
    } else {
        false
    }
}

/// Checks if Microsoft Edge WebView2 Evergreen or Fixed Runtime is physically available on the system.
pub fn is_webview2_installed() -> bool {
    ensure_webview2_environment()
}

/// Displays an English-only native Win32 interactive guidance popup when WebView2 is missing.
pub fn show_webview2_missing_dialog_en() -> bool {
    let title = to_wide_chars("MyDiskly — Microsoft Edge WebView2 Required");
    let message = to_wide_chars(
        "Microsoft Edge WebView2 Runtime is missing on your Windows installation.\n\n\
        This official Microsoft component is required to display Mydiskly's modern desktop interface.\n\n\
        Would you like to open the official download page to install it now?"
    );

    let res = unsafe {
        MessageBoxW(
            0,
            message.as_ptr(),
            title.as_ptr(),
            MB_ICONWARNING | MB_YESNO,
        )
    };

    if res == IDYES {
        open_webview2_download_page();
        true
    } else {
        false
    }
}

/// Opens the official Microsoft Edge WebView2 evergreen installer download URL in default browser.
pub fn open_webview2_download_page() {
    let operation = to_wide_chars("open");
    let url = to_wide_chars(WEBVIEW2_DOWNLOAD_URL);

    unsafe {
        ShellExecuteW(
            0,
            operation.as_ptr(),
            url.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            1, // SW_SHOWNORMAL
        );
    }
}

/// Silent background auto-bootstrapper for custom installer.
/// Executes `MicrosoftEdgeWebview2Setup.exe /silent /install` if pre-downloaded or available.
pub fn run_silent_webview2_installer(installer_exe_path: &std::path::Path) -> bool {
    if !installer_exe_path.exists() {
        return false;
    }

    let status = Command::new(installer_exe_path)
        .arg("/silent")
        .arg("/install")
        .status();

    match status {
        Ok(s) => s.success(),
        Err(_) => false,
    }
}
