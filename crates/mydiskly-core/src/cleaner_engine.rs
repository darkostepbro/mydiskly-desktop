use std::fs;
use std::path::{Path, PathBuf};
use windows_sys::Win32::UI::Shell::SHEmptyRecycleBinW;
use windows_sys::Win32::UI::Shell::{SHERB_NOCONFIRMATION, SHERB_NOPROGRESSUI, SHERB_NOSOUND};

use crate::models::{CleanResult, CleanerCategory};

pub fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

pub fn calculate_dir_size(path: &Path) -> u64 {
    let mut total = 0u64;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                total += calculate_dir_size(&p);
            } else if let Ok(meta) = p.metadata() {
                total += meta.len();
            }
        }
    }
    total
}

fn collect_chromium_cache_dirs(user_data_base: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if user_data_base.exists() {
        if let Ok(entries) = fs::read_dir(user_data_base) {
            for e in entries.flatten() {
                let name = e.file_name().to_string_lossy().to_string();
                if name == "Default" || name.starts_with("Profile ") {
                    let p = e.path();
                    for sub in &["Cache", "Code Cache", "GPUCache"] {
                        let target = p.join(sub);
                        if target.exists() {
                            paths.push(target);
                        }
                    }
                    let sw_cache = p.join("Service Worker").join("CacheStorage");
                    if sw_cache.exists() {
                        paths.push(sw_cache);
                    }
                }
            }
        }
    }
    paths
}

pub fn is_chrome_installed() -> bool {
    let local = std::env::var("LOCALAPPDATA").unwrap_or_default();
    Path::new(r"C:\Program Files\Google\Chrome\Application\chrome.exe").exists()
        || Path::new(r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe").exists()
        || Path::new(&local).join(r"Google\Chrome\Application\chrome.exe").exists()
        || Path::new(&local).join(r"Google\Chrome\User Data\Default").exists()
}

pub fn get_chrome_cache_dirs() -> Vec<PathBuf> {
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        let base = Path::new(&local).join("Google").join("Chrome").join("User Data");
        return collect_chromium_cache_dirs(&base);
    }
    Vec::new()
}

pub fn is_brave_installed() -> bool {
    let local = std::env::var("LOCALAPPDATA").unwrap_or_default();
    Path::new(r"C:\Program Files\BraveSoftware\Brave-Browser\Application\brave.exe").exists()
        || Path::new(r"C:\Program Files (x86)\BraveSoftware\Brave-Browser\Application\brave.exe").exists()
        || Path::new(&local).join(r"BraveSoftware\Brave-Browser\Application\brave.exe").exists()
        || Path::new(&local).join(r"BraveSoftware\Brave-Browser\User Data\Default").exists()
}

pub fn get_brave_cache_dirs() -> Vec<PathBuf> {
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        let base = Path::new(&local)
            .join("BraveSoftware")
            .join("Brave-Browser")
            .join("User Data");
        return collect_chromium_cache_dirs(&base);
    }
    Vec::new()
}

pub fn is_edge_installed() -> bool {
    Path::new(r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe").exists()
        || Path::new(r"C:\Program Files\Microsoft\Edge\Application\msedge.exe").exists()
}

pub fn get_edge_cache_dirs() -> Vec<PathBuf> {
    if !is_edge_installed() {
        return Vec::new();
    }
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        let base = Path::new(&local).join("Microsoft").join("Edge").join("User Data");
        return collect_chromium_cache_dirs(&base);
    }
    Vec::new()
}

pub fn is_firefox_installed() -> bool {
    let local = std::env::var("LOCALAPPDATA").unwrap_or_default();
    Path::new(r"C:\Program Files\Mozilla Firefox\firefox.exe").exists()
        || Path::new(r"C:\Program Files (x86)\Mozilla Firefox\firefox.exe").exists()
        || Path::new(&local).join(r"Mozilla Firefox\firefox.exe").exists()
}

