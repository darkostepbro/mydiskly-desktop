#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod icons;
mod sys_integration;
mod views;

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use egui::{vec2, Color32, Frame, Id, Margin, RichText, Rounding, Stroke, TextureHandle};
use mydiskly_core::*;
use windows_sys::Win32::UI::WindowsAndMessaging::{SystemParametersInfoW, SPI_GETWORKAREA};

const FONT_REGULAR: &[u8] = include_bytes!("../assets/fonts/PlusJakartaSans-Regular.ttf");
const FONT_BOLD: &[u8] = include_bytes!("../assets/fonts/PlusJakartaSans-Bold.ttf");

fn log_trace(msg: &str) {
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(r"C:\Users\DARKO\.gemini\antigravity\scratch\mydiskly-rust\app_trace.log")
    {
        let _ = writeln!(f, "[TRACE] {}", msg);
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum AppTab {
    Dashboard,
    Cleaner,
    Specs,
    About,
}

struct CleaningState {
    is_cleaning: bool,
    progress: f32,
    current_item: String,
    completed_result: Option<CleanResult>,
}

struct MyDisklyApp {
    active_tab: AppTab,
    tab_switch_time: Instant,
    lang: String, // "id" or "en"
    is_admin: bool,
    partitions: Vec<PartitionInfo>,
    expanded_partition: Option<String>,
    smart_disks: Vec<SmartDiskInfo>,
    io_snapshot: IoRateSnapshot,
    cleaner_categories: Arc<Mutex<Vec<CleanerCategory>>>,
    is_scanning_cleaner: Arc<Mutex<bool>>,
    cleaning_state: Arc<Mutex<CleaningState>>,
    clean_result: Option<CleanResult>,
    show_clean_popup: bool,
    specs: SystemSpecs,
    config: AppConfig,
    tray_manager: sys_integration::TrayManager,
    sys_notif_enabled: bool,
    has_notified_cache_high: bool,
    hotplug_banner: Option<String>,
    hotplug_timer: Option<Instant>,
    last_poll: Instant,
    save_feedback: Option<(String, Instant)>,
    dock_bounds: (i32, i32, i32, i32),
    hwnd_enforced_count: u32,
    logo_texture: TextureHandle,
}

impl MyDisklyApp {
    fn new(cc: &eframe::CreationContext<'_>, dock_bounds: (i32, i32, i32, i32)) -> Self {
        log_trace("MyDisklyApp::new started");
        sys_integration::register_egui_context(&cc.egui_ctx);
        setup_custom_fonts(&cc.egui_ctx);
        setup_custom_visuals(&cc.egui_ctx);

        let logo_image =
            egui::ColorImage::from_rgba_unmultiplied([64, 64], sys_integration::ICON_RGBA_64);
        let logo_texture = cc.egui_ctx.load_texture(
            "mydiskly_blue_logo",
            logo_image,
            egui::TextureOptions::LINEAR,
        );

        let config = ini_manager::load_config();
        let lang = config.language.clone();
        let is_admin = cleaner_engine::is_running_as_admin();

        log_trace("Loading partitions & telemetry snapshot");
        let partitions = disk_telemetry::get_partitions();
        let smart_disks = smart_diagnostics::get_smart_disks();
        let io_snapshot = disk_telemetry::get_telemetry_snapshot();
        let specs = hardware_specs::audit_system_specs(false);
        let _ = ini_manager::sync_myspecs_ini(&specs);
        let _ = ini_manager::sync_history_disk_health_ini(&partitions, &smart_disks);
        let sys_notif_enabled = sys_integration::is_system_notification_enabled();

        let cleaner_categories = Arc::new(Mutex::new(Vec::new()));
        let is_scanning = Arc::new(Mutex::new(true));

        let cats_clone = Arc::clone(&cleaner_categories);
        let scan_clone = Arc::clone(&is_scanning);
        std::thread::spawn(move || {
            log_trace("Background cleaner scan thread running");
            let scanned = cleaner_engine::scan_cleaner_categories();
            if let Ok(mut lock) = cats_clone.lock() {
                *lock = scanned;
            }
            if let Ok(mut lock) = scan_clone.lock() {
                *lock = false;
            }
            sys_integration::trim_working_set();
            log_trace("Background cleaner scan completed");
        });

        let cleaning_state = Arc::new(Mutex::new(CleaningState {
            is_cleaning: false,
            progress: 0.0,
            current_item: String::new(),
            completed_result: None,
        }));

        log_trace("MyDisklyApp::new finished successfully");
        Self {
            active_tab: AppTab::Dashboard,
            tab_switch_time: Instant::now(),
            lang,
            is_admin,
            partitions,
            expanded_partition: None,
            smart_disks,
            io_snapshot,
            cleaner_categories,
            is_scanning_cleaner: is_scanning,
            cleaning_state,
            clean_result: None,
            show_clean_popup: false,
            specs,
            config,
            tray_manager: sys_integration::TrayManager::new(),
            sys_notif_enabled,
            has_notified_cache_high: false,
            hotplug_banner: None,
            hotplug_timer: None,
            last_poll: Instant::now(),
            save_feedback: None,
            dock_bounds,
            hwnd_enforced_count: 0,
            logo_texture,
        }
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    // Clear redundant built-in fonts to reduce RAM and GPU glyph texture atlas footprint
    fonts.font_data.clear();
    fonts.families.clear();

    fonts.font_data.insert(
        "PlusJakartaSans-Regular".to_owned(),
        egui::FontData::from_static(FONT_REGULAR),
    );
    fonts.font_data.insert(
        "PlusJakartaSans-Bold".to_owned(),
        egui::FontData::from_static(FONT_BOLD),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .push("PlusJakartaSans-Regular".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("PlusJakartaSans-Regular".to_owned());

    ctx.set_fonts(fonts);
}

fn setup_custom_visuals(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = Color32::from_rgba_unmultiplied(12, 14, 18, 224);
    visuals.window_fill = Color32::from_rgba_unmultiplied(12, 14, 18, 224);
    visuals.override_text_color = Some(Color32::from_rgb(244, 244, 245));
    visuals.widgets.noninteractive.bg_fill = Color32::from_rgba_unmultiplied(20, 22, 26, 232);
    visuals.widgets.inactive.bg_fill = Color32::from_rgba_unmultiplied(26, 28, 34, 235);
    visuals.widgets.hovered.bg_fill = Color32::from_rgba_unmultiplied(36, 40, 48, 242);
    visuals.widgets.active.bg_fill = Color32::from_rgba_unmultiplied(48, 52, 62, 248);
    ctx.set_visuals(visuals);
}

impl eframe::App for MyDisklyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.047, 0.055, 0.071, 0.88]
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        static ONCE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        if !ONCE.swap(true, std::sync::atomic::Ordering::SeqCst) {
            log_trace(">>> FIRST UPDATE() CALLED - WINDOW IS ACTIVELY RENDERING! <<<");
        }

        // If user clicked the System Tray icon, re-enforce right-docked topmost bounds immediately
        if sys_integration::take_tray_restore_request() {
            self.hwnd_enforced_count = 0;
        }

        // Enforce right-docked vertical position, subclass WndProc for Tray, & Always-On-Top on startup/restore frames
        if self.hwnd_enforced_count < 5 {
            let (x, y, w, h) = self.dock_bounds;
            let hwnd = sys_integration::enforce_right_dock_topmost(
                x,
                y,
                w,
                h,
                self.hwnd_enforced_count == 0,
            );
            if hwnd != 0 {
                if self.hwnd_enforced_count == 0 {
                    log_trace(&format!(
                        "HWND found and docked TOPMOST at x={}, y={}, w={}, h={} (hwnd={})",
                        x, y, w, h, hwnd
                    ));
                }
                self.tray_manager.sync_tray(self.config.run_in_tray, hwnd);
                self.hwnd_enforced_count += 1;
                if self.hwnd_enforced_count == 5 {
                    sys_integration::trim_working_set();
                }
            }
        } else {
            self.tray_manager.sync_tray(self.config.run_in_tray, 0);
        }

        // Check if background cleaning finished
        let (is_cleaning, cleaning_progress, cleaning_item) = {
            let mut st = self
                .cleaning_state
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            if let Some(done_res) = st.completed_result.take() {
                self.clean_result = Some(done_res);
                self.show_clean_popup = true;
                sys_integration::play_interface_sound();
            }
            (st.is_cleaning, st.progress, st.current_item.clone())
        };

        // Check automatic cache notification threshold once initial scan finishes
        let is_scanning = *self
            .is_scanning_cleaner
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if !is_scanning && !self.has_notified_cache_high && self.sys_notif_enabled {
            if let Some(threshold_bytes) = self.config.get_notify_threshold_bytes() {
                if let Ok(cats) = self.cleaner_categories.lock() {
                    let total_bytes: u64 = cats
                        .iter()
                        .filter(|c| c.is_installed && (!c.requires_admin || self.is_admin))
                        .map(|c| c.size_bytes)
                        .sum();
                    if total_bytes >= threshold_bytes {
                        let mb = total_bytes as f64 / (1024.0 * 1024.0);
                        let size_str = if mb >= 1024.0 {
                            format!("{:.2} GB", mb / 1024.0)
                        } else {
                            format!("{:.1} MB", mb)
                        };
                        self.tray_manager
                            .trigger_cache_notification(&self.lang, &size_str);
                        self.has_notified_cache_high = true;
                    }
                }
            }
        }

        // Telemetry, Hotplug & System Notification status polling (1 second interval)
        let poll_interval = Duration::from_millis(1000);
        if self.last_poll.elapsed() >= poll_interval {
            self.last_poll = Instant::now();
            self.io_snapshot = disk_telemetry::get_telemetry_snapshot();
            self.partitions = disk_telemetry::get_partitions();
            self.sys_notif_enabled = sys_integration::is_system_notification_enabled();

            match hotplug_monitor::check_hotplug_change() {
                hotplug_monitor::HotplugEvent::DriveConnected(d) => {
                    self.hotplug_banner = Some(format!("Drive Connected: {}", d));
                    self.hotplug_timer = Some(Instant::now());
                    self.smart_disks = smart_diagnostics::get_smart_disks();
                    self.specs = hardware_specs::audit_system_specs(true);
                    let _ = ini_manager::sync_myspecs_ini(&self.specs);
                    let _ = ini_manager::sync_history_disk_health_ini(
                        &self.partitions,
                        &self.smart_disks,
                    );
                }
                hotplug_monitor::HotplugEvent::DriveDisconnected(d) => {
                    self.hotplug_banner = Some(format!("Drive Disconnected: {}", d));
                    self.hotplug_timer = Some(Instant::now());
                    self.smart_disks = smart_diagnostics::get_smart_disks();
                    self.specs = hardware_specs::audit_system_specs(true);
                    let _ = ini_manager::sync_myspecs_ini(&self.specs);
                    let _ = ini_manager::sync_history_disk_health_ini(
                        &self.partitions,
                        &self.smart_disks,
                    );
                }
                hotplug_monitor::HotplugEvent::None => {}
            }
        }

        if let Some(t) = self.hotplug_timer {
            if t.elapsed() > Duration::from_secs(6) {
                self.hotplug_banner = None;
                self.hotplug_timer = None;
            }
        }

        if let Some((_, t)) = &self.save_feedback {
            if t.elapsed() > Duration::from_secs(4) {
                self.save_feedback = None;
            }
        }

        // Fast repaint during animations or cleaning, low-power 500ms repaint when idle
        let anim_elapsed = self.tab_switch_time.elapsed().as_secs_f32();
        if is_cleaning || anim_elapsed < 0.25 {
            ctx.request_repaint_after(Duration::from_millis(16));
        } else {
            ctx.request_repaint_after(Duration::from_millis(500));
        }

        // =====================================================================
        // DOCKED BOTTOM TELEMETRY PANEL (Only on Dashboard: Case Photo 1 & 2)
        // =====================================================================
        if self.active_tab == AppTab::Dashboard {
            egui::TopBottomPanel::bottom("dashboard_io_dock")
                .frame(
                    Frame::none()
                        .fill(Color32::from_rgba_unmultiplied(12, 14, 18, 224))
                        .inner_margin(Margin {
                            left: 14.0,
                            right: 14.0,
                            top: 6.0,
                            bottom: 12.0,
                        }),
                )
                .show_separator_line(false)
                .show(ctx, |ui| {
                    views::dashboard::render_docked_telemetry(ui, &self.lang, &self.io_snapshot);
                });
        }

        // =====================================================================
        // MAIN CENTRAL CONTAINER
        // =====================================================================
        egui::CentralPanel::default()
            .frame(
                Frame::none()
                    .fill(Color32::from_rgba_unmultiplied(12, 14, 18, 224))
                    .inner_margin(Margin {
                        left: 14.0,
                        right: 14.0,
                        top: 12.0,
                        bottom: 8.0,
                    }),
            )
            .show(ctx, |ui| {
                // =============================================================
                // 1. MINIMALIST HEADER (NO ICON ON NAVBAR, [User/Admin mode], [ID/EN], [—], [x])
                // =============================================================
                ui.horizontal(|ui| {
                    // Only App Name + Version Badge
                    ui.colored_label(
                        Color32::from_rgb(250, 250, 252),
                        RichText::new("MYDISKLY").strong().size(15.5),
                    );
                    Frame::none()
                        .fill(Color32::from_rgb(24, 24, 28))
                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(45, 45, 52)))
                        .rounding(Rounding::same(4.0))
                        .inner_margin(Margin::symmetric(5.0, 1.5))
                        .show(ui, |ui| {
                            ui.colored_label(
                                Color32::from_rgb(161, 161, 170),
                                RichText::new("v1.1").size(10.0).strong(),
                            );
                        });

                    // Right Controls: Minimal Close (x), Minimal Minimize (—), Language [ID | EN], and [User mode] / [Admin mode]
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let exit_resp = icons::minimal_close_button(ui, 26.0);
                        if exit_resp.clicked() {
                            if self.config.run_in_tray {
                                let hwnd = self.tray_manager.hwnd();
                                self.tray_manager.sync_tray(true, hwnd);
                                sys_integration::hide_window_to_tray(hwnd);
                            } else {
                                self.tray_manager.remove_icon();
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        }

                        ui.add_space(4.0);

                        let min_resp = icons::minimal_minimize_button(ui, 26.0);
                        if min_resp.clicked() {
                            let hwnd = self.tray_manager.hwnd();
                            if hwnd != 0 {
                                sys_integration::minimize_window(hwnd);
                            } else {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                            }
                        }

                        ui.add_space(6.0);

                        // Language Toggle Pill [ID | EN]
                        ui.add_enabled_ui(!is_cleaning && !self.show_clean_popup, |ui| {
                            let lang_lbl = if self.lang == "id" { "ID" } else { "EN" };
                            let lang_btn = egui::Button::new(
                                RichText::new(lang_lbl)
                                    .color(Color32::from_rgb(212, 212, 216))
                                    .strong()
                                    .size(11.0),
                            )
                            .fill(Color32::from_rgb(24, 24, 28))
                            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(48, 48, 55)))
                            .rounding(Rounding::same(6.0))
                            .min_size(vec2(34.0, 26.0));

                            if ui
                                .add(lang_btn)
                                .on_hover_text("Switch Language [ID / EN]")
                                .clicked()
                            {
                                self.lang = if self.lang == "id" {
                                    "en".to_string()
                                } else {
                                    "id".to_string()
                                };
                                self.config.language = self.lang.clone();
                                let _ = ini_manager::save_config(&self.config);
                            }
                        });

                        ui.add_space(6.0);

                        // [User mode] (transparent text) or [Admin mode] (bright text) beside Language button
                        let (mode_txt, mode_fg, mode_bg, mode_border) = if self.is_admin {
                            (
                                "[Admin mode]",
                                Color32::from_rgb(255, 255, 255),
                                Color32::from_rgb(28, 32, 38),
                                Color32::from_rgb(72, 82, 96),
                            )
                        } else {
                            (
                                "[User mode]",
                                Color32::from_rgba_unmultiplied(180, 180, 190, 110),
                                Color32::from_rgba_unmultiplied(20, 20, 24, 90),
                                Color32::from_rgba_unmultiplied(42, 42, 48, 100),
                            )
                        };

                        Frame::none()
                            .fill(mode_bg)
                            .stroke(Stroke::new(1.0_f32, mode_border))
                            .rounding(Rounding::same(6.0))
                            .inner_margin(Margin::symmetric(8.0, 4.5))
                            .show(ui, |ui| {
                                ui.colored_label(
                                    mode_fg,
                                    RichText::new(mode_txt).strong().size(10.5),
                                );
                            });
                    });
                });

                ui.add_space(10.0);

                // =============================================================
                // 2. MONOCHROME NAVIGATION TABS (Dashboard, Cleaner, Settings, About)
                // =============================================================
                ui.add_enabled_ui(!is_cleaning && !self.show_clean_popup, |ui| {
                    Frame::none()
                        .fill(Color32::from_rgb(20, 20, 23))
                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(36, 36, 42)))
                        .rounding(Rounding::same(8.0))
                        .inner_margin(Margin::same(3.0))
                        .show(ui, |ui| {
                            let avail_w = ui.available_width();
                            let tab_w = ((avail_w - 18.0) / 4.0).max(70.0);

                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 4.0;
                                let tabs = [
                                    (
                                        AppTab::Dashboard,
                                        if self.lang == "id" { "Dasbor" } else { "Dashboard" },
                                    ),
                                    (
                                        AppTab::Cleaner,
                                        if self.lang == "id" { "Pembersih" } else { "Cleaner" },
                                    ),
                                    (
                                        AppTab::Specs,
                                        if self.lang == "id" { "Pengaturan" } else { "Settings" },
                                    ),
                                    (
                                        AppTab::About,
                                        if self.lang == "id" { "Tentang" } else { "About" },
                                    ),
                                ];

                                for (tab, label) in tabs {
                                    let is_active = self.active_tab == tab;
                                    let anim_id = Id::new(("nav_tab_anim", tab));
                                    let t = ctx.animate_bool_with_time(anim_id, is_active, 0.18);

                                    let bg_color = Color32::from_rgb(
                                        (20.0 + 22.0 * t) as u8,
                                        (20.0 + 22.0 * t) as u8,
                                        (23.0 + 25.0 * t) as u8,
                                    );
                                    let text_color = Color32::from_rgb(
                                        (145.0 + 105.0 * t) as u8,
                                        (145.0 + 105.0 * t) as u8,
                                        (155.0 + 95.0 * t) as u8,
                                    );

                                    let btn = egui::Button::new(
                                        RichText::new(label).color(text_color).strong().size(11.5),
                                    )
                                    .fill(bg_color)
                                    .stroke(if is_active {
                                        Stroke::new(1.0_f32, Color32::from_rgb(68, 68, 78))
                                    } else {
                                        Stroke::NONE
                                    })
                                    .rounding(Rounding::same(6.0))
                                    .min_size(vec2(tab_w, 29.0));

                                    if ui.add(btn).clicked() && self.active_tab != tab {
                                        self.active_tab = tab;
                                        self.tab_switch_time = Instant::now();
                                    }
                                }
                            });
                        });
                });

                ui.add_space(10.0);

                // =============================================================
                // 3. SMOOTH PAGE TRANSITION & SCROLLABLE VIEWPORT
                // =============================================================
                let page_t = (self.tab_switch_time.elapsed().as_secs_f32() / 0.18).clamp(0.0, 1.0);
                let slide_offset = (1.0 - page_t) * 5.0;
                if slide_offset > 0.1 {
                    ui.add_space(slide_offset);
                }
                ui.set_opacity(0.35 + 0.65 * page_t);

                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .hscroll(false)
                    .show(ui, |ui| {
                        match self.active_tab {
                            AppTab::Dashboard => {
                                views::dashboard::render(
                                    ui,
                                    &self.lang,
                                    &self.partitions,
                                    &self.smart_disks,
                                    self.hotplug_banner.as_deref(),
                                    &mut self.expanded_partition,
                                );
                            }
                            AppTab::Cleaner => {
                                if is_scanning {
                                    ui.add_space(32.0);
                                    ui.vertical_centered(|ui| {
                                        ui.spinner();
                                        ui.add_space(10.0);
                                        ui.colored_label(
                                            Color32::from_rgb(180, 180, 190),
                                            if self.lang == "id" {
                                                "Sedang memindai cache sistem & browser..."
                                            } else {
                                                "Scanning system & browser cache..."
                                            },
                                        );
                                    });
                                } else {
                                    let mut on_clean = false;
                                    let mut on_rescan = false;
                                    let mut on_admin = false;

                                    if let Ok(mut cats) = self.cleaner_categories.lock() {
                                        views::cleaner::render(
                                            ui,
                                            &self.lang,
                                            self.is_admin,
                                            &mut cats,
                                            is_cleaning,
                                            cleaning_progress,
                                            &cleaning_item,
                                            self.clean_result.as_ref(),
                                            &mut self.show_clean_popup,
                                            &mut on_clean,
                                            &mut on_rescan,
                                            &mut on_admin,
                                        );

                                        if on_clean && !is_cleaning {
                                            let selected_ids: Vec<String> = cats
                                                .iter()
                                                .filter(|c| {
                                                    c.is_installed
                                                        && (!c.requires_admin || self.is_admin)
                                                        && c.selected
                                                })
                                                .map(|c| c.id.clone())
                                                .collect();

                                            if !selected_ids.is_empty() {
                                                let state_arc = Arc::clone(&self.cleaning_state);
                                                let cats_arc = Arc::clone(&self.cleaner_categories);
                                                if let Ok(mut st) = state_arc.lock() {
                                                    st.is_cleaning = true;
                                                    st.progress = 0.05;
                                                    st.current_item = "Initializing...".to_string();
                                                    st.completed_result = None;
                                                }

                                                std::thread::spawn(move || {
                                                    let state_for_cb = Arc::clone(&state_arc);
                                                    let res =
                                                        cleaner_engine::execute_clean_with_progress(
                                                            &selected_ids,
                                                            |prog, label| {
                                                                if let Ok(mut st) =
                                                                    state_for_cb.lock()
                                                                {
                                                                    st.progress = prog;
                                                                    st.current_item =
                                                                        label.to_string();
                                                                }
                                                            },
                                                        );

                                                    let refreshed =
                                                        cleaner_engine::scan_cleaner_categories();
                                                    if let Ok(mut c_lock) = cats_arc.lock() {
                                                        *c_lock = refreshed;
                                                    }
                                                    if let Ok(mut st) = state_arc.lock() {
                                                        st.progress = 1.0;
                                                        st.is_cleaning = false;
                                                        st.completed_result = Some(res);
                                                    }
                                                    sys_integration::trim_working_set();
                                                });
                                            }
                                        }

                                        if on_rescan && !is_cleaning {
                                            *cats = cleaner_engine::scan_cleaner_categories();
                                            sys_integration::trim_working_set();
                                        }

                                        if on_admin {
                                            let _ = cleaner_engine::relaunch_as_admin();
                                        }
                                    }
                                }
                            }
                            AppTab::Specs => {
                                let mut on_refresh = false;
                                let mut on_save = false;
                                let mut on_test_sound = false;

                                let feedback_str =
                                    self.save_feedback.as_ref().map(|(s, _)| s.as_str());

                                views::specs::render(
                                    ui,
                                    &self.lang,
                                    &self.specs,
                                    &mut self.config,
                                    self.sys_notif_enabled,
                                    &mut on_refresh,
                                    &mut on_save,
                                    &mut on_test_sound,
                                    feedback_str,
                                );

                                if on_refresh {
                                    self.specs = hardware_specs::audit_system_specs(true);
                                    let _ = ini_manager::sync_myspecs_ini(&self.specs);
                                    let _ = ini_manager::sync_history_disk_health_ini(
                                        &self.partitions,
                                        &self.smart_disks,
                                    );
                                }

                                if on_save {
                                    if ini_manager::save_config(&self.config) {
                                        self.tray_manager
                                            .sync_tray(self.config.run_in_tray, 0);
                                        let msg = if self.lang == "id" {
                                            "Pengaturan & Registry berhasil disimpan!".to_string()
                                        } else {
                                            "Settings & Registry saved successfully!".to_string()
                                        };
                                        self.save_feedback = Some((msg, Instant::now()));
                                    }
                                }

                                if on_test_sound {
                                    self.tray_manager.trigger_cache_notification(
                                        &self.lang,
                                        "640.5 MB (Test)",
                                    );
                                }
                            }
                            AppTab::About => {
                                views::about::render(
                                    ui,
                                    &self.lang,
                                    &self.config,
                                    &self.logo_texture,
                                );
                            }
                        }
                    });
            });
    }
}

