#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ffi::c_void;
use std::fs;
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use egui::{
    pos2, vec2, Align2, Color32, FontData, FontDefinitions, FontFamily, FontId, Frame, Margin,
    Pos2, Rect, RichText, Rounding, Sense, Stroke, TextureHandle, Ui,
};
use mydiskly_core::disk_telemetry;
use mydiskly_core::gatekeeper::{enforce_os_gatekeeping, to_wide_chars};
use mydiskly_core::hardware_specs::audit_system_specs;
use mydiskly_core::ini_manager;
use mydiskly_core::smart_diagnostics;
use mydiskly_core::AppConfig;
use windows_sys::Win32::Foundation::CloseHandle;
use windows_sys::Win32::Graphics::Dwm::DwmSetWindowAttribute;
use windows_sys::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE};
use windows_sys::Win32::UI::Shell::ShellExecuteW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    FindWindowW, GetWindowThreadProcessId, PostMessageW, ShowWindow, SystemParametersInfoW,
    SPI_GETWORKAREA, SW_HIDE, SW_MINIMIZE, SW_SHOWNORMAL, WM_CLOSE, WM_USER,
};
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

const CREATE_NO_WINDOW: u32 = 0x08000000;
const WM_MYDISKLY_FORCE_QUIT: u32 = WM_USER + 99;

const FONT_REGULAR: &[u8] =
    include_bytes!("../../mydiskly-app/assets/fonts/PlusJakartaSans-Regular.ttf");
const FONT_BOLD: &[u8] =
    include_bytes!("../../mydiskly-app/assets/fonts/PlusJakartaSans-Bold.ttf");
const ICON_RGBA_64: &[u8] = include_bytes!("../../mydiskly-app/assets/app_icon_64x64.rgba");
const EMBEDDED_APP_EXE: &[u8] = include_bytes!("../../../target/release/MyDiskly.exe");

const LICENSE_TEXT: &str = "MyDiskly Modern Storage Suite (v1.1.2026)\r\n\
Publisher: Seroja Labs (https://serojalabs.sbs/mydiskly)\r\n\
Repository: https://github.com/darkostepbro/mydiskly-desktop-id\r\n\
License: GNU General Public License v3.0 (GPL-3.0)\r\n";

#[repr(C)]
struct BrowseInfoW {
    hwnd_owner: isize,
    pidl_root: *mut c_void,
    psz_display_name: *mut u16,
    lpsz_title: *const u16,
    ul_flags: u32,
    lpfn: *const c_void,
    l_param: isize,
    i_image: i32,
}