pub fn get_firefox_cache_dirs() -> Vec<PathBuf> {
    if !is_firefox_installed() {
        return Vec::new();
    }
    let mut paths = Vec::new();
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        let base = Path::new(&local).join("Mozilla").join("Firefox").join("Profiles");
        if base.exists() {
            if let Ok(entries) = fs::read_dir(&base) {
                for e in entries.flatten() {
                    let target = e.path().join("cache2");
                    if target.exists() {
                        paths.push(target);
                    }
                }
            }
        }
    }
    paths
}

pub fn is_opera_installed() -> bool {
    let local = std::env::var("LOCALAPPDATA").unwrap_or_default();
    Path::new(&local).join(r"Programs\Opera\opera.exe").exists()
        || Path::new(&local).join(r"Programs\Opera GX\opera.exe").exists()
        || Path::new(r"C:\Program Files\Opera\opera.exe").exists()
}

pub fn get_opera_cache_dirs() -> Vec<PathBuf> {
    if !is_opera_installed() {
        return Vec::new();
    }
    let mut paths = Vec::new();
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        for folder in &["Opera Stable", "Opera GX Stable"] {
            let base = Path::new(&local).join("Opera Software").join(folder);
            if base.exists() {
                for sub in &["Cache", "Code Cache", "GPUCache"] {
                    let target = base.join(sub);
                    if target.exists() {
                        paths.push(target);
                    }
                }
            }
        }
    }
    paths
}

#[repr(C)]
#[allow(non_snake_case)]
struct SHQUERYRBINFO {
    cbSize: u32,
    i64Size: i64,
    i64NumItems: i64,
}

#[link(name = "shell32")]
extern "system" {
    fn SHQueryRecycleBinW(pszRootPath: *const u16, pSHQueryRBInfo: *mut SHQUERYRBINFO) -> i32;
}

pub fn query_recycle_bin_size() -> u64 {
    let mut info = SHQUERYRBINFO {
        cbSize: std::mem::size_of::<SHQUERYRBINFO>() as u32,
        i64Size: 0,
        i64NumItems: 0,
    };
    let res = unsafe { SHQueryRecycleBinW(std::ptr::null(), &mut info) };
    if res == 0 && info.i64Size > 0 {
        info.i64Size as u64
    } else {
        0
    }
}

