use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
use std::sync::OnceLock;
use mydiskly_core::gatekeeper::to_wide_chars;
use winreg::enums::*;
use winreg::RegKey;
use windows_sys::Win32::UI::Shell::{
    ShellExecuteW, Shell_NotifyIconW, NIF_ICON, NIF_INFO, NIF_MESSAGE, NIF_TIP, NIIF_INFO,
    NIIF_NOSOUND, NIM_ADD, NIM_DELETE, NIM_MODIFY, NOTIFYICONDATAW,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    FindWindowW, SetWindowPos, HWND_TOPMOST, SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_SHOWWINDOW,
    WM_USER,
};

pub const ICON_RGBA_64: &[u8] = include_bytes!("../assets/app_icon_64x64.rgba");
const ICON_BGRA_32: &[u8] = include_bytes!("../assets/app_icon_32x32.bgra");
const INTERFACE_WAV_BYTES: &[u8] = include_bytes!("../assets/interface.wav");
const EXTERNAL_WAV_PATH: &str = r"X:\Downloads\Downloads-Brave\interface.wav";

const SND_ASYNC: u32 = 0x0001;
const SND_NODEFAULT: u32 = 0x0002;
const SND_MEMORY: u32 = 0x0004;
const SND_FILENAME: u32 = 0x00020000;

const WM_TRAYICON: u32 = WM_USER + 1;
const WM_MYDISKLY_FORCE_QUIT: u32 = WM_USER + 99;
const WM_LBUTTONUP: u32 = 0x0202;
const WM_LBUTTONDBLCLK: u32 = 0x0203;
const WM_RBUTTONUP: u32 = 0x0205;
const WM_CONTEXTMENU: u32 = 0x007B;
const WM_COMMAND: u32 = 0x0111;
const WM_SETICON: u32 = 0x0080;

const SW_HIDE: i32 = 0;
const SW_SHOWNORMAL: i32 = 1;
const SW_SHOW: i32 = 5;
const SW_MINIMIZE: i32 = 6;
const SW_RESTORE: i32 = 9;
const GWLP_WNDPROC: i32 = -4;

#[repr(C)]
struct AccentPolicy {
    accent_state: u32,
    accent_flags: u32,
    gradient_color: u32,
    animation_id: u32,
}

#[repr(C)]
struct WindowCompositionAttribData {
    attrib: u32,
    pv_data: *mut std::ffi::c_void,
    cb_data: usize,
}

#[link(name = "dwmapi")]
extern "system" {
    fn DwmSetWindowAttribute(
        hwnd: isize,
        dwAttribute: u32,
        pvAttribute: *const std::ffi::c_void,
        cbAttribute: u32,
    ) -> i32;
}

const MF_STRING: u32 = 0x00000000;
const MF_SEPARATOR: u32 = 0x00000800;
const TPM_RIGHTBUTTON: u32 = 0x0002;
const TPM_RETURNCMD: u32 = 0x0100;
const TPM_NONOTIFY: u32 = 0x0080;
const TPM_BOTTOMALIGN: u32 = 0x0020;

const ID_TRAY_OPEN: usize = 2001;
const ID_TRAY_EXIT: usize = 2002;

static ORIG_WNDPROC: AtomicIsize = AtomicIsize::new(0);
static CACHED_HICON: AtomicIsize = AtomicIsize::new(0);
static DOCK_X: AtomicIsize = AtomicIsize::new(1500);
static DOCK_Y: AtomicIsize = AtomicIsize::new(0);
static DOCK_W: AtomicIsize = AtomicIsize::new(420);
static DOCK_H: AtomicIsize = AtomicIsize::new(1032);
static TRAY_RESTORE_PENDING: AtomicBool = AtomicBool::new(false);
static EGUI_CTX: OnceLock<egui::Context> = OnceLock::new();

#[repr(C)]
struct Point {
    x: i32,
    y: i32,
}

#[link(name = "winmm")]
extern "system" {
    fn PlaySoundW(pszSound: *const u8, hmod: isize, fdwSound: u32) -> i32;
}

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(lpModuleName: *const u16) -> isize;
    fn GetCurrentProcess() -> isize;
    fn SetProcessWorkingSetSize(
        hProcess: isize,
        dwMinimumWorkingSetSize: usize,
        dwMaximumWorkingSetSize: usize,
    ) -> i32;
}