#[link(name = "shell32")]
extern "system" {
    fn SHBrowseForFolderW(lpbi: *const BrowseInfoW) -> *mut c_void;
    fn SHGetPathFromIDListW(pidl: *const c_void, pszpath: *mut u16) -> i32;
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum WizardStep {
    Welcome,
    Loading,
    Complete,
}

struct PipelineProgress {
    progress: f32,
    step_index: usize,
    status: String,
    finished: bool,
}

struct MyDisklyInstallerApp {
    is_uninstall_mode: bool,
    step: WizardStep,
    step_transition_time: Instant,
    install_dir: PathBuf,
    progress_state: Arc<Mutex<PipelineProgress>>,
    displayed_progress: f32,
    create_shortcut: bool,
    pin_start_menu: bool,
    run_as_admin: bool,
    open_file_location: bool,
    hwnd: isize,
    hwnd_configured: bool,
    logo_texture: TextureHandle,
    relocated_uninstaller_temp: Arc<Mutex<Option<PathBuf>>>,
}

impl MyDisklyInstallerApp {
    fn new(
        cc: &eframe::CreationContext<'_>,
        is_uninstall_mode: bool,
        detected_install_dir: PathBuf,
    ) -> Self {
        setup_installer_fonts(&cc.egui_ctx);
        setup_installer_visuals(&cc.egui_ctx);

        let logo_image = egui::ColorImage::from_rgba_unmultiplied([64, 64], ICON_RGBA_64);
        let logo_texture = cc.egui_ctx.load_texture(
            "mydiskly_installer_logo",
            logo_image,
            egui::TextureOptions::LINEAR,
        );

        Self {
            is_uninstall_mode,
            step: WizardStep::Welcome,
            step_transition_time: Instant::now(),
            install_dir: detected_install_dir,
            progress_state: Arc::new(Mutex::new(PipelineProgress {
                progress: 0.0,
                step_index: 0,
                status: "Preparing environment...".to_string(),
                finished: false,
            })),
            displayed_progress: 0.0,
            create_shortcut: true,
            pin_start_menu: false,
            run_as_admin: true,
            open_file_location: false,
            hwnd: 0,
            hwnd_configured: false,
            logo_texture,
            relocated_uninstaller_temp: Arc::new(Mutex::new(None)),
        }
    }

    fn start_install_pipeline(&mut self) {
        self.step = WizardStep::Loading;
        self.step_transition_time = Instant::now();
        self.displayed_progress = 0.02;
        let state = Arc::clone(&self.progress_state);
        let target_dir = self.install_dir.clone();

        thread::spawn(move || {
            let update_step = |idx: usize, prog: f32, msg: &str| {
                if let Ok(mut lock) = state.lock() {
                    lock.step_index = idx;
                    lock.progress = prog;
                    lock.status = msg.to_string();
                }
            };

            // Step 1: Stop any running MyDiskly instance & verify platform
            update_step(0, 0.20, "Verifying Windows 10 / 11 (x64) environment...");
            force_kill_running_mydiskly(&target_dir);
            thread::sleep(Duration::from_millis(260));

            // Step 2: Deploy MyDiskly.exe, Uninstall.exe, LICENSE.txt & mydiskly.ini
            update_step(
                1,
                0.50,
                "Deploying MyDiskly.exe, Uninstall.exe & configuration...",
            );
            let _ = fs::create_dir_all(&target_dir);
            let _ = fs::remove_file(target_dir.join("specs_cache.json"));

            let target_exe = target_dir.join("MyDiskly.exe");
            let _ = fs::write(&target_exe, EMBEDDED_APP_EXE);
            let _ = fs::write(target_dir.join("LICENSE.txt"), LICENSE_TEXT);

            let uninst_exe = target_dir.join("Uninstall.exe");
            if let Ok(current_exe) = std::env::current_exe() {
                if current_exe != uninst_exe {
                    let _ = fs::copy(&current_exe, &uninst_exe);
                }
            }

            let default_cfg = AppConfig::default();
            let _ = ini_manager::save_config(&default_cfg);
            let local_ini = ini_manager::get_ini_path();
            let cfg_path = target_dir.join("mydiskly.ini");
            if !cfg_path.exists() && local_ini.exists() {
                let _ = fs::copy(&local_ini, &cfg_path);
            }
            thread::sleep(Duration::from_millis(320));

            // Step 3: Hardware Diagnostic & myspecs.ini + historydiskhealth.ini
            update_step(
                2,
                0.80,
                "Generating myspecs.ini & historydiskhealth.ini...",
            );
            let specs = audit_system_specs(true);
            let partitions = disk_telemetry::get_partitions();
            let smart_disks = smart_diagnostics::get_smart_disks();
            let _ = ini_manager::sync_myspecs_ini(&specs);
            let _ = ini_manager::sync_history_disk_health_ini(&partitions, &smart_disks);

            let local_specs = ini_manager::get_myspecs_ini_path();
            if local_specs.exists() {
                let _ = fs::copy(&local_specs, target_dir.join("myspecs.ini"));
            }
            let local_hist = ini_manager::get_history_health_ini_path();
            if local_hist.exists() {
                let _ = fs::copy(&local_hist, target_dir.join("historydiskhealth.ini"));
            }
            thread::sleep(Duration::from_millis(320));

            // Step 4: Register Uninstaller in Windows Registry
            update_step(
                3,
                1.00,
                "Registering application in Windows Registry...",
            );
            register_uninstaller(&target_dir);
            thread::sleep(Duration::from_millis(260));

            if let Ok(mut lock) = state.lock() {
                lock.step_index = 4;
                lock.progress = 1.0;
                lock.status = "Installation completed successfully!".to_string();
                lock.finished = true;
            }
        });
    }

    fn start_uninstall_pipeline(&mut self) {
        self.step = WizardStep::Loading;
        self.step_transition_time = Instant::now();
        self.displayed_progress = 0.02;
        let state = Arc::clone(&self.progress_state);
        let target_dir = self.install_dir.clone();
        let relocated_arc = Arc::clone(&self.relocated_uninstaller_temp);

        thread::spawn(move || {
            let update_step = |idx: usize, prog: f32, msg: &str| {
                if let Ok(mut lock) = state.lock() {
                    lock.step_index = idx;
                    lock.progress = prog;
                    lock.status = msg.to_string();
                }
            };

            // Step 1: Force-terminate all running MyDiskly processes (even if elevated or hidden in Tray)
            update_step(0, 0.25, "Force-closing active MyDiskly processes...");
            force_kill_running_mydiskly(&target_dir);
            thread::sleep(Duration::from_millis(320));

            // Step 2: Remove Desktop & Start Menu shortcuts
            update_step(1, 0.50, "Removing Desktop & Start Menu shortcuts...");
            if let Ok(userprofile) = std::env::var("USERPROFILE") {
                let desktop_lnk = Path::new(&userprofile).join("Desktop").join("MyDiskly.lnk");
                let _ = fs::remove_file(desktop_lnk);
            }
            if let Ok(appdata) = std::env::var("APPDATA") {
                let sm_dir =
                    Path::new(&appdata).join(r"Microsoft\Windows\Start Menu\Programs\MyDiskly");
                let _ = fs::remove_dir_all(sm_dir);
            }
            thread::sleep(Duration::from_millis(260));

            // Step 3: Clean up Windows Registry keys
            update_step(2, 0.75, "Cleaning up Windows Registry entries...");
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            let _ = hkcu
                .delete_subkey_all(r"Software\Microsoft\Windows\CurrentVersion\Uninstall\MyDiskly");
            if let Ok((run_key, _)) =
                hkcu.create_subkey(r"Software\Microsoft\Windows\CurrentVersion\Run")
            {
                let _ = run_key.delete_value("MyDiskly");
            }
            thread::sleep(Duration::from_millis(260));

            // Step 4: Permanently delete all files, Uninstall.exe, and the MyDiskly directory immediately
            update_step(3, 1.00, "Deleting all MyDiskly files & Uninstaller...");
            for file_name in [
                "MyDiskly.exe",
                "mydiskly.ini",
                "myspecs.ini",
                "historydiskhealth.ini",
                "LICENSE.txt",
                "specs_cache.json",
            ] {
                let p = target_dir.join(file_name);
                if p.exists() && fs::remove_file(&p).is_err() {
                    // Retry after a brief pause in case OS handle is still releasing
                    thread::sleep(Duration::from_millis(200));
                    let _ = fs::remove_file(&p);
                }
            }

            // Relocate Uninstall.exe out of target_dir into %TEMP% so Uninstall.exe and target_dir
            // are IMMEDIATELY deleted from AppData\Local\Programs\MyDiskly right now!
            let uninst_path = target_dir.join("Uninstall.exe");
            if uninst_path.exists() {
                if fs::remove_file(&uninst_path).is_err() {
                    let temp_relocated = std::env::temp_dir()
                        .join(format!("mydiskly_uninst_cleanup_{}.exe", std::process::id()));
                    let _ = fs::remove_file(&temp_relocated);
                    if fs::rename(&uninst_path, &temp_relocated).is_ok() {
                        if let Ok(mut lock) = relocated_arc.lock() {
                            *lock = Some(temp_relocated);
                        }
                    }
                }
            }

            // Now remove the entire target_dir immediately!
            let _ = fs::remove_dir_all(&target_dir);
            thread::sleep(Duration::from_millis(260));

            if let Ok(mut lock) = state.lock() {
                lock.step_index = 4;
                lock.progress = 1.0;
                lock.status = "MyDiskly has been completely removed.".to_string();
                lock.finished = true;
            }
        });
    }

    fn finish_and_launch(&self, ctx: &egui::Context) {
        if self.is_uninstall_mode {
            self.spawn_final_self_cleanup();
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        let target_exe = self.install_dir.join("MyDiskly.exe");

        // 1. Create Desktop shortcut silently in background (CREATE_NO_WINDOW)
        if self.create_shortcut {
            if let Ok(userprofile) = std::env::var("USERPROFILE") {
                let desktop_lnk = Path::new(&userprofile).join("Desktop").join("MyDiskly.lnk");
                create_shortcut_silent(&target_exe, &desktop_lnk);
            }
        }

        // 2. Pin on Start Menu silently in background (CREATE_NO_WINDOW)
        if self.pin_start_menu {
            if let Ok(appdata) = std::env::var("APPDATA") {
                let sm_dir =
                    Path::new(&appdata).join(r"Microsoft\Windows\Start Menu\Programs\MyDiskly");
                let _ = fs::create_dir_all(&sm_dir);
                let sm_lnk = sm_dir.join("MyDiskly.lnk");
                create_shortcut_silent(&target_exe, &sm_lnk);
            }
        }

        // 3. Open File Location via ShellExecuteW (no CMD window)
        if self.open_file_location {
            open_shell_target(&self.install_dir.to_string_lossy());
        }

        // 4. Launch MyDiskly.exe (as Administrator if toggled ON)
        if target_exe.exists() {
            if self.run_as_admin {
                let verb = to_wide_chars("runas");
                let file = to_wide_chars(&target_exe.to_string_lossy());
                let dir = to_wide_chars(&self.install_dir.to_string_lossy());
                let res = unsafe {
                    ShellExecuteW(
                        0,
                        verb.as_ptr(),
                        file.as_ptr(),
                        std::ptr::null(),
                        dir.as_ptr(),
                        SW_SHOWNORMAL,
                    )
                };
                if res <= 32 {
                    let _ = Command::new(&target_exe)
                        .current_dir(&self.install_dir)
                        .spawn();
                }
            } else {
                let _ = Command::new(&target_exe)
                    .current_dir(&self.install_dir)
                    .spawn();
            }
        }

        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }

    fn spawn_final_self_cleanup(&self) {
        let uninst_in_dir = self.install_dir.join("Uninstall.exe");
        let relocated = self
            .relocated_uninstaller_temp
            .lock()
            .ok()
            .and_then(|g| g.clone());

        let mut del_cmds = format!(
            "ping 127.0.0.1 -n 2 > nul & del /f /q \"{}\" > nul 2>&1 & rmdir /s /q \"{}\" > nul 2>&1",
            uninst_in_dir.to_string_lossy(),
            self.install_dir.to_string_lossy()
        );
        if let Some(rel_path) = relocated {
            del_cmds.push_str(&format!(
                " & del /f /q \"{}\" > nul 2>&1",
                rel_path.to_string_lossy()
            ));
        }

        let _ = Command::new("cmd.exe")
            .args(["/C", &del_cmds])
            .current_dir(std::env::temp_dir())
            .creation_flags(CREATE_NO_WINDOW)
            .spawn();
    }
}

impl Drop for MyDisklyInstallerApp {
    fn drop(&mut self) {
        if self.is_uninstall_mode && self.step == WizardStep::Complete {
            self.spawn_final_self_cleanup();
        }
    }
}

impl eframe::App for MyDisklyInstallerApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.051, 0.055, 0.067, 1.0]
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.hwnd_configured {
            let win_title = if self.is_uninstall_mode {
                "MyDiskly Uninstaller"
            } else {
                "MyDiskly Installer"
            };
            let title_w = to_wide_chars(win_title);
            let hwnd = unsafe { FindWindowW(std::ptr::null(), title_w.as_ptr()) };
            if hwnd != 0 {
                self.hwnd = hwnd;
                configure_native_window_corners(hwnd);
                self.hwnd_configured = true;
            }
        }

        let mut status_text = String::new();
        let mut current_step_idx = 0usize;
        if self.step == WizardStep::Loading {
            let (target_prog, idx, msg, done) = {
                let lock = self
                    .progress_state
                    .lock()
                    .unwrap_or_else(|e| e.into_inner());
                (lock.progress, lock.step_index, lock.status.clone(), lock.finished)
            };
            status_text = msg;
            current_step_idx = idx;
            self.displayed_progress += (target_prog - self.displayed_progress) * 0.18;
            if (self.displayed_progress - target_prog).abs() < 0.004 {
                self.displayed_progress = target_prog;
            }
            if done && self.displayed_progress >= 0.99 {
                self.displayed_progress = 1.0;
                self.step = WizardStep::Complete;
                self.step_transition_time = Instant::now();
            }
            ctx.request_repaint_after(Duration::from_millis(16));
        } else if self.step_transition_time.elapsed().as_secs_f32() < 0.25 {
            ctx.request_repaint_after(Duration::from_millis(16));
        }

        // =====================================================================
        // OUTER WINDOW BORDER & ANCHORED HEADER / FOOTER / CONTENT PANELS
        // =====================================================================
        egui::TopBottomPanel::top("installer_top_panel")
            .frame(
                Frame::none()
                    .fill(Color32::from_rgb(13, 14, 17))
                    .inner_margin(Margin {
                        left: 20.0,
                        right: 20.0,
                        top: 14.0,
                        bottom: 8.0,
                    }),
            )
            .show_separator_line(false)
            .show(ctx, |ui| {
                // 1. Minimalist Draggable Header Bar
                let top_resp = ui
                    .horizontal(|ui| {
                        ui.add(egui::Image::new((
                            self.logo_texture.id(),
                            vec2(20.0, 20.0),
                        )));
                        ui.add_space(6.0);
                        ui.colored_label(
                            Color32::from_rgb(248, 250, 252),
                            RichText::new("MyDiskly").strong().size(15.0),
                        );

                        Frame::none()
                            .fill(Color32::from_rgb(24, 26, 33))
                            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(44, 48, 58)))
                            .rounding(Rounding::same(10.0))
                            .inner_margin(Margin::symmetric(6.0, 1.5))
                            .show(ui, |ui| {
                                ui.colored_label(
                                    Color32::from_rgb(161, 166, 178),
                                    RichText::new("v1.1").strong().size(9.5),
                                );
                            });

                        ui.colored_label(
                            Color32::from_rgb(112, 118, 130),
                            RichText::new(if self.is_uninstall_mode {
                                "Uninstaller"
                            } else {
                                "Installer"
                            })
                            .size(11.5),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if self.step != WizardStep::Loading {
                                if draw_window_icon_btn(ui, true).clicked() {
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                }
                                ui.add_space(4.0);
                                if draw_window_icon_btn(ui, false).clicked() {
                                    if self.hwnd != 0 {
                                        unsafe {
                                            ShowWindow(self.hwnd, SW_MINIMIZE);
                                        }
                                    } else {
                                        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(
                                            true,
                                        ));
                                    }
                                }
                            }
                        });
                    })
                    .response;

                let drag_zone = ui.interact(
                    Rect::from_min_size(
                        top_resp.rect.min,
                        vec2(top_resp.rect.width() - 60.0, top_resp.rect.height()),
                    ),
                    ui.id().with("hdr_drag"),
                    Sense::click_and_drag(),
                );
                if drag_zone.drag_started() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }

                ui.add_space(10.0);

                // 2. Mathematically Exact 3-Step Roadmap Pill Bar (Zero Overflow, Pure Vector Checkmarks)
                render_exact_roadmap_bar(ui, self.step, self.is_uninstall_mode);
            });

        // Anchored Bottom Action Bar (Guaranteed never clipped at bottom!)
        if self.step != WizardStep::Loading {
            egui::TopBottomPanel::bottom("installer_bottom_panel")
                .frame(
                    Frame::none()
                        .fill(Color32::from_rgb(13, 14, 17))
                        .inner_margin(Margin {
                            left: 20.0,
                            right: 20.0,
                            top: 6.0,
                            bottom: 14.0,
                        }),
                )
                .show_separator_line(false)
                .show(ctx, |ui| {
                    self.render_bottom_bar(ui, ctx);
                });
        }

        // Central Content Area
        egui::CentralPanel::default()
            .frame(
                Frame::none()
                    .fill(Color32::from_rgb(13, 14, 17))
                    .inner_margin(Margin {
                        left: 20.0,
                        right: 20.0,
                        top: 4.0,
                        bottom: 4.0,
                    }),
            )
            .show(ctx, |ui| {
                let anim_t =
                    (self.step_transition_time.elapsed().as_secs_f32() / 0.18).clamp(0.0, 1.0);
                ui.set_opacity(0.4 + 0.6 * anim_t);

                match self.step {
                    WizardStep::Welcome => {
                        self.render_welcome_body(ui);
                    }
                    WizardStep::Loading => {
                        self.render_loading_body(ui, &status_text, current_step_idx);
                    }
                    WizardStep::Complete => {
                        self.render_complete_body(ui);
                    }
                }
            });

        // Crisp 1px outer window border painted on the foreground layer
        let screen_rect = ctx.screen_rect();
        ctx.layer_painter(egui::LayerId::new(
            egui::Order::Foreground,
            egui::Id::new("window_border_layer"),
        ))
        .rect_stroke(
            screen_rect.shrink(0.5),
            Rounding::same(10.0),
            Stroke::new(1.0_f32, Color32::from_rgb(38, 42, 52)),
        );
    }
}