pub fn scan_cleaner_categories() -> Vec<CleanerCategory> {
    let local = std::env::var("LOCALAPPDATA").unwrap_or_default();
    let temp = std::env::var("TEMP").unwrap_or_default();

    let rb_sz = query_recycle_bin_size();
    let temp_sz = calculate_dir_size(Path::new(&temp));
    let d3d_sz = calculate_dir_size(&Path::new(&local).join("D3DSCache"));
    let crash_sz = calculate_dir_size(&Path::new(&local).join("CrashDumps"));
    let win_temp_sz = calculate_dir_size(Path::new(r"C:\Windows\Temp"));
    let update_sz = calculate_dir_size(Path::new(r"C:\Windows\SoftwareDistribution\Download"));

    let chrome_installed = is_chrome_installed();
    let chrome_sz: u64 = if chrome_installed {
        get_chrome_cache_dirs().iter().map(|p| calculate_dir_size(p)).sum()
    } else {
        0
    };

    let brave_installed = is_brave_installed();
    let brave_sz: u64 = if brave_installed {
        get_brave_cache_dirs().iter().map(|p| calculate_dir_size(p)).sum()
    } else {
        0
    };

    let edge_installed = is_edge_installed();
    let edge_sz: u64 = if edge_installed {
        get_edge_cache_dirs().iter().map(|p| calculate_dir_size(p)).sum()
    } else {
        0
    };

    let ff_installed = is_firefox_installed();
    let ff_sz: u64 = if ff_installed {
        get_firefox_cache_dirs().iter().map(|p| calculate_dir_size(p)).sum()
    } else {
        0
    };

    let opera_installed = is_opera_installed();
    let opera_sz: u64 = if opera_installed {
        get_opera_cache_dirs().iter().map(|p| calculate_dir_size(p)).sum()
    } else {
        0
    };

    vec![
        // ================= SYSTEM CATEGORIES =================
        CleanerCategory {
            id: "recycle_bin".to_string(),
            name_id: "Windows Recycle Bin".to_string(),
            name_en: "Windows Recycle Bin".to_string(),
            desc_id: "Berkas yang dihapus dan tersimpan di keranjang sampah".to_string(),
            desc_en: "Deleted files stored in Windows Recycle Bin".to_string(),
            size_bytes: rb_sz,
            size_str: format_size(rb_sz),
            requires_admin: false,
            is_safe: true,
            selected: rb_sz > 0,
            is_browser: false,
            is_installed: true,
        },
        CleanerCategory {
            id: "user_temp".to_string(),
            name_id: "User Temp Files (%TEMP%)".to_string(),
            name_en: "User Temp Files (%TEMP%)".to_string(),
            desc_id: "Berkas sementara aplikasi pengguna yang tidak aktif".to_string(),
            desc_en: "Temporary application scratch files in user profile".to_string(),
            size_bytes: temp_sz,
            size_str: format_size(temp_sz),
            requires_admin: false,
            is_safe: true,
            selected: temp_sz > 0,
            is_browser: false,
            is_installed: true,
        },
        CleanerCategory {
            id: "d3d_cache".to_string(),
            name_id: "DirectX Shader Cache".to_string(),
            name_en: "DirectX Shader Cache".to_string(),
            desc_id: "Cache kompilasi shader grafis GPU DirectX".to_string(),
            desc_en: "Precompiled graphics shader cache from DirectX GPU".to_string(),
            size_bytes: d3d_sz,
            size_str: format_size(d3d_sz),
            requires_admin: false,
            is_safe: true,
            selected: d3d_sz > 0,
            is_browser: false,
            is_installed: true,
        },
        CleanerCategory {
            id: "crash_dumps".to_string(),
            name_id: "Crash Error Dumps".to_string(),
            name_en: "Crash Error Dumps".to_string(),
            desc_id: "Laporan crash diagnostik aplikasi Windows lama".to_string(),
            desc_en: "Diagnostic memory dump files from past app crashes".to_string(),
            size_bytes: crash_sz,
            size_str: format_size(crash_sz),
            requires_admin: false,
            is_safe: true,
            selected: crash_sz > 0,
            is_browser: false,
            is_installed: true,
        },
        CleanerCategory {
            id: "win_temp".to_string(),
            name_id: "Windows System Temp".to_string(),
            name_en: "Windows System Temp".to_string(),
            desc_id: "Berkas sementara sistem Windows (C:\\Windows\\Temp)".to_string(),
            desc_en: "Global Windows OS temporary files (C:\\Windows\\Temp)".to_string(),
            size_bytes: win_temp_sz,
            size_str: format_size(win_temp_sz),
            requires_admin: true,
            is_safe: true,
            selected: false,
            is_browser: false,
            is_installed: true,
        },
        CleanerCategory {
            id: "win_update".to_string(),
            name_id: "Windows Update Cache".to_string(),
            name_en: "Windows Update Cache".to_string(),
            desc_id: "Paket unduhan pembaruan sistem lama (SoftwareDistribution)".to_string(),
            desc_en: "Obsolete Windows update packages in SoftwareDistribution".to_string(),
            size_bytes: update_sz,
            size_str: format_size(update_sz),
            requires_admin: true,
            is_safe: true,
            selected: false,
            is_browser: false,
            is_installed: true,
        },
        // ================= BROWSER CATEGORIES =================
        CleanerCategory {
            id: "brave_cache".to_string(),
            name_id: "Brave Browser Cache".to_string(),
            name_en: "Brave Browser Cache".to_string(),
            desc_id: if brave_installed {
                "Cache web Brave. History, cookie & akun tetap aman".to_string()
            } else {
                "Browser tidak terdeteksi di perangkat ini (Read-Only)".to_string()
            },
            desc_en: if brave_installed {
                "Brave web render cache. History, cookies & logins kept".to_string()
            } else {
                "Browser not detected on this device (Read-Only)".to_string()
            },
            size_bytes: brave_sz,
            size_str: if brave_installed { format_size(brave_sz) } else { "—".to_string() },
            requires_admin: false,
            is_safe: true,
            selected: brave_installed && brave_sz > 0,
            is_browser: true,
            is_installed: brave_installed,
        },
        CleanerCategory {
            id: "chrome_cache".to_string(),
            name_id: "Google Chrome Cache".to_string(),
            name_en: "Google Chrome Cache".to_string(),
            desc_id: if chrome_installed {
                "Cache web Chrome. History, cookie & akun tetap aman".to_string()
            } else {
                "Browser tidak terdeteksi di perangkat ini (Read-Only)".to_string()
            },
            desc_en: if chrome_installed {
                "Chrome web render cache. History, cookies & logins kept".to_string()
            } else {
                "Browser not detected on this device (Read-Only)".to_string()
            },
            size_bytes: chrome_sz,
            size_str: if chrome_installed { format_size(chrome_sz) } else { "—".to_string() },
            requires_admin: false,
            is_safe: true,
            selected: chrome_installed && chrome_sz > 0,
            is_browser: true,
            is_installed: chrome_installed,
        },
        CleanerCategory {
            id: "edge_cache".to_string(),
            name_id: "Microsoft Edge Cache".to_string(),
            name_en: "Microsoft Edge Cache".to_string(),
            desc_id: if edge_installed {
                "Cache web Edge. History, cookie & akun tetap aman".to_string()
            } else {
                "Browser tidak terdeteksi di perangkat ini (Read-Only)".to_string()
            },
            desc_en: if edge_installed {
                "Edge web render cache. History, cookies & logins kept".to_string()
            } else {
                "Browser not detected on this device (Read-Only)".to_string()
            },
            size_bytes: edge_sz,
            size_str: if edge_installed { format_size(edge_sz) } else { "—".to_string() },
            requires_admin: false,
            is_safe: true,
            selected: edge_installed && edge_sz > 0,
            is_browser: true,
            is_installed: edge_installed,
        },
        CleanerCategory {
            id: "firefox_cache".to_string(),
            name_id: "Mozilla Firefox Cache".to_string(),
            name_en: "Mozilla Firefox Cache".to_string(),
            desc_id: if ff_installed {
                "Cache HTTP Firefox. History, cookie & akun tetap aman".to_string()
            } else {
                "Browser tidak terdeteksi di perangkat ini (Read-Only)".to_string()
            },
            desc_en: if ff_installed {
                "Firefox HTTP cache. History, cookies & logins kept".to_string()
            } else {
                "Browser not detected on this device (Read-Only)".to_string()
            },
            size_bytes: ff_sz,
            size_str: if ff_installed { format_size(ff_sz) } else { "—".to_string() },
            requires_admin: false,
            is_safe: true,
            selected: ff_installed && ff_sz > 0,
            is_browser: true,
            is_installed: ff_installed,
        },
        CleanerCategory {
            id: "opera_cache".to_string(),
            name_id: "Opera / Opera GX Cache".to_string(),
            name_en: "Opera / Opera GX Cache".to_string(),
            desc_id: if opera_installed {
                "Cache web Opera. History, cookie & akun tetap aman".to_string()
            } else {
                "Browser tidak terdeteksi di perangkat ini (Read-Only)".to_string()
            },
            desc_en: if opera_installed {
                "Opera web render cache. History, cookies & logins kept".to_string()
            } else {
                "Browser not detected on this device (Read-Only)".to_string()
            },
            size_bytes: opera_sz,
            size_str: if opera_installed { format_size(opera_sz) } else { "—".to_string() },
            requires_admin: false,
            is_safe: true,
            selected: opera_installed && opera_sz > 0,
            is_browser: true,
            is_installed: opera_installed,
        },
    ]
}