#[link(name = "shell32")]
extern "system" {
    fn SetCurrentProcessExplicitAppUserModelID(AppID: *const u16) -> i32;
}

#[link(name = "user32")]
extern "system" {
    fn LoadIconW(hInstance: isize, lpIconName: *const u16) -> isize;
    fn CreateIcon(
        hInstance: isize,
        nWidth: i32,
        nHeight: i32,
        cPlanes: u8,
        cBitsPixel: u8,
        lpbANDbits: *const u8,
        lpbXORbits: *const u8,
    ) -> isize;
    fn ShowWindow(hWnd: isize, nCmdShow: i32) -> i32;
    fn BringWindowToTop(hWnd: isize) -> i32;
    fn SetForegroundWindow(hWnd: isize) -> i32;
    fn InvalidateRect(hWnd: isize, lpRect: *const std::ffi::c_void, bErase: i32) -> i32;
    fn UpdateWindow(hWnd: isize) -> i32;
    fn SetWindowLongPtrW(hWnd: isize, nIndex: i32, dwNewLong: isize) -> isize;
    fn CallWindowProcW(
        lpPrevWndFunc: isize,
        hWnd: isize,
        Msg: u32,
        wParam: usize,
        lParam: isize,
    ) -> isize;
    fn CreatePopupMenu() -> isize;
    fn AppendMenuW(hMenu: isize, uFlags: u32, uIDNewItem: usize, lpNewItem: *const u16) -> i32;
    fn TrackPopupMenu(
        hMenu: isize,
        uFlags: u32,
        x: i32,
        y: i32,
        nReserved: i32,
        hWnd: isize,
        prcRect: *const std::ffi::c_void,
    ) -> i32;
    fn DestroyMenu(hMenu: isize) -> i32;
    fn GetCursorPos(lpPoint: *mut Point) -> i32;
    fn SendMessageW(hWnd: isize, Msg: u32, wParam: usize, lParam: isize) -> isize;
    fn SetWindowCompositionAttribute(
        hWnd: isize,
        pAttrData: *mut WindowCompositionAttribData,
    ) -> i32;
}

/// Applies native Windows 10/11 Dark Blur (Acrylic / BlurBehind + DWM Immersive Dark Mode) to `hwnd`.
pub fn enable_window_dark_blur(hwnd: isize) {
    if hwnd == 0 {
        return;
    }
    unsafe {
        // 1. DWM Immersive Dark Mode (20) & Rounded Corners (33 -> DWMWCP_ROUND = 2)
        let dark_mode: i32 = 1;
        let _ = DwmSetWindowAttribute(
            hwnd,
            20,
            &dark_mode as *const _ as *const std::ffi::c_void,
            std::mem::size_of::<i32>() as u32,
        );

        // 2. SetWindowCompositionAttribute with ACCENT_ENABLE_BLURBEHIND (3) / ACRYLIC (4)
        let mut policy = AccentPolicy {
            accent_state: 3, // ACCENT_ENABLE_BLURBEHIND (smooth zero-lag dark blur)
            accent_flags: 2,
            gradient_color: 0xD814100E, // ABGR dark tint
            animation_id: 0,
        };
        let mut data = WindowCompositionAttribData {
            attrib: 19, // WCA_ACCENT_POLICY
            pv_data: &mut policy as *mut _ as *mut std::ffi::c_void,
            cb_data: std::mem::size_of::<AccentPolicy>(),
        };
        let _ = SetWindowCompositionAttribute(hwnd, &mut data);
    }
}

pub fn minimize_window(hwnd: isize) {
    if hwnd != 0 {
        unsafe {
            ShowWindow(hwnd, SW_MINIMIZE);
        }
    }
}

/// Registers `egui::Context` so Win32 System Tray events can wake up and restore the viewport immediately.
/// Also starts a lightweight force-quit watchdog so the Uninstaller can terminate MyDiskly even when elevated or in Tray.
pub fn register_egui_context(ctx: &egui::Context) {
    let _ = EGUI_CTX.set(ctx.clone());
    let sig_file = std::env::temp_dir().join("mydiskly_force_quit.signal");
    let _ = std::fs::remove_file(&sig_file);

    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(200));
        if sig_file.exists() {
            let _ = std::fs::remove_file(&sig_file);
            let title = to_wide_chars("MyDiskly");
            let hwnd = unsafe { FindWindowW(std::ptr::null(), title.as_ptr()) };
            if hwnd != 0 {
                remove_tray_icon_raw(hwnd);
            }
            std::process::exit(0);
        }
    });
}