impl MyDisklyInstallerApp {
    fn render_welcome_body(&mut self, ui: &mut Ui) {
        // Minimalist Hero Card
        Frame::none()
            .fill(Color32::from_rgb(18, 20, 25))
            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(32, 36, 45)))
            .rounding(Rounding::same(10.0))
            .inner_margin(Margin::symmetric(14.0, 12.0))
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.add(egui::Image::new((
                        self.logo_texture.id(),
                        vec2(40.0, 40.0),
                    )));
                    ui.add_space(10.0);
                    ui.vertical(|ui| {
                        ui.colored_label(
                            Color32::from_rgb(248, 250, 252),
                            RichText::new(if self.is_uninstall_mode {
                                "Uninstall MyDiskly Storage Suite"
                            } else {
                                "MyDiskly Modern Storage Suite"
                            })
                            .strong()
                            .size(14.5),
                        );
                        ui.add_space(2.0);
                        ui.colored_label(
                            Color32::from_rgb(142, 148, 162),
                            RichText::new(if self.is_uninstall_mode {
                                "Completely remove MyDiskly, shortcuts, and registry entries."
                            } else {
                                "A Super Lightweight Smart Disk Scanner & Cleaner in one app."
                            })
                            .size(11.0),
                        );
                    });
                });
            });

        ui.add_space(8.0);

        // Minimalist Directory Selector Card
        Frame::none()
            .fill(Color32::from_rgb(18, 20, 25))
            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(32, 36, 45)))
            .rounding(Rounding::same(10.0))
            .inner_margin(Margin::symmetric(14.0, 10.0))
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.colored_label(
                        Color32::from_rgb(130, 136, 150),
                        RichText::new(if self.is_uninstall_mode {
                            "INSTALLED LOCATION"
                        } else {
                            "INSTALLATION DIRECTORY"
                        })
                        .strong()
                        .size(9.5),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.colored_label(
                            Color32::from_rgb(110, 116, 130),
                            RichText::new("Size: ~10.5 MB").size(9.5),
                        );
                    });
                });

                ui.add_space(6.0);

                ui.horizontal(|ui| {
                    let avail = ui.available_width();
                    let btn_space = if self.is_uninstall_mode { 0.0 } else { 88.0 };
                    let path_w = (avail - btn_space).max(160.0);

                    Frame::none()
                        .fill(Color32::from_rgb(12, 13, 16))
                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(36, 40, 50)))
                        .rounding(Rounding::same(7.0))
                        .inner_margin(Margin::symmetric(10.0, 6.5))
                        .show(ui, |ui| {
                            ui.set_width(path_w - 20.0);
                            ui.colored_label(
                                Color32::from_rgb(220, 224, 232),
                                RichText::new(self.install_dir.to_string_lossy()).size(10.5),
                            );
                        });

                    if !self.is_uninstall_mode {
                        let browse_btn = egui::Button::new(
                            RichText::new("Browse")
                                .color(Color32::from_rgb(235, 238, 245))
                                .strong()
                                .size(10.5),
                        )
                        .fill(Color32::from_rgb(28, 32, 40))
                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(50, 56, 68)))
                        .rounding(Rounding::same(14.0))
                        .min_size(vec2(78.0, 28.0));

                        let resp = ui.add(browse_btn);
                        if resp.hovered() {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                        }
                        if resp.clicked() {
                            if let Some(chosen) = browse_for_install_folder(self.hwnd) {
                                self.install_dir = chosen;
                            }
                        }
                    }
                });
            });
    }

    fn render_loading_body(&mut self, ui: &mut Ui, status_text: &str, current_step_idx: usize) {
        Frame::none()
            .fill(Color32::from_rgb(18, 20, 25))
            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(32, 36, 45)))
            .rounding(Rounding::same(10.0))
            .inner_margin(Margin::symmetric(16.0, 14.0))
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());

                let pct = (self.displayed_progress * 100.0).round() as u32;
                ui.horizontal(|ui| {
                    ui.colored_label(
                        Color32::from_rgb(235, 238, 245),
                        RichText::new(status_text).strong().size(11.5),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.colored_label(
                            Color32::from_rgb(56, 189, 248),
                            RichText::new(format!("{}%", pct)).strong().size(13.0),
                        );
                    });
                });

                ui.add_space(8.0);

                // Full-Rounded Smooth Progress Bar
                let (bar_rect, _) =
                    ui.allocate_exact_size(vec2(ui.available_width(), 8.0), Sense::hover());
                let painter = ui.painter();
                painter.rect_filled(
                    bar_rect,
                    Rounding::same(4.0),
                    Color32::from_rgb(28, 32, 40),
                );
                let fill_w =
                    (bar_rect.width() * self.displayed_progress.clamp(0.0, 1.0)).max(8.0);
                painter.rect_filled(
                    Rect::from_min_size(bar_rect.min, vec2(fill_w, bar_rect.height())),
                    Rounding::same(4.0),
                    Color32::from_rgb(37, 99, 235),
                );

                ui.add_space(12.0);

                let steps: &[&str] = if self.is_uninstall_mode {
                    &[
                        "Force-terminate active MyDiskly background processes",
                        "Remove Desktop & Start Menu shortcuts",
                        "Clean up Windows Registry entries",
                        "Delete all MyDiskly binaries, configs & Uninstaller",
                    ]
                } else {
                    &[
                        "Verify 64-bit Windows 10 / 11 compatibility",
                        "Deploy MyDiskly.exe, Uninstall.exe & configuration",
                        "Audit hardware & generate myspecs.ini + historydiskhealth.ini",
                        "Register application & uninstaller in Windows Registry",
                    ]
                };

                for (i, label) in steps.iter().enumerate() {
                    let is_done = current_step_idx > i;
                    let is_active = current_step_idx == i;

                    ui.horizontal(|ui| {
                        let (icon_rect, _) =
                            ui.allocate_exact_size(vec2(16.0, 16.0), Sense::hover());
                        let p = ui.painter();
                        let c = icon_rect.center();

                        if is_done {
                            p.circle_filled(c, 7.0, Color32::from_rgb(16, 185, 129));
                            draw_vector_checkmark(p, c, 1.4, Color32::WHITE);
                        } else if is_active {
                            p.circle_filled(
                                c,
                                7.0,
                                Color32::from_rgba_unmultiplied(56, 189, 248, 45),
                            );
                            p.circle_filled(c, 3.5, Color32::from_rgb(56, 189, 248));
                        } else {
                            p.circle_stroke(
                                c,
                                5.5,
                                Stroke::new(1.2_f32, Color32::from_rgb(75, 82, 96)),
                            );
                        }

                        ui.add_space(4.0);
                        let txt_col = if is_done {
                            Color32::from_rgb(208, 214, 224)
                        } else if is_active {
                            Color32::from_rgb(248, 250, 252)
                        } else {
                            Color32::from_rgb(112, 118, 132)
                        };
                        ui.colored_label(txt_col, RichText::new(*label).size(11.0).strong());
                    });
                    ui.add_space(4.0);
                }
            });
    }

    fn render_complete_body(&mut self, ui: &mut Ui) {
        // 1. Minimalist Completion Status Card with Pure Vector Checkmark
        Frame::none()
            .fill(Color32::from_rgb(16, 26, 22))
            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(26, 62, 46)))
            .rounding(Rounding::same(10.0))
            .inner_margin(Margin::symmetric(14.0, 10.0))
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.horizontal(|ui| {
                    let (chk_rect, _) = ui.allocate_exact_size(vec2(22.0, 22.0), Sense::hover());
                    let p = ui.painter();
                    let c = chk_rect.center();
                    p.circle_filled(c, 9.5, Color32::from_rgb(16, 185, 129));
                    draw_vector_checkmark(p, c, 1.6, Color32::WHITE);

                    ui.add_space(6.0);
                    ui.vertical(|ui| {
                        ui.colored_label(
                            Color32::from_rgb(240, 253, 244),
                            RichText::new(if self.is_uninstall_mode {
                                "MyDiskly v1.1 has been permanently uninstalled."
                            } else {
                                "MyDiskly v1.1 has been installed successfully!"
                            })
                            .strong()
                            .size(12.0),
                        );
                        ui.colored_label(
                            Color32::from_rgb(142, 174, 158),
                            RichText::new(if self.is_uninstall_mode {
                                "All files, shortcuts, and registry entries were removed.".to_string()
                            } else {
                                format!("Installed to: {}", self.install_dir.display())
                            })
                            .size(10.0),
                        );
                    });
                });
            });

        if !self.is_uninstall_mode {
            ui.add_space(8.0);

            // 2. Minimalist 2x2 Grid with FULL-ROUNDED Pill Toggle Switches
            Frame::none()
                .fill(Color32::from_rgb(18, 20, 25))
                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(32, 36, 45)))
                .rounding(Rounding::same(10.0))
                .inner_margin(Margin::symmetric(14.0, 11.0))
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    let col_w = (ui.available_width() - 12.0) * 0.5;

                    ui.horizontal(|ui| {
                        ui.allocate_ui(vec2(col_w, 22.0), |ui| {
                            draw_full_rounded_toggle(
                                ui,
                                "tog_shortcut",
                                &mut self.create_shortcut,
                                "Create the shortcut",
                                None,
                            );
                        });
                        ui.allocate_ui(vec2(col_w, 22.0), |ui| {
                            draw_full_rounded_toggle(
                                ui,
                                "tog_admin",
                                &mut self.run_as_admin,
                                "Run as ",
                                Some("Administrator"),
                            );
                        });
                    });

                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        ui.allocate_ui(vec2(col_w, 22.0), |ui| {
                            draw_full_rounded_toggle(
                                ui,
                                "tog_startmenu",
                                &mut self.pin_start_menu,
                                "Pin on Start Menu",
                                None,
                            );
                        });
                        ui.allocate_ui(vec2(col_w, 22.0), |ui| {
                            draw_full_rounded_toggle(
                                ui,
                                "tog_openloc",
                                &mut self.open_file_location,
                                "Open File Location",
                                None,
                            );
                        });
                    });
                });
        }
    }

    fn render_bottom_bar(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        ui.horizontal(|ui| {
            if self.step == WizardStep::Complete {
                ui.colored_label(
                    Color32::from_rgb(115, 122, 136),
                    RichText::new("(C) 2026 MyDiskly  |").size(10.0),
                );
                for (label, url) in [
                    ("Website", "https://serojalabs.sbs/mydiskly"),
                    ("GitHub", "https://github.com/darkostepbro/mydiskly-desktop-id"),
                    ("Follow us", "https://github.com/darkostepbro"),
                ] {
                    let resp = ui.add(
                        egui::Label::new(
                            RichText::new(label)
                                .color(Color32::from_rgb(158, 165, 180))
                                .underline()
                                .size(10.0),
                        )
                        .sense(Sense::click()),
                    );
                    if resp.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }
                    if resp.clicked() {
                        open_shell_target(url);
                    }
                    ui.add_space(4.0);
                }
            } else {
                ui.colored_label(
                    Color32::from_rgb(112, 118, 132),
                    RichText::new("Seroja Labs  |  GPL-3.0 Open Source").size(10.5),
                );
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let (btn_txt, btn_bg, btn_border) = match (self.step, self.is_uninstall_mode) {
                    (WizardStep::Welcome, false) => (
                        "Install Now",
                        Color32::from_rgb(37, 99, 235),
                        Color32::from_rgb(96, 165, 250),
                    ),
                    (WizardStep::Welcome, true) => (
                        "Uninstall Now",
                        Color32::from_rgb(220, 38, 38),
                        Color32::from_rgb(248, 113, 113),
                    ),
                    (WizardStep::Complete, false) => (
                        "START",
                        Color32::from_rgb(37, 99, 235),
                        Color32::from_rgb(96, 165, 250),
                    ),
                    _ => (
                        "Finish",
                        Color32::from_rgb(37, 99, 235),
                        Color32::from_rgb(96, 165, 250),
                    ),
                };

                let btn = egui::Button::new(
                    RichText::new(btn_txt)
                        .color(Color32::WHITE)
                        .strong()
                        .size(11.5),
                )
                .fill(btn_bg)
                .stroke(Stroke::new(1.0_f32, btn_border))
                .rounding(Rounding::same(16.0)) // Full-rounded pill CTA button
                .min_size(vec2(128.0, 32.0));

                let resp = ui.add(btn);
                if resp.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                if resp.clicked() {
                    match self.step {
                        WizardStep::Welcome => {
                            if self.is_uninstall_mode {
                                self.start_uninstall_pipeline();
                            } else {
                                self.start_install_pipeline();
                            }
                        }
                        WizardStep::Complete => {
                            self.finish_and_launch(ctx);
                        }
                        WizardStep::Loading => {}
                    }
                }
            });
        });
    }
}

