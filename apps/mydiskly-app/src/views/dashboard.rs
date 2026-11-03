use egui::{pos2, vec2, Color32, Frame, Id, Margin, Pos2, Rect, RichText, Rounding, Sense, Stroke, Ui};
use mydiskly_core::{disk_telemetry, IoRateSnapshot, PartitionInfo, SmartDiskInfo};
use crate::icons;

pub fn render(
    ui: &mut Ui,
    lang: &str,
    partitions: &[PartitionInfo],
    smart_disks: &[SmartDiskInfo],
    hotplug_banner: Option<&str>,
    expanded_partition: &mut Option<String>,
) {
    // Hotplug notification banner if any USB drive plugged/unplugged
    if let Some(banner) = hotplug_banner {
        Frame::none()
            .fill(Color32::from_rgb(24, 24, 27))
            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(82, 82, 91)))
            .rounding(Rounding::same(8.0))
            .inner_margin(Margin::symmetric(12.0, 10.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    icons::icon_hard_drive(ui, 14.0, Color32::from_rgb(244, 244, 245));
                    ui.add_space(4.0);
                    ui.colored_label(Color32::from_rgb(244, 244, 245), RichText::new(banner).strong().size(12.0));
                });
            });
        ui.add_space(10.0);
    }

    // =========================================================================
    // 1. STORAGE PARTITIONS SECTION (Taller Cards + Interactive Dropdown)
    // =========================================================================
    ui.horizontal(|ui| {
        icons::icon_hard_drive(ui, 14.0, Color32::from_rgb(161, 161, 170));
        ui.add_space(4.0);
        let title = if lang == "id" { "PARTISI PENYIMPANAN" } else { "STORAGE PARTITIONS" };
        ui.colored_label(
            Color32::from_rgb(161, 161, 170),
            RichText::new(title).strong().size(11.5),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let hint = if lang == "id" { "Klik partisi untuk opsi" } else { "Click partition for menu" };
            ui.colored_label(Color32::from_rgb(113, 113, 122), RichText::new(hint).size(10.5));
        });
    });
    ui.add_space(6.0);

    for p in partitions {
        render_partition_card(ui, lang, p, expanded_partition);
        ui.add_space(8.0);
    }

    ui.add_space(10.0);

    // =========================================================================
    // 2. S.M.A.R.T DRIVE HEALTH SECTION (Taller Cards + Bold % and Temp°)
    // =========================================================================
    ui.horizontal(|ui| {
        icons::icon_activity(ui, 14.0, Color32::from_rgb(161, 161, 170));
        ui.add_space(4.0);
        let title = if lang == "id" { "KESEHATAN DRIVE (S.M.A.R.T)" } else { "DRIVE HEALTH (S.M.A.R.T)" };
        ui.colored_label(
            Color32::from_rgb(161, 161, 170),
            RichText::new(title).strong().size(11.5),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.colored_label(
                Color32::from_rgb(113, 113, 122),
                RichText::new(format!("{} Physical Units", smart_disks.len())).size(10.5),
            );
        });
    });
    ui.add_space(6.0);

    for disk in smart_disks {
        render_smart_card(ui, lang, disk);
        ui.add_space(8.0);
    }
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).round() as u8
}

fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    Color32::from_rgb(
        lerp_u8(a.r(), b.r(), t),
        lerp_u8(a.g(), b.g(), t),
        lerp_u8(a.b(), b.b(), t),
    )
}