/// Checks and consumes any pending tray restore request so `update()` can re-enforce dock geometry.
pub fn take_tray_restore_request() -> bool {
    TRAY_RESTORE_PENDING.swap(false, Ordering::SeqCst)
}

/// Trims unused working set pages from RAM to keep Task Manager memory usage minimal.
pub fn trim_working_set() {
    unsafe {
        let proc = GetCurrentProcess();
        let _ = SetProcessWorkingSetSize(proc, usize::MAX, usize::MAX);
    }
}

/// Sets the process AppUserModelID to `MyDiskly` and updates cached `NotifyIconGeneratedAumid_*`
/// registry entries so Windows 11 Notification Center always displays `MyDiskly` (never `mydiskly-app.exe`).
pub fn register_app_identity() {
    let aumid = to_wide_chars("SerojaLabs.MyDiskly");
    unsafe {
        let _ = SetCurrentProcessExplicitAppUserModelID(aumid.as_ptr());
    }

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok((key, _)) = hkcu.create_subkey(r"Software\Classes\AppUserModelId\SerojaLabs.MyDiskly")
    {
        let _ = key.set_value("DisplayName", &"MyDiskly");
    }

    // Also fix any cached NotifyIconGeneratedAumid_* entry where Windows previously stored "mydiskly-app.exe"
    if let Ok(classes_root) = hkcu.open_subkey_with_flags(
        r"Software\Classes\AppUserModelId",
        KEY_READ | KEY_WRITE,
    ) {
        for sub_name in classes_root.enum_keys().flatten() {
            if let Ok(sub_key) = classes_root.open_subkey_with_flags(&sub_name, KEY_READ | KEY_WRITE)
            {
                if let Ok(disp) = sub_key.get_value::<String, _>("DisplayName") {
                    if disp.to_lowercase().contains("mydiskly") {
                        let _ = sub_key.set_value("DisplayName", &"MyDiskly");
                    }
                }
            }
        }
    }
}

/// Loads the official MyDiskly blue square HICON from embedded PE resource or 32x32 BGRA bitmap.
pub fn get_mydiskly_hicon() -> isize {
    let cached = CACHED_HICON.load(Ordering::SeqCst);
    if cached != 0 {
        return cached;
    }

    unsafe {
        let hinst = GetModuleHandleW(std::ptr::null());
        let mut hicon = LoadIconW(hinst, 1 as *const u16);

        if hicon == 0 && ICON_BGRA_32.len() == 4096 {
            let and_mask = [0u8; 128];
            hicon = CreateIcon(
                hinst,
                32,
                32,
                1,
                32,
                and_mask.as_ptr(),
                ICON_BGRA_32.as_ptr(),
            );
        }

        if hicon != 0 {
            CACHED_HICON.store(hicon, Ordering::SeqCst);
        }
        hicon
    }
}

/// Builds `egui::IconData` (64x64 RGBA) for `ViewportBuilder::with_icon`.
pub fn get_egui_icon_data() -> egui::IconData {
    egui::IconData {
        rgba: ICON_RGBA_64.to_vec(),
        width: 64,
        height: 64,
    }
}

/// Plays the custom notification sound (`interface.wav`) asynchronously.
pub fn play_interface_sound() {
    if Path::new(EXTERNAL_WAV_PATH).exists() {
        let wide_path = to_wide_chars(EXTERNAL_WAV_PATH);
        unsafe {
            PlaySoundW(
                wide_path.as_ptr() as *const u8,
                0,
                SND_FILENAME | SND_ASYNC | SND_NODEFAULT,
            );
        }
    } else {
        unsafe {
            PlaySoundW(
                INTERFACE_WAV_BYTES.as_ptr(),
                0,
                SND_MEMORY | SND_ASYNC | SND_NODEFAULT,
            );
        }
    }
}