/// Renders the 3-step roadmap bar using exact proportional coordinates so it never overflows on the right
/// and uses pure vector checkmarks instead of missing font glyphs.
fn render_exact_roadmap_bar(ui: &mut Ui, current_step: WizardStep, is_uninstall: bool) {
    let avail_w = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(vec2(avail_w, 34.0), Sense::hover());
    let painter = ui.painter();

    // Full-rounded capsule container
    painter.rect_filled(
        rect,
        Rounding::same(17.0),
        Color32::from_rgb(18, 20, 26),
    );
    painter.rect_stroke(
        rect,
        Rounding::same(17.0),
        Stroke::new(1.0_f32, Color32::from_rgb(32, 36, 46)),
    );

    let labels = if is_uninstall {
        ["Confirm", "Uninstalling", "Complete"]
    } else {
        ["Welcome", "Installing", "Complete"]
    };

    let cur_idx = match current_step {
        WizardStep::Welcome => 0,
        WizardStep::Loading => 1,
        WizardStep::Complete => 2,
    };

    let cy = rect.center().y;
    let x_positions = [
        rect.min.x + 18.0,
        rect.center().x - 32.0,
        rect.max.x - 86.0,
    ];

    let mut step_right_edges = [0.0f32; 3];
    let mut step_left_edges = [0.0f32; 3];

    for i in 0..3 {
        let is_done = i < cur_idx || current_step == WizardStep::Complete;
        let is_active = i == cur_idx && current_step != WizardStep::Complete;

        let cx = x_positions[i] + 8.0;
        step_left_edges[i] = cx - 10.0;

        let (circle_bg, text_col) = if is_done {
            (
                Color32::from_rgb(16, 185, 129),
                Color32::from_rgb(212, 218, 228),
            )
        } else if is_active {
            (
                Color32::from_rgb(37, 99, 235),
                Color32::from_rgb(248, 250, 252),
            )
        } else {
            (
                Color32::from_rgb(32, 36, 46),
                Color32::from_rgb(115, 122, 136),
            )
        };

        painter.circle_filled(pos2(cx, cy), 8.5, circle_bg);
        if is_done {
            draw_vector_checkmark(painter, pos2(cx, cy), 1.5, Color32::WHITE);
        } else {
            painter.text(
                pos2(cx, cy),
                Align2::CENTER_CENTER,
                format!("{}", i + 1),
                FontId::new(9.5, FontFamily::Name("Bold".into())),
                if is_active {
                    Color32::WHITE
                } else {
                    Color32::from_rgb(142, 148, 162)
                },
            );
        }

        let galley = painter.layout_no_wrap(
            labels[i].to_string(),
            FontId::new(11.0, FontFamily::Name("Bold".into())),
            text_col,
        );
        let tx = cx + 13.0;
        let tw = galley.size().x;
        painter.galley(pos2(tx, cy - galley.size().y * 0.5), galley, text_col);
        step_right_edges[i] = tx + tw + 8.0;
    }

    // Draw connecting progress lines between Step 1 -> 2 and Step 2 -> 3
    for i in 0..2 {
        let start_x = step_right_edges[i] + 4.0;
        let end_x = step_left_edges[i + 1] - 4.0;
        if end_x > start_x {
            let line_col = if cur_idx > i || current_step == WizardStep::Complete {
                Color32::from_rgb(16, 185, 129)
            } else if cur_idx == i {
                Color32::from_rgb(59, 130, 246)
            } else {
                Color32::from_rgb(36, 40, 52)
            };
            painter.line_segment(
                [pos2(start_x, cy), pos2(end_x, cy)],
                Stroke::new(2.0_f32, line_col),
            );
        }
    }
}

