use egui::{vec2, Color32, Frame, Margin, RichText, Rounding, Sense, Stroke, TextureHandle, Ui};
use mydiskly_core::gatekeeper::to_wide_chars;
use mydiskly_core::AppConfig;
use windows_sys::Win32::UI::Shell::ShellExecuteW;

fn open_url(url: &str) {
    let verb = to_wide_chars("open");
    let target = to_wide_chars(url);
    unsafe {
        ShellExecuteW(
            0,
            verb.as_ptr(),
            target.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            1,
        );
    }
}

pub fn render(ui: &mut Ui, lang: &str, config: &AppConfig, logo_texture: &TextureHandle) {
    ui.add_space(6.0);

    // 1. Main Identity Card featuring the Original Blue Square MyDiskly Icon
    Frame::none()
        .fill(Color32::from_rgb(20, 20, 22))
        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(38, 38, 44)))
        .rounding(Rounding::same(12.0))
        .inner_margin(Margin::symmetric(18.0, 22.0))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());

            ui.vertical_centered(|ui| {
                // Blue Square MyDiskly Icon (58x58)
                ui.add(egui::Image::new((logo_texture.id(), vec2(58.0, 58.0))));

                ui.add_space(12.0);

                // MyDiskly [version]
                ui.colored_label(
                    Color32::from_rgb(250, 250, 252),
                    RichText::new(format!("MyDiskly v{}", config.version))
                        .strong()
                        .size(18.0),
                );

                ui.add_space(6.0);

                // Bio Text (No box around it, per user instruction)
                ui.colored_label(
                    Color32::from_rgb(185, 185, 195),
                    RichText::new("A Super Lightweight Smart Disk Scanner & Cleaner in one app.")
                        .size(11.5),
                );

                ui.add_space(12.0);

                // Publisher : Seroja Labs
                ui.colored_label(
                    Color32::from_rgb(212, 212, 218),
                    RichText::new("Publisher : Seroja Labs").strong().size(12.5),
                );

                ui.add_space(4.0);

                // GPL License. Check on GitHub
                ui.colored_label(
                    Color32::from_rgb(145, 145, 155),
                    RichText::new("GPL License. Check on GitHub").size(11.5),
                );

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(14.0);

                // Hyperlink Row: [ Official Website | Run Source Code ]
                ui.horizontal(|ui| {
                    let total_w = ui.available_width();
                    let btn_w = ((total_w - 14.0) * 0.5).max(120.0);

                    // Official Website Hyperlink Button
                    let web_btn = egui::Button::new(
                        RichText::new("Official Website")
                            .color(Color32::from_rgb(56, 189, 248))
                            .underline()
                            .strong()
                            .size(12.0),
                    )
                    .fill(Color32::from_rgb(24, 28, 34))
                    .stroke(Stroke::new(1.0_f32, Color32::from_rgb(40, 68, 92)))
                    .rounding(Rounding::same(6.0))
                    .min_size(vec2(btn_w, 32.0));

                    let web_resp = ui
                        .add(web_btn)
                        .on_hover_text("https://serojalabs.sbs/mydiskly");
                    if web_resp.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }
                    if web_resp.clicked() {
                        open_url("https://serojalabs.sbs/mydiskly");
                    }

                    ui.colored_label(
                        Color32::from_rgb(90, 90, 100),
                        RichText::new("|").strong().size(13.0),
                    );

                    // Run Source Code Hyperlink Button
                    let repo_btn = egui::Button::new(
                        RichText::new("Run Source Code")
                            .color(Color32::from_rgb(56, 189, 248))
                            .underline()
                            .strong()
                            .size(12.0),
                    )
                    .fill(Color32::from_rgb(24, 28, 34))
                    .stroke(Stroke::new(1.0_f32, Color32::from_rgb(40, 68, 92)))
                    .rounding(Rounding::same(6.0))
                    .min_size(vec2(btn_w, 32.0));

                    let repo_resp = ui
                        .add(repo_btn)
                        .on_hover_text("https://github.com/darkostepbro/mydiskly-desktop-id");
                    if repo_resp.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }
                    if repo_resp.clicked() {
                        open_url("https://github.com/darkostepbro/mydiskly-desktop-id");
                    }
                });
            });
        });

    ui.add_space(12.0);

    // 2. Official Repository & Technical Metadata Card
    Frame::none()
        .fill(Color32::from_rgb(18, 18, 20))
        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(36, 36, 42)))
        .rounding(Rounding::same(10.0))
        .inner_margin(Margin::symmetric(14.0, 12.0))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());

            let info_hdr = if lang == "id" {
                "INFORMASI TAUTAN & ARSITEKTUR"
            } else {
                "OFFICIAL REPOSITORY & LINKS"
            };
            ui.colored_label(
                Color32::from_rgb(150, 150, 160),
                RichText::new(info_hdr).strong().size(10.5),
            );
            ui.add_space(8.0);

            // Official Website Row
            ui.horizontal(|ui| {
                ui.colored_label(
                    Color32::from_rgb(145, 145, 155),
                    RichText::new("Official Website :").size(11.5),
                );
                let link_resp = ui.add(
                    egui::Label::new(
                        RichText::new("serojalabs.sbs/mydiskly")
                            .color(Color32::from_rgb(56, 189, 248))
                            .underline()
                            .strong()
                            .size(11.5),
                    )
                    .sense(Sense::click()),
                );
                if link_resp.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                if link_resp.clicked() {
                    open_url("https://serojalabs.sbs/mydiskly");
                }
            });

            ui.add_space(6.0);

            // GitHub Row
            ui.horizontal(|ui| {
                ui.colored_label(
                    Color32::from_rgb(145, 145, 155),
                    RichText::new("GitHub :").size(11.5),
                );
                let gh_resp = ui.add(
                    egui::Label::new(
                        RichText::new("github.com/darkostepbro/mydiskly-desktop-id")
                            .color(Color32::from_rgb(56, 189, 248))
                            .underline()
                            .strong()
                            .size(11.5),
                    )
                    .sense(Sense::click()),
                );
                if gh_resp.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                if gh_resp.clicked() {
                    open_url("https://github.com/darkostepbro/mydiskly-desktop-id");
                }
            });

            ui.add_space(6.0);

            // Author
            ui.horizontal(|ui| {
                ui.colored_label(
                    Color32::from_rgb(145, 145, 155),
                    RichText::new("Author :").size(11.5),
                );
                ui.colored_label(
                    Color32::from_rgb(235, 235, 240),
                    RichText::new(&config.author).strong().size(11.5),
                );
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            // Release : 2026
            ui.horizontal(|ui| {
                ui.colored_label(
                    Color32::from_rgb(145, 145, 155),
                    RichText::new("Release :").size(11.5),
                );
                ui.colored_label(
                    Color32::from_rgb(235, 235, 240),
                    RichText::new("2026").strong().size(11.5),
                );
            });

            ui.add_space(5.0);

            // Category : Installer app
            ui.horizontal(|ui| {
                ui.colored_label(
                    Color32::from_rgb(145, 145, 155),
                    RichText::new("Category :").size(11.5),
                );
                ui.colored_label(
                    Color32::from_rgb(235, 235, 240),
                    RichText::new("Installer app").strong().size(11.5),
                );
            });

            ui.add_space(5.0);

            // Program Language : Rust
            ui.horizontal(|ui| {
                ui.colored_label(
                    Color32::from_rgb(145, 145, 155),
                    RichText::new("Program Language :").size(11.5),
                );
                ui.colored_label(
                    Color32::from_rgb(235, 235, 240),
                    RichText::new("Rust").strong().size(11.5),
                );
            });

            ui.add_space(5.0);

            // Framework : eframe / egui 0.29 + glow (Pure Native Win32 GUI)
            ui.horizontal(|ui| {
                ui.colored_label(
                    Color32::from_rgb(145, 145, 155),
                    RichText::new("Framework :").size(11.5),
                );
                ui.colored_label(
                    Color32::from_rgb(235, 235, 240),
                    RichText::new("eframe / egui 0.29 + glow (Native GUI)")
                        .strong()
                        .size(11.5),
                );
            });
        });

    // 3. Watermark Footer (Only on About page, placed comfortably below the card per Case Photo 1)
    ui.add_space(36.0);
    ui.vertical_centered(|ui| {
        ui.colored_label(
            Color32::from_rgb(95, 95, 108),
            RichText::new("(C) 2026 MyDiskly. Give us feedback in Website & GitHub.")
                .size(10.5)
                .strong(),
        );
        ui.add_space(2.0);
        ui.colored_label(
            Color32::from_rgb(78, 78, 90),
            RichText::new("Thanks to use this super lightweight app.").size(10.0),
        );
    });
}