/// Detects whether Windows System Notifications are enabled on the user's device.
pub fn is_system_notification_enabled() -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // 1. Check HKCU\Software\Microsoft\Windows\CurrentVersion\PushNotifications -> ToastEnabled
    if let Ok(key) =
        hkcu.open_subkey(r"Software\Microsoft\Windows\CurrentVersion\PushNotifications")
    {
        if let Ok(val) = key.get_value::<u32, _>("ToastEnabled") {
            if val == 0 {
                return false;
            }
        }
    }

    // 2. Check HKCU\Software\Microsoft\Windows\CurrentVersion\Notifications\Settings -> NOC_GLOBAL_SETTING_TOASTS_ENABLED
    if let Ok(key) =
        hkcu.open_subkey(r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings")
    {
        if let Ok(val) = key.get_value::<u32, _>("NOC_GLOBAL_SETTING_TOASTS_ENABLED") {
            if val == 0 {
                return false;
            }
        }
    }

    // 3. Check HKCU\Software\Policies\Microsoft\Windows\Explorer -> DisableNotificationCenter
    if let Ok(key) = hkcu.open_subkey(r"Software\Policies\Microsoft\Windows\Explorer") {
        if let Ok(val) = key.get_value::<u32, _>("DisableNotificationCenter") {
            if val == 1 {
                return false;
            }
        }
    }

    // 4. Check HKCU\Software\Policies\Microsoft\Windows\CurrentVersion\PushNotifications -> NoToastApplicationNotification
    if let Ok(key) =
        hkcu.open_subkey(r"Software\Policies\Microsoft\Windows\CurrentVersion\PushNotifications")
    {
        if let Ok(val) = key.get_value::<u32, _>("NoToastApplicationNotification") {
            if val == 1 {
                return false;
            }
        }
    }

    // 5. Check Windows 11 Do Not Disturb / Focus Assist in CloudStore QuietHoursSettings
    if let Ok(key) = hkcu.open_subkey(
        r"Software\Microsoft\Windows\CurrentVersion\CloudStore\Store\DefaultAccount\Current\default$windows.data.donotdisturb.quiethourssettings\windows.data.donotdisturb.quiethourssettings",
    ) {
        if let Ok(raw) = key.get_raw_value("Data") {
            let u16_slice: Vec<u16> = raw
                .bytes
                .chunks_exact(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .collect();
            let decoded = String::from_utf16_lossy(&u16_slice);
            if decoded.contains("PriorityOnly") || decoded.contains("AlarmsOnly") {
                return false;
            }
        }
    }

    true
}

/// Opens Windows System Notification Settings (`ms-settings:notifications`).
pub fn open_windows_notification_settings() {
    let verb = to_wide_chars("open");
    let uri = to_wide_chars("ms-settings:notifications");
    unsafe {
        ShellExecuteW(
            0,
            verb.as_ptr(),
            uri.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            1,
        );
    }
}

/// Window Procedure hook to handle System Tray clicks & context menu while window is visible or hidden.
unsafe extern "system" fn tray_subclass_wndproc(
    hwnd: isize,
    msg: u32,
    wparam: usize,
    lparam: isize,
) -> isize {
    if msg == WM_MYDISKLY_FORCE_QUIT {
        remove_tray_icon_raw(hwnd);
        std::process::exit(0);
    } else if msg == WM_TRAYICON {
        let event = (lparam as u32) & 0xFFFF;
        if event == WM_LBUTTONUP || event == WM_LBUTTONDBLCLK {
            restore_window_from_tray(hwnd);
            return 0;
        } else if event == WM_RBUTTONUP || event == WM_CONTEXTMENU {
            show_tray_context_menu(hwnd);
            return 0;
        }
    } else if msg == WM_COMMAND {
        let cmd_id = wparam & 0xFFFF;
        if cmd_id == ID_TRAY_OPEN {
            restore_window_from_tray(hwnd);
            return 0;
        } else if cmd_id == ID_TRAY_EXIT {
            remove_tray_icon_raw(hwnd);
            std::process::exit(0);
        }
    }

    let orig = ORIG_WNDPROC.load(Ordering::SeqCst);
    if orig != 0 {
        CallWindowProcW(orig, hwnd, msg, wparam, lparam)
    } else {
        0
    }
}

unsafe fn show_tray_context_menu(hwnd: isize) {
    let hmenu = CreatePopupMenu();
    if hmenu == 0 {
        return;
    }

    let open_txt = to_wide_chars("Open MyDiskly");
    let exit_txt = to_wide_chars("Exit MyDiskly");

    AppendMenuW(hmenu, MF_STRING, ID_TRAY_OPEN, open_txt.as_ptr());
    AppendMenuW(hmenu, MF_SEPARATOR, 0, std::ptr::null());
    AppendMenuW(hmenu, MF_STRING, ID_TRAY_EXIT, exit_txt.as_ptr());

    let mut pt = Point { x: 0, y: 0 };
    GetCursorPos(&mut pt);

    SetForegroundWindow(hwnd);
    let selected = TrackPopupMenu(
        hmenu,
        TPM_RIGHTBUTTON | TPM_RETURNCMD | TPM_NONOTIFY | TPM_BOTTOMALIGN,
        pt.x,
        pt.y,
        0,
        hwnd,
        std::ptr::null(),
    );
    DestroyMenu(hmenu);

    if selected as usize == ID_TRAY_OPEN {
        restore_window_from_tray(hwnd);
    } else if selected as usize == ID_TRAY_EXIT {
        remove_tray_icon_raw(hwnd);
        std::process::exit(0);
    }
}

/// Immediately restores the MyDiskly window on the very first click from the System Tray.
pub fn restore_window_from_tray(hwnd: isize) {
    let x = DOCK_X.load(Ordering::SeqCst) as i32;
    let y = DOCK_Y.load(Ordering::SeqCst) as i32;
    let w = DOCK_W.load(Ordering::SeqCst) as i32;
    let h = DOCK_H.load(Ordering::SeqCst) as i32;

    TRAY_RESTORE_PENDING.store(true, Ordering::SeqCst);

    // 1. Wake up egui / winit viewport state first so OpenGL swapchain is active
    if let Some(ctx) = EGUI_CTX.get() {
        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(egui::pos2(
            x as f32, y as f32,
        )));
        ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        ctx.request_repaint();
    }

    // 2. Enforce Win32 visibility, position, topmost Z-order, and immediate frame repaint
    if hwnd != 0 {
        unsafe {
            ShowWindow(hwnd, SW_RESTORE);
            ShowWindow(hwnd, SW_SHOWNORMAL);
            ShowWindow(hwnd, SW_SHOW);
            SetWindowPos(
                hwnd,
                HWND_TOPMOST,
                x,
                y,
                w,
                h,
                SWP_SHOWWINDOW | SWP_FRAMECHANGED,
            );
            BringWindowToTop(hwnd);
            SetForegroundWindow(hwnd);
            InvalidateRect(hwnd, std::ptr::null(), 1);
            UpdateWindow(hwnd);
        }
    }
}