/// Draws a crisp anti-aliased vector checkmark centered at `center` without using any font glyph.
fn draw_vector_checkmark(painter: &egui::Painter, center: Pos2, thickness: f32, color: Color32) {
    let p1 = pos2(center.x - 3.2, center.y + 0.2);
    let p2 = pos2(center.x - 0.8, center.y + 2.6);
    let p3 = pos2(center.x + 3.6, center.y - 2.4);
    painter.line_segment([p1, p2], Stroke::new(thickness, color));
    painter.line_segment([p2, p3], Stroke::new(thickness, color));
}

/// Draws a 100% Full-Rounded Pill Toggle Switch (`Rounding = height / 2`) with a smooth circular knob.
fn draw_full_rounded_toggle(
    ui: &mut Ui,
    id_str: &str,
    value: &mut bool,
    label_reg: &str,
    label_bold: Option<&str>,
) {
    let desired = vec2(ui.available_width(), 20.0);
    let (rect, resp) = ui.allocate_exact_size(desired, Sense::click());
    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    if resp.clicked() {
        *value = !*value;
    }

    let painter = ui.painter();
    let track_w = 32.0;
    let track_h = 17.0;
    let track_rect = Rect::from_min_size(
        pos2(rect.min.x, rect.center().y - track_h * 0.5),
        vec2(track_w, track_h),
    );

    let t = ui
        .ctx()
        .animate_bool_with_time(ui.id().with((id_str, "pill_anim")), *value, 0.16);

    let track_bg = Color32::from_rgb(
        (36.0 + (37.0 - 36.0) * t) as u8,
        (40.0 + (99.0 - 40.0) * t) as u8,
        (50.0 + (235.0 - 50.0) * t) as u8,
    );
    painter.rect_filled(track_rect, Rounding::same(track_h * 0.5), track_bg);

    if !*value {
        painter.rect_stroke(
            track_rect,
            Rounding::same(track_h * 0.5),
            Stroke::new(1.0_f32, Color32::from_rgb(52, 58, 72)),
        );
    }

    let knob_r = 6.2;
    let knob_x = (track_rect.min.x + 8.5) + t * (track_w - 17.0);
    let knob_col = if *value {
        Color32::WHITE
    } else {
        Color32::from_rgb(148, 155, 170)
    };
    painter.circle_filled(pos2(knob_x, track_rect.center().y), knob_r, knob_col);

    let text_x = track_rect.max.x + 8.0;
    let text_y = rect.center().y;
    let galley = painter.layout_no_wrap(
        label_reg.to_string(),
        FontId::new(11.0, FontFamily::Proportional),
        Color32::from_rgb(216, 220, 230),
    );
    let reg_w = galley.size().x;
    painter.galley(
        pos2(text_x, text_y - galley.size().y * 0.5),
        galley,
        Color32::WHITE,
    );

    if let Some(bold_str) = label_bold {
        painter.text(
            pos2(text_x + reg_w, text_y),
            Align2::LEFT_CENTER,
            bold_str,
            FontId::new(11.0, FontFamily::Name("Bold".into())),
            Color32::from_rgb(248, 250, 252),
        );
    }
}

