use egui::{vec2, Align2, Color32, Frame, Id, Margin, Order, RichText, Rounding, Sense, Stroke, Ui};
use mydiskly_core::{CleanResult, CleanerCategory};
use crate::icons;

pub fn render(
    ui: &mut Ui,
    lang: &str,
    is_admin: bool,
    categories: &mut [CleanerCategory],
    is_cleaning: bool,
    cleaning_progress: f32,
    cleaning_item: &str,
    clean_result: Option<&CleanResult>,
    show_clean_popup: &mut bool,
    on_execute_clean: &mut bool,
    on_rescan: &mut bool,
    on_relaunch_admin: &mut bool,
) {
    // Enforce Read-Only unselection for uninstalled browsers OR admin-required items when not running as Admin
    for c in categories.iter_mut() {
        let can_use = c.is_installed && (!c.requires_admin || is_admin);
        if !can_use {
            c.selected = false;
        }
    }

    let total_junk_bytes: u64 = categories
        .iter()
        .filter(|c| c.is_installed && (!c.requires_admin || is_admin))
        .map(|c| c.size_bytes)
        .sum();
    let total_junk_mb = total_junk_bytes as f64 / (1024.0 * 1024.0);
    let total_junk_gb = total_junk_mb / 1024.0;
    let total_str = if total_junk_gb >= 1.0 {
        format!("{:.2} GB", total_junk_gb)
    } else {
        format!("{:.1} MB", total_junk_mb)
    };

    // Entire view becomes Read-Only while cleaning is in progress or popup is open
    let ui_interactive = !is_cleaning && !*show_clean_popup;

    ui.add_enabled_ui(ui_interactive, |ui| {
        // 1. HEADER SUMMARY CARD
        Frame::none()
            .fill(Color32::from_rgb(20, 20, 22))
            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(38, 38, 42)))
            .rounding(Rounding::same(10.0))
            .inner_margin(Margin::symmetric(14.0, 12.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        let title = if lang == "id" {
                            "TOTAL AKUMULASI CACHE & SAMPAH"
                        } else {
                            "TOTAL DETECTED CACHE & JUNK"
                        };
                        ui.colored_label(
                            Color32::from_rgb(150, 150, 160),
                            RichText::new(title).strong().size(10.5),
                        );
                        ui.add_space(2.0);
                        ui.colored_label(
                            Color32::from_rgb(250, 250, 250),
                            RichText::new(&total_str).strong().size(20.0),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let rescan_txt = if lang == "id" { "Pindai Ulang" } else { "Rescan" };
                        let btn = egui::Button::new(
                            RichText::new(rescan_txt)
                                .color(Color32::from_rgb(228, 228, 231))
                                .size(11.5)
                                .strong(),
                        )
                        .fill(Color32::from_rgb(32, 32, 36))
                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(52, 52, 58)))
                        .rounding(Rounding::same(6.0))
                        .min_size(vec2(88.0, 30.0));

                        if ui.add(btn).clicked() {
                            *on_rescan = true;
                        }
                    });
                });
            });

        ui.add_space(10.0);

        // Select All / Deselect All Bar (Only affects installed & permitted items)
        ui.horizontal(|ui| {
            let usable_cats: Vec<&CleanerCategory> = categories
                .iter()
                .filter(|c| c.is_installed && (!c.requires_admin || is_admin))
                .collect();
            let all_selected = !usable_cats.is_empty() && usable_cats.iter().all(|c| c.selected);

            let btn_text = if all_selected {
                if lang == "id" { "Batalkan Pilihan" } else { "Deselect All" }
            } else {
                if lang == "id" { "Pilih Semua Aktif" } else { "Select All Active" }
            };

            let toggle_btn = egui::Button::new(
                RichText::new(btn_text)
                    .color(Color32::from_rgb(212, 212, 216))
                    .size(11.0),
            )
            .fill(Color32::from_rgb(24, 24, 27))
            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(45, 45, 50)))
            .rounding(Rounding::same(5.0));

            if ui.add(toggle_btn).clicked() {
                let new_val = !all_selected;
                for c in categories.iter_mut() {
                    let can_use = c.is_installed && (!c.requires_admin || is_admin);
                    if can_use {
                        c.selected = new_val;
                    } else {
                        c.selected = false;
                    }
                }
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let selected_count = categories
                    .iter()
                    .filter(|c| c.is_installed && (!c.requires_admin || is_admin) && c.selected)
                    .count();
                let total_usable = categories
                    .iter()
                    .filter(|c| c.is_installed && (!c.requires_admin || is_admin))
                    .count();
                ui.colored_label(
                    Color32::from_rgb(130, 130, 140),
                    RichText::new(format!("{}/{} active selected", selected_count, total_usable))
                        .size(11.0),
                );
            });
        });

        ui.add_space(8.0);

        // =====================================================================
        // 2. SYSTEM JUNK SECTION
        // =====================================================================
        ui.horizontal(|ui| {
            icons::icon_trash(ui, 13.0, Color32::from_rgb(161, 161, 170));
            ui.add_space(4.0);
            let sys_title = if lang == "id" {
                "SAMPAH SISTEM WINDOWS"
            } else {
                "WINDOWS SYSTEM JUNK"
            };
            ui.colored_label(
                Color32::from_rgb(161, 161, 170),
                RichText::new(sys_title).strong().size(11.0),
            );
        });
        ui.add_space(4.0);

        for cat in categories.iter_mut().filter(|c| !c.is_browser) {
            render_cleaner_item(ui, lang, is_admin, cat);
            ui.add_space(5.0);
        }

        ui.add_space(10.0);

        // =====================================================================
        // 3. DEDICATED BROWSER CLEANER SECTION + SAFETY OVERVIEW
        // =====================================================================
        ui.horizontal(|ui| {
            icons::icon_browser(ui, 13.0, Color32::from_rgb(161, 161, 170));
            ui.add_space(4.0);
            let brw_title = if lang == "id" {
                "PEMBERSIH KHUSUS BROWSER (DETEKSI OTOMATIS)"
            } else {
                "DEDICATED BROWSER CLEANER (AUTO-DETECTED)"
            };
            ui.colored_label(
                Color32::from_rgb(161, 161, 170),
                RichText::new(brw_title).strong().size(11.0),
            );
        });
        ui.add_space(5.0);

        // Browser Safety Overview Box
        Frame::none()
            .fill(Color32::from_rgb(18, 22, 20))
            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(42, 65, 50)))
            .rounding(Rounding::same(8.0))
            .inner_margin(Margin::symmetric(12.0, 10.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    icons::icon_shield(ui, 14.0, Color32::from_rgb(161, 210, 175));
                    ui.add_space(4.0);
                    let info_hdr = if lang == "id" { "Informasi :" } else { "Information :" };
                    ui.colored_label(
                        Color32::from_rgb(230, 245, 235),
                        RichText::new(info_hdr).strong().size(11.5),
                    );
                });
                ui.add_space(3.0);
                let info_body = if lang == "id" {
                    "Pembersihan internet browser kamu 100% Aman dan tidak akan menghapus data history,cookie,akun. ini hanya cache sistem sesuai dengan browser yang kamu pakai."
                } else {
                    "Your internet browser cleaning is 100% Safe and will not delete history, cookie, or account data. This only cleans system cache according to the browser you use."
                };
                ui.add(
                    egui::Label::new(
                        RichText::new(info_body)
                            .color(Color32::from_rgb(175, 190, 180))
                            .size(11.0),
                    )
                    .wrap(),
                );
            });

        ui.add_space(6.0);

        for cat in categories.iter_mut().filter(|c| c.is_browser) {
            render_cleaner_item(ui, lang, is_admin, cat);
            ui.add_space(5.0);
        }

        ui.add_space(10.0);

        // =====================================================================
        // 4. LOADING PROCESS OR CLEAN ACTION BUTTON
        // =====================================================================
        if is_cleaning {
            Frame::none()
                .fill(Color32::from_rgb(22, 22, 25))
                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(63, 63, 70)))
                .rounding(Rounding::same(8.0))
                .inner_margin(Margin::symmetric(12.0, 10.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.add_space(6.0);
                        let status_lbl = if lang == "id" {
                            format!(
                                "Sedang membersihkan: {} ({:.0}%)...",
                                cleaning_item,
                                cleaning_progress * 100.0
                            )
                        } else {
                            format!(
                                "Cleaning in progress: {} ({:.0}%)...",
                                cleaning_item,
                                cleaning_progress * 100.0
                            )
                        };
                        ui.colored_label(
                            Color32::from_rgb(244, 244, 245),
                            RichText::new(status_lbl).strong().size(11.5),
                        );
                    });
                    ui.add_space(6.0);
                    let (bar_rect, _) =
                        ui.allocate_exact_size(vec2(ui.available_width(), 6.0), Sense::hover());
                    let painter = ui.painter();
                    painter.rect_filled(
                        bar_rect,
                        Rounding::same(3.0),
                        Color32::from_rgb(38, 38, 42),
                    );
                    let fill_w = (bar_rect.width() * cleaning_progress.clamp(0.05, 1.0)).max(6.0);
                    let fill_rect =
                        egui::Rect::from_min_size(bar_rect.min, vec2(fill_w, bar_rect.height()));
                    painter.rect_filled(
                        fill_rect,
                        Rounding::same(3.0),
                        Color32::from_rgb(244, 244, 245),
                    );
                });
        } else {
            let selected_count = categories
                .iter()
                .filter(|c| c.is_installed && (!c.requires_admin || is_admin) && c.selected)
                .count();
            let selected_bytes: u64 = categories
                .iter()
                .filter(|c| c.is_installed && (!c.requires_admin || is_admin) && c.selected)
                .map(|c| c.size_bytes)
                .sum();
            let selected_mb = selected_bytes as f64 / (1024.0 * 1024.0);
            let selected_str = if selected_mb >= 1024.0 {
                format!("{:.2} GB", selected_mb / 1024.0)
            } else {
                format!("{:.1} MB", selected_mb)
            };

            let clean_lbl = if lang == "id" {
                format!("Bersihkan Sekarang ({})", selected_str)
            } else {
                format!("Clean Selected Now ({})", selected_str)
            };

            let btn = egui::Button::new(
                RichText::new(&clean_lbl)
                    .color(Color32::from_rgb(16, 16, 18))
                    .strong()
                    .size(13.0),
            )
            .fill(Color32::from_rgb(244, 244, 245))
            .rounding(Rounding::same(8.0))
            .min_size(vec2(ui.available_width(), 38.0));

            if ui.add_enabled(selected_count > 0, btn).clicked() {
                *on_execute_clean = true;
            }
        }

        // Administrator Relaunch Button when running in User Mode
        if !is_admin {
            ui.add_space(6.0);
            let admin_btn_lbl = if lang == "id" {
                "Buka Ulang Sebagai Administrator (Aktifkan Cache Admin)"
            } else {
                "Relaunch as Administrator (Unlock Admin Caches)"
            };
            let admin_btn = egui::Button::new(
                RichText::new(admin_btn_lbl)
                    .color(Color32::from_rgb(212, 212, 216))
                    .size(11.0)
                    .strong(),
            )
            .fill(Color32::from_rgb(26, 26, 30))
            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(58, 58, 66)))
            .rounding(Rounding::same(6.0))
            .min_size(vec2(ui.available_width(), 32.0));

            if ui.add(admin_btn).clicked() {
                *on_relaunch_admin = true;
            }
        }
    });

    // =========================================================================
    // 5. SOFT-BLUR COMPLETION MODAL POPUP
    // =========================================================================
    if *show_clean_popup {
        if let Some(res) = clean_result {
            render_soft_blur_completion_popup(ui.ctx(), lang, res, show_clean_popup);
        }
    }
}