fn render_partition_card(
    ui: &mut Ui,
    lang: &str,
    p: &PartitionInfo,
    expanded_partition: &mut Option<String>,
) {
    let is_expanded = expanded_partition.as_deref() == Some(p.drive.as_str());
    let card_id = Id::new(("part_card", &p.drive));
    let hover_id = Id::new(("part_hover", &p.drive));

    let prev_hovered = ui.ctx().data(|d| d.get_temp::<bool>(hover_id).unwrap_or(false));
    let hover_t = ui.ctx().animate_bool_with_time(card_id, prev_hovered || is_expanded, 0.16);

    let bg_color = lerp_color(Color32::from_rgb(20, 20, 22), Color32::from_rgb(28, 28, 32), hover_t);
    let border_color = lerp_color(Color32::from_rgb(38, 38, 42), Color32::from_rgb(72, 72, 80), hover_t);

    let frame_resp = Frame::none()
        .fill(bg_color)
        .stroke(Stroke::new(1.0_f32, border_color))
        .rounding(Rounding::same(10.0))
        .inner_margin(Margin::symmetric(14.0, 14.0)) // Taller card height
        .show(ui, |ui| {
            // Top Row: Drive Letter + Label + FileSystem + Dropdown Chevron
            let header_resp = ui
                .horizontal(|ui| {
                    icons::icon_hard_drive(ui, 15.0, Color32::from_rgb(244, 244, 245));
                    ui.add_space(4.0);
                    ui.colored_label(
                        Color32::from_rgb(250, 250, 250),
                        RichText::new(&p.drive).strong().size(14.0),
                    );
                    if !p.label.is_empty() {
                        ui.colored_label(
                            Color32::from_rgb(161, 161, 170),
                            RichText::new(format!("({})", p.label)).size(12.5),
                        );
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        icons::icon_chevron(ui, 13.0, Color32::from_rgb(161, 161, 170), is_expanded);
                        ui.add_space(4.0);
                        Frame::none()
                            .fill(Color32::from_rgb(32, 32, 36))
                            .rounding(Rounding::same(4.0))
                            .inner_margin(Margin::symmetric(6.0, 2.0))
                            .show(ui, |ui| {
                                ui.colored_label(
                                    Color32::from_rgb(180, 180, 188),
                                    RichText::new(&p.fstype).size(10.5).strong(),
                                );
                            });
                        if p.is_removable {
                            ui.colored_label(
                                Color32::from_rgb(212, 212, 216),
                                RichText::new("USB").size(10.5).strong(),
                            );
                        }
                    });
                })
                .response;

            ui.add_space(10.0);

            // Taller Progress Bar (8px height)
            let (bar_rect, bar_resp) =
                ui.allocate_exact_size(vec2(ui.available_width(), 8.0), Sense::click());
            let painter = ui.painter();

            painter.rect_filled(bar_rect, Rounding::same(4.0), Color32::from_rgb(34, 34, 38));

            let ratio = (p.percent_used / 100.0).clamp(0.0, 1.0);
            let fill_width = (bar_rect.width() * ratio).max(4.0);
            let fill_rect = Rect::from_min_size(bar_rect.min, vec2(fill_width, bar_rect.height()));

            let fill_color = match p.color_status.as_str() {
                "critical" | "danger" => Color32::from_rgb(239, 68, 68),
                "warning" => Color32::from_rgb(234, 179, 8),
                _ => Color32::from_rgb(228, 228, 231), // Clean high-contrast monochrome bar
            };

            painter.rect_filled(fill_rect, Rounding::same(4.0), fill_color);

            ui.add_space(10.0);

            // Bottom Metrics Row
            let metrics_resp = ui
                .horizontal(|ui| {
                    let used_lbl = if lang == "id" { "Terpakai" } else { "Used" };
                    let free_lbl = if lang == "id" { "Tersedia" } else { "Free" };
                    ui.colored_label(
                        Color32::from_rgb(212, 212, 216),
                        RichText::new(format!("{:.1} GB {} ({:.0}%)", p.free_gb, free_lbl, p.percent_free)).size(12.0),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.colored_label(
                            Color32::from_rgb(140, 140, 150),
                            RichText::new(format!("{:.1} / {:.1} GB {}", p.used_gb, p.total_gb, used_lbl)).size(11.5),
                        );
                    });
                })
                .response;

            // Check click on header/bar/metrics to toggle interactive dropdown
            let clicked_card = header_resp.interact(Sense::click()).clicked()
                || bar_resp.clicked()
                || metrics_resp.interact(Sense::click()).clicked();

            if clicked_card {
                if is_expanded {
                    *expanded_partition = None;
                } else {
                    *expanded_partition = Some(p.drive.clone());
                }
            }

            // Interactive Dropdown Menu
            if is_expanded {
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);

                ui.horizontal(|ui| {
                    let btn_w = (ui.available_width() - 8.0) * 0.5;

                    // Option 1: 1. Open file
                    let open_lbl = if lang == "id" { "1. Buka Berkas" } else { "1. Open file" };
                    let open_btn = egui::Button::new(
                        RichText::new(open_lbl)
                            .color(Color32::from_rgb(244, 244, 245))
                            .strong()
                            .size(11.5),
                    )
                    .fill(Color32::from_rgb(36, 36, 42))
                    .stroke(Stroke::new(1.0_f32, Color32::from_rgb(63, 63, 70)))
                    .rounding(Rounding::same(6.0))
                    .min_size(vec2(btn_w, 30.0));

                    if ui.add(open_btn).clicked() {
                        disk_telemetry::open_partition_explorer(&p.drive);
                    }

                    ui.add_space(6.0);

                    // Option 2: 2. Properties
                    let prop_lbl = if lang == "id" { "2. Properti" } else { "2. Properties" };
                    let prop_btn = egui::Button::new(
                        RichText::new(prop_lbl)
                            .color(Color32::from_rgb(244, 244, 245))
                            .strong()
                            .size(11.5),
                    )
                    .fill(Color32::from_rgb(36, 36, 42))
                    .stroke(Stroke::new(1.0_f32, Color32::from_rgb(63, 63, 70)))
                    .rounding(Rounding::same(6.0))
                    .min_size(vec2(btn_w, 30.0));

                    if ui.add(prop_btn).clicked() {
                        disk_telemetry::open_partition_properties(&p.drive);
                    }
                });
            }
        });

    let is_hovered = frame_resp.response.hovered();
    ui.ctx().data_mut(|d| d.insert_temp(hover_id, is_hovered));
    if is_hovered {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
}