fn draw_window_icon_btn(ui: &mut Ui, is_close: bool) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(24.0, 24.0), Sense::click());
    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    let painter = ui.painter();
    if resp.hovered() {
        let bg = if is_close {
            Color32::from_rgb(185, 28, 28)
        } else {
            Color32::from_rgb(34, 38, 48)
        };
        painter.rect_filled(rect, Rounding::same(12.0), bg);
    }
    let fg = if resp.hovered() {
        Color32::WHITE
    } else {
        Color32::from_rgb(150, 156, 170)
    };
    let c = rect.center();
    if is_close {
        painter.line_segment(
            [pos2(c.x - 3.8, c.y - 3.8), pos2(c.x + 3.8, c.y + 3.8)],
            Stroke::new(1.3_f32, fg),
        );
        painter.line_segment(
            [pos2(c.x + 3.8, c.y - 3.8), pos2(c.x - 3.8, c.y + 3.8)],
            Stroke::new(1.3_f32, fg),
        );
    } else {
        painter.line_segment(
            [pos2(c.x - 4.2, c.y), pos2(c.x + 4.2, c.y)],
            Stroke::new(1.3_f32, fg),
        );
    }
    resp
}

/// Force-terminates all running `MyDiskly` / `mydiskly-app` processes using 4 layered mechanisms:
/// 1. Watchdog signal file (`mydiskly_force_quit.signal`)
/// 2. Window message (`WM_MYDISKLY_FORCE_QUIT` + `WM_CLOSE`) & Win32 `TerminateProcess` on window PID
/// 3. Silent `taskkill /F` (`CREATE_NO_WINDOW`)
/// 4. Elevated `taskkill` fallback if `MyDiskly.exe` is still locked by an elevated process
fn force_kill_running_mydiskly(install_dir: &Path) {
    // 1. Write watchdog signal file so MyDiskly removes its tray icon and exits immediately
    let sig_file = std::env::temp_dir().join("mydiskly_force_quit.signal");
    let _ = fs::write(&sig_file, b"1");

    // 2. Find MyDiskly window, post WM_MYDISKLY_FORCE_QUIT & WM_CLOSE, and TerminateProcess by PID
    let title_w = to_wide_chars("MyDiskly");
    let hwnd = unsafe { FindWindowW(std::ptr::null(), title_w.as_ptr()) };
    if hwnd != 0 {
        unsafe {
            PostMessageW(hwnd, WM_MYDISKLY_FORCE_QUIT, 0, 0);
            PostMessageW(hwnd, WM_CLOSE, 0, 0);
            let mut pid: u32 = 0;
            GetWindowThreadProcessId(hwnd, &mut pid);
            if pid != 0 {
                let h_proc = OpenProcess(PROCESS_TERMINATE, 0, pid);
                if h_proc != 0 {
                    let _ = TerminateProcess(h_proc, 0);
                    let _ = CloseHandle(h_proc);
                }
            }
        }
    }

    // 3. Run taskkill silently
    let _ = Command::new("taskkill")
        .args(["/F", "/IM", "MyDiskly.exe", "/IM", "mydiskly-app.exe", "/T"])
        .creation_flags(CREATE_NO_WINDOW)
        .status();

    thread::sleep(Duration::from_millis(250));

    // 4. Check if MyDiskly.exe is still locked (e.g. running as Admin while Uninstaller is User)
    let target_exe = install_dir.join("MyDiskly.exe");
    if target_exe.exists() {
        let still_locked = fs::OpenOptions::new()
            .write(true)
            .open(&target_exe)
            .is_err();
        if still_locked {
            let verb = to_wide_chars("runas");
            let file = to_wide_chars("taskkill.exe");
            let args = to_wide_chars("/F /IM MyDiskly.exe /IM mydiskly-app.exe /T");
            unsafe {
                ShellExecuteW(
                    0,
                    verb.as_ptr(),
                    file.as_ptr(),
                    args.as_ptr(),
                    std::ptr::null(),
                    SW_HIDE,
                );
            }
            thread::sleep(Duration::from_millis(450));
        }
    }

    let _ = fs::remove_file(&sig_file);
}