pub fn hide_window_to_tray(hwnd: isize) {
    if hwnd != 0 {
        unsafe {
            ShowWindow(hwnd, SW_HIDE);
        }
        trim_working_set();
    }
}

fn remove_tray_icon_raw(hwnd: isize) {
    let mut nid: NOTIFYICONDATAW = unsafe { std::mem::zeroed() };
    nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
    nid.hWnd = hwnd;
    nid.uID = 1001;
    unsafe {
        Shell_NotifyIconW(NIM_DELETE, &nid);
    }
}

/// Finds the MyDiskly window handle, sets its blue square icon, subclasses WndProc for tray interaction,
/// and enforces right-docked vertical position & Always-On-Top level.
pub fn enforce_right_dock_topmost(x: i32, y: i32, w: i32, h: i32, activate: bool) -> isize {
    DOCK_X.store(x as isize, Ordering::SeqCst);
    DOCK_Y.store(y as isize, Ordering::SeqCst);
    DOCK_W.store(w as isize, Ordering::SeqCst);
    DOCK_H.store(h as isize, Ordering::SeqCst);

    let title = to_wide_chars("MyDiskly");
    let hwnd = unsafe { FindWindowW(std::ptr::null(), title.as_ptr()) };
    if hwnd != 0 {
        let hicon = get_mydiskly_hicon();
        if hicon != 0 {
            unsafe {
                SendMessageW(hwnd, WM_SETICON, 0, hicon);
                SendMessageW(hwnd, WM_SETICON, 1, hicon);
            }
        }

        if ORIG_WNDPROC.load(Ordering::SeqCst) == 0 {
            enable_window_dark_blur(hwnd);
            unsafe {
                let prev = SetWindowLongPtrW(
                    hwnd,
                    GWLP_WNDPROC,
                    tray_subclass_wndproc as *const () as usize as isize,
                );
                if prev != 0 {
                    ORIG_WNDPROC.store(prev, Ordering::SeqCst);
                }
            }
        }

        let flags = if activate {
            SWP_SHOWWINDOW
        } else {
            SWP_SHOWWINDOW | SWP_NOACTIVATE
        };
        unsafe {
            SetWindowPos(hwnd, HWND_TOPMOST, x, y, w, h, flags);
        }
    }
    hwnd
}