fn render_cleaner_item(ui: &mut Ui, lang: &str, is_admin: bool, cat: &mut CleanerCategory) {
    // Item is Read-Only if browser is not installed OR if item requires admin and app is in User mode
    let is_locked_by_admin = cat.requires_admin && !is_admin;
    let is_usable = cat.is_installed && !is_locked_by_admin;

    let item_id = Id::new(("clean_item", &cat.id));
    let hover_id = Id::new(("clean_hover", &cat.id));
    let prev_hovered = ui.ctx().data(|d| d.get_temp::<bool>(hover_id).unwrap_or(false));
    let hover_t = ui
        .ctx()
        .animate_bool_with_time(item_id, prev_hovered && is_usable, 0.15);

    let bg_color = if !is_usable {
        Color32::from_rgb(15, 15, 17) // Read-only muted background
    } else if hover_t > 0.01 {
        Color32::from_rgb(
            (20.0 + 7.0 * hover_t) as u8,
            (20.0 + 7.0 * hover_t) as u8,
            (22.0 + 8.0 * hover_t) as u8,
        )
    } else {
        Color32::from_rgb(20, 20, 22)
    };

    let border_color = if !is_usable {
        Color32::from_rgb(28, 28, 32)
    } else if cat.selected {
        Color32::from_rgb(82, 82, 91)
    } else {
        Color32::from_rgb(38, 38, 42)
    };

    let resp = Frame::none()
        .fill(bg_color)
        .stroke(Stroke::new(1.0_f32, border_color))
        .rounding(Rounding::same(8.0))
        .inner_margin(Margin::symmetric(12.0, 10.0))
        .show(ui, |ui| {
            // Read-Only enforcement when not installed OR when requires_admin in User mode
            ui.add_enabled_ui(is_usable, |ui| {
                ui.horizontal(|ui| {
                    if !is_usable {
                        cat.selected = false;
                    }
                    ui.checkbox(&mut cat.selected, "");

                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            let name = if lang == "id" { &cat.name_id } else { &cat.name_en };
                            let name_col = if is_usable {
                                Color32::from_rgb(244, 244, 245)
                            } else {
                                Color32::from_rgb(105, 105, 115)
                            };
                            ui.colored_label(name_col, RichText::new(name).strong().size(12.5));

                            if cat.requires_admin {
                                Frame::none()
                                    .fill(Color32::from_rgb(36, 32, 20))
                                    .rounding(Rounding::same(3.0))
                                    .inner_margin(Margin::symmetric(5.0, 1.0))
                                    .show(ui, |ui| {
                                        ui.colored_label(
                                            Color32::from_rgb(234, 179, 8),
                                            RichText::new("ADMIN").size(9.5).strong(),
                                        );
                                    });
                            }
                        });

                        let desc = if is_locked_by_admin {
                            if lang == "id" {
                                "Membutuhkan izin Run as Administrator (Read-Only)"
                            } else {
                                "Requires Run as Administrator privilege (Read-Only)"
                            }
                        } else if lang == "id" {
                            &cat.desc_id
                        } else {
                            &cat.desc_en
                        };
                        let desc_col = if is_usable {
                            Color32::from_rgb(130, 130, 140)
                        } else {
                            Color32::from_rgb(85, 85, 95)
                        };
                        ui.colored_label(desc_col, RichText::new(desc).size(10.8));
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if is_usable {
                            ui.colored_label(
                                Color32::from_rgb(212, 212, 216),
                                RichText::new(&cat.size_str).strong().size(12.0),
                            );
                        } else {
                            Frame::none()
                                .fill(Color32::from_rgb(24, 24, 27))
                                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(42, 42, 46)))
                                .rounding(Rounding::same(4.0))
                                .inner_margin(Margin::symmetric(6.0, 2.0))
                                .show(ui, |ui| {
                                    ui.colored_label(
                                        Color32::from_rgb(105, 105, 115),
                                        RichText::new("Read-Only").size(10.0),
                                    );
                                });
                        }
                    });
                });
            });
        });

    ui.ctx()
        .data_mut(|d| d.insert_temp(hover_id, resp.response.hovered()));
}