fn setup_installer_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.clear();
    fonts.families.clear();

    fonts.font_data.insert(
        "PlusJakartaSans-Regular".to_owned(),
        FontData::from_static(FONT_REGULAR),
    );
    fonts.font_data.insert(
        "PlusJakartaSans-Bold".to_owned(),
        FontData::from_static(FONT_BOLD),
    );

    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .push("PlusJakartaSans-Regular".to_owned());
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("PlusJakartaSans-Regular".to_owned());
    fonts
        .families
        .entry(FontFamily::Name("Bold".into()))
        .or_default()
        .push("PlusJakartaSans-Bold".to_owned());

    ctx.set_fonts(fonts);
}

fn setup_installer_visuals(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = Color32::from_rgb(13, 14, 17);
    visuals.window_fill = Color32::from_rgb(13, 14, 17);
    visuals.override_text_color = Some(Color32::from_rgb(244, 244, 246));
    ctx.set_visuals(visuals);
}

fn get_default_install_dir() -> PathBuf {
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        Path::new(&local).join("Programs").join("MyDiskly")
    } else {
        PathBuf::from(r"C:\Program Files\MyDiskly")
    }
}

fn browse_for_install_folder(hwnd: isize) -> Option<PathBuf> {
    let title = to_wide_chars("Select MyDiskly Installation Folder");
    let mut display_name = [0u16; 260];
    let bi = BrowseInfoW {
        hwnd_owner: hwnd,
        pidl_root: std::ptr::null_mut(),
        psz_display_name: display_name.as_mut_ptr(),
        lpsz_title: title.as_ptr(),
        ul_flags: 0x00000001 | 0x00000040,
        lpfn: std::ptr::null(),
        l_param: 0,
        i_image: 0,
    };

    unsafe {
        let pidl = SHBrowseForFolderW(&bi);
        if !pidl.is_null() {
            let mut path_buf = [0u16; 260];
            if SHGetPathFromIDListW(pidl, path_buf.as_mut_ptr()) != 0 {
                let len = path_buf
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(path_buf.len());
                let folder = String::from_utf16_lossy(&path_buf[..len]);
                let base = PathBuf::from(folder);
                if base
                    .file_name()
                    .map(|n| n.to_string_lossy().eq_ignore_ascii_case("MyDiskly"))
                    .unwrap_or(false)
                {
                    return Some(base);
                } else {
                    return Some(base.join("MyDiskly"));
                }
            }
        }
    }
    None
}