fn calculate_right_docked_bounds() -> (i32, i32, i32, i32) {
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
            &mut rect as *mut _ as *mut std::ffi::c_void,
            0,
        );
    }

    let screen_w = rect.right - rect.left;
    let screen_h = rect.bottom - rect.top;
    let win_w = 420;

    if screen_w > 500 && screen_h > 400 {
        (rect.right - win_w, rect.top, win_w, screen_h)
    } else {
        (100, 50, 420, 820)
    }
}

fn main() -> Result<(), eframe::Error> {
    log_trace("=== MyDiskly Native App main() started ===");

    // 1. Register explicit AppUserModelID & DisplayName ("MyDiskly") for Windows 11 Notification Center
    sys_integration::register_app_identity();

    // 2. Strict OS Gatekeeping: Only Windows 10 & Windows 11
    let _profile = enforce_os_gatekeeping();

    // 3. Single-Instance Mutex Enforcement
    let _guard = match try_acquire_single_instance() {
        Some(g) => g,
        None => {
            log_trace("Single-instance mutex already held by running process. Exiting cleanly.");
            std::process::exit(0);
        }
    };

    // 4. Screen bounds calculation (Vertically Right-Docked)
    let (x, y, w, h) = calculate_right_docked_bounds();
    log_trace(&format!(
        "Right-docked bounds: x={}, y={}, w={}, h={}",
        x, y, w, h
    ));

    // 5. Native Frameless Right-Docked Always-On-Top Viewport Options with Blue Square App Icon
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("MyDiskly")
            .with_app_id("MyDiskly")
            .with_icon(Arc::new(sys_integration::get_egui_icon_data()))
            .with_position([x as f32, y as f32])
            .with_inner_size([w as f32, h as f32])
            .with_decorations(false) // Frameless docking window
            .with_always_on_top()    // Always on Top
            .with_resizable(false)
            .with_transparent(true)
            .with_visible(true)
            .with_active(true),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_native(
        "MyDiskly",
        options,
        Box::new(move |cc| Ok(Box::new(MyDisklyApp::new(cc, (x, y, w, h))))),
    )
}