fn render_smart_card(ui: &mut Ui, lang: &str, disk: &SmartDiskInfo) {
    let card_id = Id::new(("smart_card", &disk.id));
    let hover_id = Id::new(("smart_hover", &disk.id));
    let prev_hovered = ui.ctx().data(|d| d.get_temp::<bool>(hover_id).unwrap_or(false));
    let hover_t = ui.ctx().animate_bool_with_time(card_id, prev_hovered, 0.16);

    let bg_color = lerp_color(Color32::from_rgb(20, 20, 22), Color32::from_rgb(27, 27, 31), hover_t);
    let border_color = lerp_color(Color32::from_rgb(38, 38, 42), Color32::from_rgb(68, 68, 76), hover_t);

    let resp = Frame::none()
        .fill(bg_color)
        .stroke(Stroke::new(1.0_f32, border_color))
        .rounding(Rounding::same(10.0))
        .inner_margin(Margin::symmetric(14.0, 14.0)) // Taller card height
        .show(ui, |ui| {
            // Row 1: Disk Model + Health Badge
            ui.horizontal(|ui| {
                icons::icon_hard_drive(ui, 15.0, Color32::from_rgb(228, 228, 231));
                ui.add_space(4.0);
                ui.colored_label(
                    Color32::from_rgb(250, 250, 250),
                    RichText::new(&disk.name).strong().size(13.5),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (status_color, status_text) = if disk.health_status.to_lowercase().contains("healthy")
                        || disk.health_status.to_lowercase().contains("baik")
                    {
                        (Color32::from_rgb(34, 197, 94), if lang == "id" { "OPTIMAL" } else { "HEALTHY" })
                    } else {
                        (Color32::from_rgb(234, 179, 8), if lang == "id" { "PERHATIAN" } else { "WARNING" })
                    };

                    Frame::none()
                        .fill(Color32::from_rgb(26, 26, 30))
                        .stroke(Stroke::new(1.0_f32, status_color))
                        .rounding(Rounding::same(4.0))
                        .inner_margin(Margin::symmetric(7.0, 2.5))
                        .show(ui, |ui| {
                            ui.colored_label(status_color, RichText::new(status_text).strong().size(10.5));
                        });
                });
            });

            ui.add_space(6.0);

            // Row 2: Media Type & Capacity
            ui.horizontal(|ui| {
                let badge = format!("{}  •  {}", disk.media_type, disk.bus_type);
                ui.colored_label(Color32::from_rgb(145, 145, 155), RichText::new(badge).size(11.5));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.colored_label(
                        Color32::from_rgb(200, 200, 210),
                        RichText::new(format!("{:.1} GB", disk.size_gb)).strong().size(12.0),
                    );
                });
            });

            ui.add_space(10.0);

            // Row 3: BOLD Health % and BOLD Temperature (°C) + Power Hours
            Frame::none()
                .fill(Color32::from_rgb(15, 15, 17))
                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(32, 32, 36)))
                .rounding(Rounding::same(6.0))
                .inner_margin(Margin::symmetric(10.0, 8.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let wear_lbl = if lang == "id" { "Kesehatan:" } else { "Health:" };
                        ui.colored_label(Color32::from_rgb(150, 150, 160), RichText::new(wear_lbl).size(11.5));
                        // BOLD Percentage (e.g. 98%, 100%)
                        ui.colored_label(
                            Color32::from_rgb(255, 255, 255),
                            RichText::new(&disk.wear_life).strong().size(13.5),
                        );

                        ui.add_space(12.0);

                        let temp_lbl = if lang == "id" { "Suhu:" } else { "Temp:" };
                        ui.colored_label(Color32::from_rgb(150, 150, 160), RichText::new(temp_lbl).size(11.5));
                        // BOLD Temperature (e.g. 34°C, 50°)
                        ui.colored_label(
                            Color32::from_rgb(255, 255, 255),
                            RichText::new(&disk.temperature).strong().size(13.5),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.colored_label(
                                Color32::from_rgb(150, 150, 160),
                                RichText::new(&disk.power_hours).size(11.5),
                            );
                        });
                    });
                });
        });

    ui.ctx().data_mut(|d| d.insert_temp(hover_id, resp.response.hovered()));
}