pub struct TrayManager {
    is_added: bool,
    hwnd: isize,
}

impl TrayManager {
    pub fn new() -> Self {
        Self {
            is_added: false,
            hwnd: 0,
        }
    }

    pub fn hwnd(&self) -> isize {
        self.hwnd
    }

    pub fn sync_tray(&mut self, enable: bool, hwnd: isize) {
        if hwnd != 0 {
            self.hwnd = hwnd;
        }
        if self.hwnd == 0 {
            return;
        }
        if enable && !self.is_added {
            self.add_icon();
        } else if !enable && self.is_added {
            self.remove_icon();
        }
    }

    fn create_base_nid(&self) -> NOTIFYICONDATAW {
        let mut nid: NOTIFYICONDATAW = unsafe { std::mem::zeroed() };
        nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
        nid.hWnd = self.hwnd;
        nid.uID = 1001;
        nid.uFlags = NIF_ICON | NIF_TIP | NIF_MESSAGE;
        nid.uCallbackMessage = WM_TRAYICON;
        nid.hIcon = get_mydiskly_hicon();

        let tip = to_wide_chars("MyDiskly");
        for (i, &ch) in tip.iter().take(127).enumerate() {
            nid.szTip[i] = ch;
        }
        nid
    }

    pub fn add_icon(&mut self) {
        if self.hwnd == 0 {
            return;
        }
        let nid = self.create_base_nid();
        let res = unsafe { Shell_NotifyIconW(NIM_ADD, &nid) };
        if res != 0 {
            self.is_added = true;
            register_app_identity();
        } else {
            let mod_res = unsafe { Shell_NotifyIconW(NIM_MODIFY, &nid) };
            if mod_res != 0 {
                self.is_added = true;
                register_app_identity();
            }
        }
    }

    pub fn remove_icon(&mut self) {
        if self.is_added && self.hwnd != 0 {
            remove_tray_icon_raw(self.hwnd);
            self.is_added = false;
        }
    }

    /// Triggers a Windows System Notification Toast/Balloon with title "MyDiskly" and plays `interface.wav`.
    pub fn trigger_cache_notification(&mut self, lang: &str, cache_size_str: &str) {
        if !is_system_notification_enabled() {
            return;
        }

        // Ensure registry DisplayName is "MyDiskly" (never mydiskly-app.exe)
        register_app_identity();

        // 1. Play custom interface.wav sound
        play_interface_sound();

        // 2. Ensure tray icon exists so Shell_NotifyIconW can display the system notification
        if !self.is_added {
            self.add_icon();
        }

        let mut nid = self.create_base_nid();
        nid.uFlags |= NIF_INFO;
        nid.dwInfoFlags = NIIF_INFO | NIIF_NOSOUND;

        let title_str = "MyDiskly";

        let body_str = if lang == "id" {
            format!(
                "Prepare to cleaning your disk.\nCache sistem hampir penuh ({}). Buka untuk membersihkan sekarang.",
                cache_size_str
            )
        } else {
            format!(
                "Prepare to cleaning your disk.\nYour cache is almost high ({}). Open to cleaning now.",
                cache_size_str
            )
        };

        let wide_title = to_wide_chars(title_str);
        for (i, &ch) in wide_title.iter().take(63).enumerate() {
            nid.szInfoTitle[i] = ch;
        }

        let wide_body = to_wide_chars(&body_str);
        for (i, &ch) in wide_body.iter().take(255).enumerate() {
            nid.szInfo[i] = ch;
        }

        unsafe {
            Shell_NotifyIconW(NIM_MODIFY, &nid);
        }
    }
}

impl Drop for TrayManager {
    fn drop(&mut self) {
        self.remove_icon();
    }
}