fn create_shortcut_silent(target_exe: &Path, shortcut_path: &Path) {
    let script = format!(
        "Set oWS = WScript.CreateObject(\"WScript.Shell\")\r\n\
        Set oLink = oWS.CreateShortcut(\"{}\")\r\n\
        oLink.TargetPath = \"{}\"\r\n\
        oLink.WorkingDirectory = \"{}\"\r\n\
        oLink.Description = \"MyDiskly Modern Storage Suite\"\r\n\
        oLink.IconLocation = \"{},0\"\r\n\
        oLink.Save\r\n",
        shortcut_path.to_string_lossy(),
        target_exe.to_string_lossy(),
        target_exe
            .parent()
            .unwrap_or(Path::new("C:\\"))
            .to_string_lossy(),
        target_exe.to_string_lossy()
    );

    let unique_name = format!(
        "mydiskly_sc_{}.vbs",
        shortcut_path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "lnk".to_string())
    );
    let temp_vbs = std::env::temp_dir().join(unique_name);
    if fs::write(&temp_vbs, script).is_ok() {
        let _ = Command::new("wscript.exe")
            .args(["//B", "//Nologo", &temp_vbs.to_string_lossy()])
            .creation_flags(CREATE_NO_WINDOW)
            .status();
        let _ = fs::remove_file(temp_vbs);
    }
}

fn open_shell_target(target: &str) {
    let verb = to_wide_chars("open");
    let file = to_wide_chars(target);
    unsafe {
        ShellExecuteW(
            0,
            verb.as_ptr(),
            file.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            SW_SHOWNORMAL,
        );
    }
}

fn register_uninstaller(install_dir: &Path) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok((key, _)) =
        hkcu.create_subkey(r"Software\Microsoft\Windows\CurrentVersion\Uninstall\MyDiskly")
    {
        let _ = key.set_value("DisplayName", &"MyDiskly");
        let _ = key.set_value("DisplayVersion", &"1.1.2026");
        let _ = key.set_value("Publisher", &"Seroja Labs");
        let _ = key.set_value("InstallLocation", &install_dir.to_string_lossy().as_ref());
        let exe_path = install_dir.join("MyDiskly.exe");
        let uninst_path = install_dir.join("Uninstall.exe");
        let _ = key.set_value("DisplayIcon", &exe_path.to_string_lossy().as_ref());
        let _ = key.set_value(
            "UninstallString",
            &format!("\"{}\" --uninstall", uninst_path.to_string_lossy()),
        );
        let _ = key.set_value(
            "QuietUninstallString",
            &format!("\"{}\" --uninstall", uninst_path.to_string_lossy()),
        );
    }
}

fn configure_native_window_corners(hwnd: isize) {
    if hwnd == 0 {
        return;
    }
    unsafe {
        let dark_mode: i32 = 1;
        let _ = DwmSetWindowAttribute(
            hwnd,
            20,
            &dark_mode as *const _ as *const c_void,
            std::mem::size_of::<i32>() as u32,
        );
        let round_pref: i32 = 2; // DWMWCP_ROUND
        let _ = DwmSetWindowAttribute(
            hwnd,
            33,
            &round_pref as *const _ as *const c_void,
            std::mem::size_of::<i32>() as u32,
        );
    }
}

fn calculate_centered_bounds(win_w: i32, win_h: i32) -> (i32, i32) {
    let mut rect = windows_sys::Win32::Foundation::RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    unsafe {
        SystemParametersInfoW(
            SPI_GETWORKAREA,
            0,
            &mut rect as *mut _ as *mut c_void,
            0,
        );
    }
    let screen_w = rect.right - rect.left;
    let screen_h = rect.bottom - rect.top;
    if screen_w > win_w && screen_h > win_h {
        (
            rect.left + (screen_w - win_w) / 2,
            rect.top + (screen_h - win_h) / 2,
        )
    } else {
        (240, 200)
    }
}

fn detect_uninstall_mode_and_dir() -> (bool, PathBuf) {
    let mut is_uninstall = std::env::args().any(|a| a.eq_ignore_ascii_case("--uninstall"));
    let mut detected_dir = get_default_install_dir();

    if let Ok(exe) = std::env::current_exe() {
        if let Some(stem) = exe.file_stem() {
            if stem
                .to_string_lossy()
                .to_lowercase()
                .contains("uninstall")
            {
                is_uninstall = true;
            }
        }
        if is_uninstall {
            if let Some(parent) = exe.parent() {
                // Only use parent if it's not target\release
                let p_str = parent.to_string_lossy().to_lowercase();
                if !p_str.ends_with(r"target\release") && !p_str.ends_with(r"target\debug") {
                    detected_dir = parent.to_path_buf();
                }
            }
        }
    }

    // Unlock the installation folder immediately by moving current working directory to %TEMP%
    let _ = std::env::set_current_dir(std::env::temp_dir());

    (is_uninstall, detected_dir)
}

fn main() -> Result<(), eframe::Error> {
    enforce_os_gatekeeping();

    let (is_uninstall_mode, detected_dir) = detect_uninstall_mode_and_dir();
    let win_w = 480;
    let win_h = 300;
    let (x, y) = calculate_centered_bounds(win_w, win_h);
    let icon_data = egui::IconData {
        rgba: ICON_RGBA_64.to_vec(),
        width: 64,
        height: 64,
    };

    let win_title = if is_uninstall_mode {
        "MyDiskly Uninstaller"
    } else {
        "MyDiskly Installer"
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(win_title)
            .with_app_id("MyDiskly.Installer")
            .with_icon(Arc::new(icon_data))
            .with_position([x as f32, y as f32])
            .with_inner_size([win_w as f32, win_h as f32])
            .with_min_inner_size([win_w as f32, win_h as f32])
            .with_max_inner_size([win_w as f32, win_h as f32])
            .with_decorations(false)
            .with_resizable(false)
            .with_transparent(false)
            .with_visible(true)
            .with_active(true),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_native(
        win_title,
        options,
        Box::new(move |cc| {
            Ok(Box::new(MyDisklyInstallerApp::new(
                cc,
                is_uninstall_mode,
                detected_dir,
            )))
        }),
    )
}