pub fn execute_clean(selected_ids: &[String]) -> CleanResult {
    execute_clean_with_progress(selected_ids, |_, _| {})
}

pub fn execute_clean_with_progress<F>(selected_ids: &[String], mut on_progress: F) -> CleanResult
where
    F: FnMut(f32, &str),
{
    let mut total_freed = 0u64;
    let mut count = 0u32;
    let local = std::env::var("LOCALAPPDATA").unwrap_or_default();
    let temp = std::env::var("TEMP").unwrap_or_default();

    let total_steps = selected_ids.len().max(1) as f32;

    for (idx, id) in selected_ids.iter().enumerate() {
        let step_prog = (idx as f32 / total_steps).clamp(0.05, 0.95);
        on_progress(step_prog, id.as_str());
        std::thread::sleep(std::time::Duration::from_millis(220));

        match id.as_str() {
            "recycle_bin" => {
                let before = query_recycle_bin_size();
                unsafe {
                    let _ = SHEmptyRecycleBinW(
                        0,
                        std::ptr::null(),
                        SHERB_NOCONFIRMATION | SHERB_NOPROGRESSUI | SHERB_NOSOUND,
                    );
                }
                total_freed += before;
                count += 1;
            }
            "user_temp" => {
                total_freed += clean_dir_contents(Path::new(&temp));
                count += 1;
            }
            "d3d_cache" => {
                total_freed += clean_dir_contents(&Path::new(&local).join("D3DSCache"));
                count += 1;
            }
            "crash_dumps" => {
                total_freed += clean_dir_contents(&Path::new(&local).join("CrashDumps"));
                count += 1;
            }
            "brave_cache" => {
                for p in get_brave_cache_dirs() {
                    total_freed += clean_dir_contents(&p);
                }
                count += 1;
            }
            "chrome_cache" => {
                for p in get_chrome_cache_dirs() {
                    total_freed += clean_dir_contents(&p);
                }
                count += 1;
            }
            "edge_cache" => {
                for p in get_edge_cache_dirs() {
                    total_freed += clean_dir_contents(&p);
                }
                count += 1;
            }
            "firefox_cache" => {
                for p in get_firefox_cache_dirs() {
                    total_freed += clean_dir_contents(&p);
                }
                count += 1;
            }
            "opera_cache" => {
                for p in get_opera_cache_dirs() {
                    total_freed += clean_dir_contents(&p);
                }
                count += 1;
            }
            "win_temp" => {
                total_freed += clean_dir_contents(Path::new(r"C:\Windows\Temp"));
                count += 1;
            }
            "win_update" => {
                total_freed += clean_dir_contents(Path::new(r"C:\Windows\SoftwareDistribution\Download"));
                count += 1;
            }
            _ => {}
        }
    }

    on_progress(1.0, "completed");
    let freed_str = format_size(total_freed);
    CleanResult {
        success: true,
        freed_bytes: total_freed,
        freed_str: freed_str.clone(),
        items_cleaned: count,
        message: freed_str,
    }
}

fn clean_dir_contents(dir: &Path) -> u64 {
    let mut freed = 0u64;
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                let sub_sz = calculate_dir_size(&p);
                if fs::remove_dir_all(&p).is_ok() {
                    freed += sub_sz;
                } else {
                    freed += clean_dir_contents(&p);
                }
            } else if let Ok(meta) = p.metadata() {
                let sz = meta.len();
                if fs::remove_file(&p).is_ok() {
                    freed += sz;
                }
            }
        }
    }
    freed
}

#[link(name = "shell32")]
extern "system" {
    fn IsUserAnAdmin() -> i32;
}

/// Returns true if the current process is running with Administrator privileges.
pub fn is_running_as_admin() -> bool {
    unsafe { IsUserAnAdmin() != 0 }
}

pub fn relaunch_as_admin() -> bool {
    if let Ok(exe) = std::env::current_exe() {
        let op = crate::gatekeeper::to_wide_chars("runas");
        let path = crate::gatekeeper::to_wide_chars(&exe.to_string_lossy());
        unsafe {
            windows_sys::Win32::UI::Shell::ShellExecuteW(
                0,
                op.as_ptr(),
                path.as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
                1,
            );
        }
        std::process::exit(0);
    }
    false
}