fn render_soft_blur_completion_popup(
    ctx: &egui::Context,
    lang: &str,
    res: &CleanResult,
    show_clean_popup: &mut bool,
) {
    let screen_rect = ctx.screen_rect();

    // 1. Multi-layer Soft-Blur Dimmed Backdrop Overlay
    egui::Area::new(Id::new("soft_blur_backdrop"))
        .order(Order::Foreground)
        .fixed_pos(screen_rect.min)
        .show(ctx, |ui| {
            let (_, _) = ui.allocate_exact_size(screen_rect.size(), Sense::click_and_drag());
            let painter = ui.painter();
            painter.rect_filled(
                screen_rect,
                Rounding::ZERO,
                Color32::from_rgba_unmultiplied(8, 8, 10, 215),
            );
            painter.rect_filled(
                screen_rect.shrink(4.0),
                Rounding::same(8.0),
                Color32::from_rgba_unmultiplied(18, 18, 22, 95),
            );
        });

    // 2. Centered Completion Dialog Box
    egui::Area::new(Id::new("clean_complete_modal"))
        .order(Order::Tooltip)
        .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
        .show(ctx, |ui| {
            Frame::none()
                .fill(Color32::from_rgb(22, 22, 25))
                .stroke(Stroke::new(1.2_f32, Color32::from_rgb(75, 75, 85)))
                .rounding(Rounding::same(12.0))
                .inner_margin(Margin::symmetric(18.0, 16.0))
                .show(ui, |ui| {
                    ui.set_width(340.0);

                    // Top Right Minimal Close (x) Button
                    ui.horizontal(|ui| {
                        icons::icon_shield(ui, 15.0, Color32::from_rgb(244, 244, 245));
                        ui.add_space(4.0);
                        let hdr = if lang == "id" {
                            "Pembersihan Selesai"
                        } else {
                            "Cleaning Complete"
                        };
                        ui.colored_label(
                            Color32::from_rgb(244, 244, 245),
                            RichText::new(hdr).strong().size(13.0),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if icons::minimal_close_button(ui, 24.0).clicked() {
                                *show_clean_popup = false;
                            }
                        });
                    });

                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(12.0);

                    let (main_msg, total_label) = if lang == "id" {
                        (
                            "MyDiskly telah berhasil membersihkan secara permanent dari komputer anda.",
                            format!("Total Pembersihan : {}", res.freed_str),
                        )
                    } else {
                        (
                            "MyDiskly has successfully permanent cleaned from your devices",
                            format!("Total cleaning : {}", res.freed_str),
                        )
                    };

                    ui.add(
                        egui::Label::new(
                            RichText::new(main_msg)
                                .color(Color32::from_rgb(228, 228, 231))
                                .size(12.5),
                        )
                        .wrap(),
                    );

                    ui.add_space(10.0);

                    Frame::none()
                        .fill(Color32::from_rgb(14, 14, 16))
                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(42, 42, 48)))
                        .rounding(Rounding::same(6.0))
                        .inner_margin(Margin::symmetric(12.0, 10.0))
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.colored_label(
                                Color32::from_rgb(255, 255, 255),
                                RichText::new(total_label).strong().size(13.5),
                            );
                        });

                    ui.add_space(16.0);

                    let done_btn = egui::Button::new(
                        RichText::new("Done")
                            .color(Color32::from_rgb(16, 16, 18))
                            .strong()
                            .size(12.5),
                    )
                    .fill(Color32::from_rgb(244, 244, 245))
                    .rounding(Rounding::same(6.0))
                    .min_size(vec2(ui.available_width(), 34.0));

                    if ui.add(done_btn).clicked() {
                        *show_clean_popup = false;
                    }
                });
        });
}