pub fn render_docked_telemetry(ui: &mut Ui, lang: &str, io: &IoRateSnapshot) {
    Frame::none()
        .fill(Color32::from_rgb(20, 20, 22))
        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(38, 38, 42)))
        .rounding(Rounding::same(10.0))
        .inner_margin(Margin::symmetric(14.0, 12.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                icons::icon_activity(ui, 14.0, Color32::from_rgb(180, 180, 190));
                ui.add_space(4.0);
                let lbl = if lang == "id" { "AKTIVITAS READ / WRITE & GRAFIK" } else { "READ / WRITE & I/O GRAPH" };
                ui.colored_label(
                    Color32::from_rgb(161, 161, 170),
                    RichText::new(lbl).strong().size(11.0),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.colored_label(
                        Color32::from_rgb(56, 189, 248),
                        RichText::new(format!("W: {:.1} MB/s", io.write_mb)).strong().size(11.5),
                    );
                    ui.add_space(8.0);
                    ui.colored_label(
                        Color32::from_rgb(34, 197, 94),
                        RichText::new(format!("R: {:.1} MB/s", io.read_mb)).strong().size(11.5),
                    );
                });
            });

            ui.add_space(8.0);

            // Graph Canvas
            let height = 74.0;
            let (rect, _response) = ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
            let painter = ui.painter();

            // Background & subtle grid lines
            painter.rect_filled(rect, Rounding::same(6.0), Color32::from_rgb(13, 13, 15));
            let grid_color = Color32::from_rgb(26, 26, 30);
            for frac in [0.25_f32, 0.50_f32, 0.75_f32] {
                let gy = rect.min.y + height * frac;
                painter.line_segment(
                    [pos2(rect.min.x + 2.0, gy), pos2(rect.max.x - 2.0, gy)],
                    Stroke::new(1.0_f32, grid_color),
                );
            }

            let history = &io.history;
            if history.len() >= 2 {
                let max_val = history
                    .iter()
                    .map(|p| p.read_mb.max(p.write_mb))
                    .fold(1.0f64, |a, b| a.max(b))
                    .max(5.0) as f32;

                let dx = rect.width() / (history.len() - 1) as f32;

                let mut read_points = Vec::with_capacity(history.len());
                let mut write_points = Vec::with_capacity(history.len());

                for (i, pt) in history.iter().enumerate() {
                    let x = rect.min.x + i as f32 * dx;
                    let ry = rect.max.y - (pt.read_mb as f32 / max_val * (height - 10.0)) - 5.0;
                    let wy = rect.max.y - (pt.write_mb as f32 / max_val * (height - 10.0)) - 5.0;

                    read_points.push(Pos2::new(x, ry.clamp(rect.min.y + 3.0, rect.max.y - 3.0)));
                    write_points.push(Pos2::new(x, wy.clamp(rect.min.y + 3.0, rect.max.y - 3.0)));
                }

                for w in write_points.windows(2) {
                    painter.line_segment([w[0], w[1]], Stroke::new(1.6_f32, Color32::from_rgb(56, 189, 248)));
                }
                for r in read_points.windows(2) {
                    painter.line_segment([r[0], r[1]], Stroke::new(1.6_f32, Color32::from_rgb(34, 197, 94)));
                }

                if let Some(&last_r) = read_points.last() {
                    painter.circle_filled(last_r, 3.2, Color32::from_rgb(34, 197, 94));
                }
                if let Some(&last_w) = write_points.last() {
                    painter.circle_filled(last_w, 3.2, Color32::from_rgb(56, 189, 248));
                }
            }

            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.colored_label(
                    Color32::from_rgb(130, 130, 140),
                    RichText::new(format!("Peak Read: {:.1} MB/s", io.peak_read_mb)).size(11.0),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.colored_label(
                        Color32::from_rgb(130, 130, 140),
                        RichText::new(format!("Peak Write: {:.1} MB/s", io.peak_write_mb)).size(11.0),
                    );
                });
            });
        });
}
