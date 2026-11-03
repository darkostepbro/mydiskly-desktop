use egui::{pos2, vec2, Align2, Color32, FontId, Frame, Margin, RichText, Rounding, Sense, Stroke, Ui};
use mydiskly_core::{ini_manager, AppConfig, SystemSpecs};
use crate::{icons, sys_integration};

pub fn render(
    ui: &mut Ui,
    lang: &str,
    specs: &SystemSpecs,
    config: &mut AppConfig,
    sys_notif_enabled: bool,
    on_refresh_specs: &mut bool,
    on_save_config: &mut bool,
    on_test_sound: &mut bool,
    save_feedback: Option<&str>,
) {
    // =========================================================================
    // 1. RESPONSIVE HARDWARE SPECIFICATIONS TABLE (FULL-WIDTH 2-COLUMN GRID)
    // =========================================================================
    ui.horizontal(|ui| {
        icons::icon_settings(ui, 14.0, Color32::from_rgb(161, 161, 170));
        ui.add_space(4.0);
        let title = if lang == "id" {
            "TABEL SPESIFIKASI PERANGKAT"
        } else {
            "HARDWARE SPECIFICATIONS TABLE"
        };
        ui.colored_label(
            Color32::from_rgb(161, 161, 170),
            RichText::new(title).strong().size(11.5),
        );

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let refresh_txt = if lang == "id" { "Segarkan" } else { "Refresh" };
            let btn = egui::Button::new(
                RichText::new(refresh_txt)
                    .color(Color32::from_rgb(228, 228, 231))
                    .size(11.0)
                    .strong(),
            )
            .fill(Color32::from_rgb(28, 28, 32))
            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(52, 52, 58)))
            .rounding(Rounding::same(5.0))
            .min_size(vec2(72.0, 24.0));

            if ui.add(btn).clicked() {
                *on_refresh_specs = true;
            }
        });
    });

    ui.add_space(6.0);

    let rows: [(&str, &str); 15] = [
        (
            if lang == "id" { "Sistem Operasi" } else { "Operating System" },
            &specs.os,
        ),
        (
            if lang == "id" { "Arsitektur" } else { "Architecture" },
            &specs.arch,
        ),
        (
            if lang == "id" { "Prosesor (CPU)" } else { "Processor (CPU)" },
            &specs.cpu,
        ),
        (
            if lang == "id" { "Kartu Grafis (GPU)" } else { "Graphics (GPU)" },
            &specs.gpu,
        ),
        (
            if lang == "id" { "Memori (RAM)" } else { "Installed RAM" },
            &specs.ram,
        ),
        (
            if lang == "id" { "Motherboard" } else { "Motherboard" },
            &specs.mobo,
        ),
        (
            if lang == "id" { "Mode BIOS" } else { "BIOS Mode" },
            &specs.bios_mode,
        ),
        (
            if lang == "id" { "Media Simpan" } else { "Storage Drives" },
            &specs.storage,
        ),
        (
            if lang == "id" { "Firmware Disk" } else { "Drive Firmware" },
            &specs.drive_fw,
        ),
        (
            if lang == "id" { "Layar Monitor" } else { "Display Monitor" },
            &specs.monitor,
        ),
        (
            if lang == "id" { "Perangkat Audio" } else { "Audio Endpoint" },
            &specs.audio,
        ),
        (
            if lang == "id" { "Periferal Input" } else { "Peripherals" },
            &specs.peripherals,
        ),
        (
            if lang == "id" { "Jaringan Aktif" } else { "Active Network" },
            &specs.net_active,
        ),
        ("Wi-Fi Adapter", &specs.net_wifi),
        ("LAN Ethernet", &specs.net_lan),
    ];

    let full_table_w = ui.available_width();
    let col1_w = 126.0_f32;
    let col2_w = (full_table_w - 24.0 - col1_w - 20.0).max(160.0);

    Frame::none()
        .fill(Color32::from_rgb(18, 18, 20))
        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(42, 42, 48)))
        .rounding(Rounding::same(10.0))
        .inner_margin(Margin::ZERO)
        .show(ui, |ui| {
            ui.set_min_width(full_table_w);
            ui.set_max_width(full_table_w);

            // Table Header Row (Pixel-perfect column alignment with body rows)
            let hdr_resp = Frame::none()
                .fill(Color32::from_rgb(28, 28, 32))
                .rounding(Rounding {
                    nw: 10.0,
                    ne: 10.0,
                    sw: 0.0,
                    se: 0.0,
                })
                .inner_margin(Margin::symmetric(12.0, 8.5))
                .show(ui, |ui| {
                    ui.set_min_width(full_table_w - 24.0);
                    ui.horizontal_top(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;

                        ui.allocate_ui_with_layout(
                            vec2(col1_w, 16.0),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                ui.set_min_width(col1_w);
                                ui.set_max_width(col1_w);
                                let hdr_k = if lang == "id" { "KOMPONEN" } else { "COMPONENT" };
                                ui.colored_label(
                                    Color32::from_rgb(185, 185, 195),
                                    RichText::new(hdr_k).strong().size(10.5),
                                );
                            },
                        );

                        ui.add_space(20.0);

                        ui.allocate_ui_with_layout(
                            vec2(col2_w, 16.0),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                ui.set_min_width(col2_w);
                                ui.set_max_width(col2_w);
                                let hdr_v = if lang == "id" {
                                    "DETAIL SPESIFIKASI"
                                } else {
                                    "SPECIFICATION DETAILS"
                                };
                                ui.colored_label(
                                    Color32::from_rgb(185, 185, 195),
                                    RichText::new(hdr_v).strong().size(10.5),
                                );
                            },
                        );
                    });
                });

            // Draw vertical separator on header & bottom border
            {
                let r = hdr_resp.response.rect;
                let vx = r.min.x + 12.0 + col1_w + 10.0;
                ui.painter().line_segment(
                    [pos2(vx, r.min.y), pos2(vx, r.max.y)],
                    Stroke::new(1.0_f32, Color32::from_rgb(48, 48, 56)),
                );
                ui.painter().line_segment(
                    [pos2(r.min.x, r.max.y), pos2(r.max.x, r.max.y)],
                    Stroke::new(1.0_f32, Color32::from_rgb(48, 48, 56)),
                );
            }

            // Table Body Rows
            for (idx, (label, value)) in rows.iter().enumerate() {
                let is_last = idx + 1 == rows.len();
                let row_bg = if idx % 2 == 0 {
                    Color32::from_rgb(18, 18, 20)
                } else {
                    Color32::from_rgb(23, 23, 26)
                };

                let row_rounding = if is_last {
                    Rounding {
                        nw: 0.0,
                        ne: 0.0,
                        sw: 10.0,
                        se: 10.0,
                    }
                } else {
                    Rounding::ZERO
                };

                let row_resp = Frame::none()
                    .fill(row_bg)
                    .rounding(row_rounding)
                    .inner_margin(Margin::symmetric(12.0, 7.0))
                    .show(ui, |ui| {
                        ui.set_min_width(full_table_w - 24.0);
                        ui.horizontal_top(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;

                            // Column 1: Component Name
                            ui.allocate_ui_with_layout(
                                vec2(col1_w, 16.0),
                                egui::Layout::top_down(egui::Align::Min),
                                |ui| {
                                    ui.set_min_width(col1_w);
                                    ui.set_max_width(col1_w);
                                    ui.add(
                                        egui::Label::new(
                                            RichText::new(*label)
                                                .color(Color32::from_rgb(160, 160, 170))
                                                .strong()
                                                .size(11.0),
                                        )
                                        .wrap(),
                                    );
                                },
                            );

                            ui.add_space(20.0);

                            // Column 2: Specification Value (Full remaining width, wrapped)
                            ui.allocate_ui_with_layout(
                                vec2(col2_w, 16.0),
                                egui::Layout::top_down(egui::Align::Min),
                                |ui| {
                                    ui.set_min_width(col2_w);
                                    ui.set_max_width(col2_w);
                                    ui.add(
                                        egui::Label::new(
                                            RichText::new(*value)
                                                .color(Color32::from_rgb(244, 244, 246))
                                                .strong()
                                                .size(11.0),
                                        )
                                        .wrap(),
                                    );
                                },
                            );
                        });
                    });

                let r = row_resp.response.rect;
                let vx = r.min.x + 12.0 + col1_w + 10.0;
                // Vertical column divider
                ui.painter().line_segment(
                    [pos2(vx, r.min.y), pos2(vx, r.max.y)],
                    Stroke::new(1.0_f32, Color32::from_rgb(36, 36, 42)),
                );
                // Horizontal row divider
                if !is_last {
                    ui.painter().line_segment(
                        [pos2(r.min.x, r.max.y), pos2(r.max.x, r.max.y)],
                        Stroke::new(1.0_f32, Color32::from_rgb(32, 32, 38)),
                    );
                }
            }
        });

    ui.add_space(12.0);

    // =========================================================================
    // 2. SYSTEM STARTUP & TRAY INTEGRATION (1. Run on startup, 2. Always on tray)
    // =========================================================================
    ui.horizontal(|ui| {
        icons::icon_shield(ui, 14.0, Color32::from_rgb(161, 161, 170));
        ui.add_space(4.0);
        let sys_title = if lang == "id" {
            "INTEGRASI SISTEM & STARTUP"
        } else {
            "SYSTEM & STARTUP INTEGRATION"
        };
        ui.colored_label(
            Color32::from_rgb(161, 161, 170),
            RichText::new(sys_title).strong().size(11.5),
        );
    });
    ui.add_space(6.0);

    Frame::none()
        .fill(Color32::from_rgb(20, 20, 22))
        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(38, 38, 42)))
        .rounding(Rounding::same(10.0))
        .inner_margin(Margin::symmetric(14.0, 12.0))
        .show(ui, |ui| {
            // 1. Run on startup (Registry HKCU\...\Run)
            ui.horizontal(|ui| {
                let lbl1 = if lang == "id" {
                    "1. Run on startup (Daftar Otomatis di Registry)"
                } else {
                    "1. Run on startup (Register in Windows Registry)"
                };
                if ui
                    .checkbox(
                        &mut config.start_on_boot,
                        RichText::new(lbl1)
                            .color(Color32::from_rgb(235, 235, 240))
                            .strong()
                            .size(11.5),
                    )
                    .changed()
                {
                    let _ = ini_manager::sync_startup_registry(config.start_on_boot);
                    let _ = ini_manager::save_config(config);
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let reg_active = ini_manager::is_startup_registered();
                    let (badge_txt, badge_col) = if reg_active {
                        ("HKCU: ON", Color32::from_rgb(34, 197, 94))
                    } else {
                        ("HKCU: OFF", Color32::from_rgb(130, 130, 140))
                    };
                    Frame::none()
                        .fill(Color32::from_rgb(28, 28, 32))
                        .rounding(Rounding::same(4.0))
                        .inner_margin(Margin::symmetric(6.0, 2.0))
                        .show(ui, |ui| {
                            ui.colored_label(badge_col, RichText::new(badge_txt).strong().size(9.5));
                        });
                });
            });

            ui.add_space(6.0);

            // 2. Always on tray (Shell_NotifyIconW + Minimize to Tray on Close)
            ui.horizontal(|ui| {
                let lbl2 = if lang == "id" {
                    "2. Always on tray (Tetap di System Tray saat ditutup)"
                } else {
                    "2. Always on tray (Keep in System Tray on close)"
                };
                if ui
                    .checkbox(
                        &mut config.run_in_tray,
                        RichText::new(lbl2)
                            .color(Color32::from_rgb(235, 235, 240))
                            .strong()
                            .size(11.5),
                    )
                    .changed()
                {
                    let _ = ini_manager::save_config(config);
                }
            });
        });

    ui.add_space(12.0);

    // =========================================================================
    // 3. CACHE NOTIFICATION SETTINGS & AUTO-DETECTION OVERLAY
    // =========================================================================
    ui.horizontal(|ui| {
        icons::icon_bell(ui, 14.0, Color32::from_rgb(161, 161, 170));
        ui.add_space(4.0);
        let notif_title = if lang == "id" {
            "NOTIFIKASI PENGINGAT PEMBERSIHAN"
        } else {
            "CACHE CLEANING NOTIFICATION"
        };
        ui.colored_label(
            Color32::from_rgb(161, 161, 170),
            RichText::new(notif_title).strong().size(11.5),
        );
    });
    ui.add_space(6.0);

    // Enforce MB max 999 rule: if user sets > 999 in MB mode, auto-switch to GB and convert size!
    if config.notify_custom_unit.eq_ignore_ascii_case("MB") && config.notify_custom_value > 999 {
        let gb_val = (config.notify_custom_value / 1000).max(1);
        config.notify_custom_value = gb_val;
        config.notify_custom_unit = "GB".to_string();
    }

    // Card background becomes transparent dark when system notifications are disabled
    let notif_card_bg = if sys_notif_enabled {
        Color32::from_rgb(20, 20, 22)
    } else {
        Color32::from_rgba_unmultiplied(14, 14, 16, 110)
    };
    let notif_card_border = if sys_notif_enabled {
        Color32::from_rgb(38, 38, 42)
    } else {
        Color32::from_rgba_unmultiplied(36, 36, 40, 120)
    };

    let notif_card_resp = Frame::none()
        .fill(notif_card_bg)
        .stroke(Stroke::new(1.0_f32, notif_card_border))
        .rounding(Rounding::same(10.0))
        .inner_margin(Margin::symmetric(14.0, 12.0))
        .show(ui, |ui| {
            ui.add_enabled_ui(sys_notif_enabled, |ui| {
                let remind_hdr = if lang == "id" {
                    "Remind me to clean (Ingatkan jika cache melampaui) :"
                } else {
                    "Remind me to clean :"
                };
                ui.colored_label(
                    Color32::from_rgb(240, 240, 245),
                    RichText::new(remind_hdr).strong().size(12.0),
                );
                ui.add_space(8.0);

                // Preset Radio Options: 500mb, 800mb, 1GB, 2GB
                let presets = [
                    ("500mb", "500mb"),
                    ("800mb", "800mb"),
                    ("1gb", "1GB"),
                    ("2gb", "2GB"),
                ];

                ui.horizontal_wrapped(|ui| {
                    for (val, label) in presets {
                        let is_sel = config.notify_mode.eq_ignore_ascii_case(val);
                        if ui
                            .radio(
                                is_sel,
                                RichText::new(label)
                                    .color(Color32::from_rgb(228, 228, 231))
                                    .strong()
                                    .size(12.0),
                            )
                            .clicked()
                        {
                            config.notify_mode = val.to_string();
                        }
                        ui.add_space(10.0);
                    }
                });

                ui.add_space(4.0);

                // Custom [MB/GB] Row (Max 999 MB -> auto converts to 1+ GB)
                ui.horizontal(|ui| {
                    let is_custom = config.notify_mode.eq_ignore_ascii_case("custom");
                    if ui
                        .radio(
                            is_custom,
                            RichText::new("Custom")
                                .color(Color32::from_rgb(228, 228, 231))
                                .strong()
                                .size(12.0),
                        )
                        .clicked()
                    {
                        config.notify_mode = "custom".to_string();
                    }

                    ui.add_space(4.0);

                    let drag = egui::DragValue::new(&mut config.notify_custom_value)
                        .range(1..=9999)
                        .speed(5.0);
                    if ui.add_sized([64.0, 22.0], drag).changed() {
                        config.notify_mode = "custom".to_string();
                        if config.notify_custom_unit.eq_ignore_ascii_case("MB")
                            && config.notify_custom_value > 999
                        {
                            let gb_val = (config.notify_custom_value / 1000).max(1);
                            config.notify_custom_value = gb_val;
                            config.notify_custom_unit = "GB".to_string();
                        } else if config.notify_custom_unit.eq_ignore_ascii_case("GB")
                            && config.notify_custom_value > 999
                        {
                            config.notify_custom_value = 999;
                        }
                    }

                    // [MB / GB] Unit Toggle Pill
                    for unit in ["MB", "GB"] {
                        let active_unit = config.notify_custom_unit.eq_ignore_ascii_case(unit);
                        let u_btn = egui::Button::new(
                            RichText::new(unit)
                                .color(if active_unit {
                                    Color32::from_rgb(16, 16, 18)
                                } else {
                                    Color32::from_rgb(180, 180, 190)
                                })
                                .strong()
                                .size(10.5),
                        )
                        .fill(if active_unit {
                            Color32::from_rgb(244, 244, 245)
                        } else {
                            Color32::from_rgb(30, 30, 34)
                        })
                        .rounding(Rounding::same(4.0))
                        .min_size(vec2(32.0, 22.0));

                        if ui.add(u_btn).clicked() {
                            config.notify_custom_unit = unit.to_string();
                            config.notify_mode = "custom".to_string();
                            if unit == "MB" && config.notify_custom_value > 999 {
                                config.notify_custom_value = 999;
                            }
                        }
                    }
                });

                ui.add_space(4.0);

                // Never Option
                ui.horizontal(|ui| {
                    let is_never = config.notify_mode.eq_ignore_ascii_case("never");
                    if ui
                        .radio(
                            is_never,
                            RichText::new("Never")
                                .color(Color32::from_rgb(228, 228, 231))
                                .strong()
                                .size(12.0),
                        )
                        .clicked()
                    {
                        config.notify_mode = "never".to_string();
                    }
                });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(10.0);

                // Action Buttons: [Save] and [Test sound]
                ui.horizontal(|ui| {
                    let btn_w = (ui.available_width() - 8.0) * 0.5;

                    let save_btn = egui::Button::new(
                        RichText::new("Save")
                            .color(Color32::from_rgb(16, 16, 18))
                            .strong()
                            .size(12.5),
                    )
                    .fill(Color32::from_rgb(244, 244, 245))
                    .rounding(Rounding::same(6.0))
                    .min_size(vec2(btn_w, 34.0));

                    if ui.add(save_btn).clicked() {
                        *on_save_config = true;
                    }

                    ui.add_space(6.0);

                    let test_btn = egui::Button::new(
                        RichText::new("Test sound")
                            .color(Color32::from_rgb(244, 244, 245))
                            .strong()
                            .size(12.5),
                    )
                    .fill(Color32::from_rgb(32, 32, 38))
                    .stroke(Stroke::new(1.0_f32, Color32::from_rgb(68, 68, 76)))
                    .rounding(Rounding::same(6.0))
                    .min_size(vec2(btn_w, 34.0));

                    if ui.add(test_btn).clicked() {
                        *on_test_sound = true;
                    }
                });

                if let Some(msg) = save_feedback {
                    ui.add_space(8.0);
                    ui.colored_label(
                        Color32::from_rgb(34, 197, 94),
                        RichText::new(msg).strong().size(11.5),
                    );
                }
            });
        });

    // If system notification is disabled on device, draw transparent dark overlay + "Enable notification on system"
    if !sys_notif_enabled {
        let card_rect = notif_card_resp.response.rect;
        let overlay_resp = ui.interact(
            card_rect,
            egui::Id::new("disabled_sys_notif_overlay"),
            Sense::click(),
        );
        let painter = ui.painter();

        // Transparent dark backdrop over card
        painter.rect_filled(
            card_rect,
            Rounding::same(10.0),
            Color32::from_rgba_unmultiplied(10, 10, 12, 188),
        );

        // Centered message pill
        let pill_w = 240.0;
        let pill_h = 38.0;
        let pill_rect = egui::Rect::from_center_size(card_rect.center(), vec2(pill_w, pill_h));
        painter.rect_filled(
            pill_rect,
            Rounding::same(8.0),
            Color32::from_rgba_unmultiplied(24, 24, 28, 240),
        );
        painter.rect_stroke(
            pill_rect,
            Rounding::same(8.0),
            Stroke::new(1.0_f32, Color32::from_rgb(72, 72, 82)),
        );
        painter.text(
            pill_rect.center(),
            Align2::CENTER_CENTER,
            "Enable notification on system",
            FontId::proportional(12.5),
            Color32::from_rgb(244, 244, 245),
        );

        if overlay_resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
        if overlay_resp.clicked() {
            sys_integration::open_windows_notification_settings();
        }
    }
}
