use egui::{pos2, vec2, Color32, Pos2, Rect, Rounding, Sense, Stroke, Ui};

/// Draws a minimal, crisp vector Minimize `(—)` button matching `minimal_close_button`.
pub fn minimal_minimize_button(ui: &mut Ui, size: f32) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(size, size), Sense::click());
    let p = ui.painter();

    let is_hovered = resp.hovered();
    let is_active = resp.is_pointer_button_down_on();

    let bg = if is_active {
        Color32::from_rgb(44, 44, 52)
    } else if is_hovered {
        Color32::from_rgb(36, 36, 42)
    } else {
        Color32::from_rgb(24, 24, 28)
    };

    let border = if is_hovered {
        Color32::from_rgb(82, 82, 92)
    } else {
        Color32::from_rgb(44, 44, 50)
    };

    let fg = if is_hovered {
        Color32::from_rgb(255, 255, 255)
    } else {
        Color32::from_rgb(185, 185, 195)
    };

    p.rect_filled(rect, Rounding::same(6.0), bg);
    p.rect_stroke(rect, Rounding::same(6.0), Stroke::new(1.0_f32, border));

    let c = rect.center();
    let d = size * 0.21;
    p.line_segment(
        [pos2(c.x - d, c.y), pos2(c.x + d, c.y)],
        Stroke::new(1.5_f32, fg),
    );

    if is_hovered {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp
}

/// Draws a minimal, crisp vector Close `(x)` button that never depends on font glyphs.
pub fn minimal_close_button(ui: &mut Ui, size: f32) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(size, size), Sense::click());
    let p = ui.painter();

    let is_hovered = resp.hovered();
    let is_active = resp.is_pointer_button_down_on();

    let bg = if is_active {
        Color32::from_rgb(56, 30, 34)
    } else if is_hovered {
        Color32::from_rgb(38, 38, 44)
    } else {
        Color32::from_rgb(24, 24, 28)
    };

    let border = if is_hovered {
        Color32::from_rgb(82, 82, 92)
    } else {
        Color32::from_rgb(44, 44, 50)
    };

    let fg = if is_hovered {
        Color32::from_rgb(255, 255, 255)
    } else {
        Color32::from_rgb(185, 185, 195)
    };

    p.rect_filled(rect, Rounding::same(6.0), bg);
    p.rect_stroke(rect, Rounding::same(6.0), Stroke::new(1.0_f32, border));

    let c = rect.center();
    let d = size * 0.19;
    let stroke = Stroke::new(1.5_f32, fg);
    p.line_segment([pos2(c.x - d, c.y - d), pos2(c.x + d, c.y + d)], stroke);
    p.line_segment([pos2(c.x + d, c.y - d), pos2(c.x - d, c.y + d)], stroke);

    if is_hovered {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp
}

/// Draws a crisp monochrome hard-drive vector icon.
pub fn icon_hard_drive(ui: &mut Ui, size: f32, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
    let p = ui.painter();
    let s = Stroke::new(1.4_f32, color);

    let r = rect.shrink(1.5);
    p.rect_stroke(r, Rounding::same(2.5), s);
    let mid_y = r.min.y + r.height() * 0.62;
    p.line_segment([pos2(r.min.x, mid_y), pos2(r.max.x, mid_y)], s);
    p.circle_filled(pos2(r.max.x - 3.5, mid_y + (r.max.y - mid_y) * 0.5), 1.2, color);
}

/// Draws a crisp monochrome pulse/activity vector icon for S.M.A.R.T & I/O.
pub fn icon_activity(ui: &mut Ui, size: f32, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
    let p = ui.painter();
    let s = Stroke::new(1.5_f32, color);

    let r = rect.shrink(1.5);
    let y_mid = r.center().y;
    let pts = [
        pos2(r.min.x, y_mid),
        pos2(r.min.x + r.width() * 0.25, y_mid),
        pos2(r.min.x + r.width() * 0.42, r.min.y + 1.0),
        pos2(r.min.x + r.width() * 0.60, r.max.y - 1.0),
        pos2(r.min.x + r.width() * 0.75, y_mid),
        pos2(r.max.x, y_mid),
    ];
    for w in pts.windows(2) {
        p.line_segment([w[0], w[1]], s);
    }
}

/// Draws a crisp monochrome trash/cleaner vector icon.
pub fn icon_trash(ui: &mut Ui, size: f32, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
    let p = ui.painter();
    let s = Stroke::new(1.4_f32, color);

    let r = rect.shrink(2.0);
    let top_y = r.min.y + 2.5;
    p.line_segment([pos2(r.min.x, top_y), pos2(r.max.x, top_y)], s);
    p.line_segment(
        [pos2(r.center().x - 2.5, r.min.y), pos2(r.center().x + 2.5, r.min.y)],
        s,
    );
    let body = Rect::from_min_max(pos2(r.min.x + 1.5, top_y), pos2(r.max.x - 1.5, r.max.y));
    p.rect_stroke(body, Rounding { nw: 0.0, ne: 0.0, sw: 2.0, se: 2.0 }, s);
}

/// Draws a crisp monochrome sliders/settings vector icon.
pub fn icon_settings(ui: &mut Ui, size: f32, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
    let p = ui.painter();
    let s = Stroke::new(1.4_f32, color);

    let r = rect.shrink(2.0);
    for (i, frac) in [0.28_f32, 0.72_f32].iter().enumerate() {
        let y = r.min.y + r.height() * frac;
        p.line_segment([pos2(r.min.x, y), pos2(r.max.x, y)], s);
        let kx = if i == 0 {
            r.min.x + r.width() * 0.35
        } else {
            r.min.x + r.width() * 0.68
        };
        p.circle_filled(pos2(kx, y), 2.2, color);
    }
}

/// Draws a monochrome browser/globe vector icon.
pub fn icon_browser(ui: &mut Ui, size: f32, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
    let p = ui.painter();
    let s = Stroke::new(1.3_f32, color);

    let c = rect.center();
    let rad = (size * 0.5) - 1.5;
    p.circle_stroke(c, rad, s);
    p.line_segment([pos2(c.x - rad, c.y), pos2(c.x + rad, c.y)], s);
    p.line_segment([pos2(c.x, c.y - rad), pos2(c.x, c.y + rad)], s);
}

/// Draws a monochrome folder vector icon.
#[allow(dead_code)]
pub fn icon_folder(ui: &mut Ui, size: f32, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
    let p = ui.painter();
    let s = Stroke::new(1.3_f32, color);

    let r = rect.shrink(1.8);
    let body = Rect::from_min_max(pos2(r.min.x, r.min.y + 2.5), r.max);
    p.rect_stroke(body, Rounding::same(2.0), s);
    p.line_segment(
        [pos2(r.min.x + 1.0, r.min.y + 1.0), pos2(r.min.x + r.width() * 0.45, r.min.y + 1.0)],
        s,
    );
}

/// Draws a monochrome info/properties circle vector icon.
#[allow(dead_code)]
pub fn icon_info(ui: &mut Ui, size: f32, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
    let p = ui.painter();
    let s = Stroke::new(1.3_f32, color);

    let c = rect.center();
    let rad = (size * 0.5) - 1.5;
    p.circle_stroke(c, rad, s);
    p.circle_filled(pos2(c.x, c.y - rad * 0.38), 1.0, color);
    p.line_segment([pos2(c.x, c.y - rad * 0.05), pos2(c.x, c.y + rad * 0.48)], s);
}

/// Draws a monochrome shield-check vector icon.
pub fn icon_shield(ui: &mut Ui, size: f32, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
    let p = ui.painter();
    let s = Stroke::new(1.4_f32, color);

    let r = rect.shrink(1.8);
    let top = pos2(r.center().x, r.min.y);
    let left = pos2(r.min.x, r.min.y + 2.5);
    let right = pos2(r.max.x, r.min.y + 2.5);
    let bot_left = pos2(r.min.x + 1.5, r.min.y + r.height() * 0.65);
    let bot_right = pos2(r.max.x - 1.5, r.min.y + r.height() * 0.65);
    let bot = pos2(r.center().x, r.max.y);

    p.line_segment([top, left], s);
    p.line_segment([top, right], s);
    p.line_segment([left, bot_left], s);
    p.line_segment([right, bot_right], s);
    p.line_segment([bot_left, bot], s);
    p.line_segment([bot_right, bot], s);
}

/// Draws a monochrome bell icon for notifications.
pub fn icon_bell(ui: &mut Ui, size: f32, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
    let p = ui.painter();
    let s = Stroke::new(1.3_f32, color);

    let r = rect.shrink(2.0);
    let body = Rect::from_min_max(
        pos2(r.min.x + 2.0, r.min.y + 1.0),
        pos2(r.max.x - 2.0, r.max.y - 2.5),
    );
    p.rect_stroke(body, Rounding { nw: 4.0, ne: 4.0, sw: 1.0, se: 1.0 }, s);
    p.line_segment(
        [pos2(r.min.x, r.max.y - 2.5), pos2(r.max.x, r.max.y - 2.5)],
        s,
    );
    p.circle_filled(pos2(r.center().x, r.max.y - 0.8), 1.2, color);
}

/// Draws a monochrome chevron arrow (down or up).
pub fn icon_chevron(ui: &mut Ui, size: f32, color: Color32, expanded: bool) {
    let (rect, _) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
    let p = ui.painter();
    let s = Stroke::new(1.5_f32, color);

    let c = rect.center();
    let dx = size * 0.26;
    let dy = size * 0.15;
    if expanded {
        p.line_segment([pos2(c.x - dx, c.y + dy), pos2(c.x, c.y - dy)], s);
        p.line_segment([pos2(c.x, c.y - dy), pos2(c.x + dx, c.y + dy)], s);
    } else {
        p.line_segment([pos2(c.x - dx, c.y - dy), pos2(c.x, c.y + dy)], s);
        p.line_segment([pos2(c.x, c.y + dy), pos2(c.x + dx, c.y - dy)], s);
    }
}

/// Draws a monochrome refresh arrow icon.
#[allow(dead_code)]
pub fn icon_refresh(ui: &mut Ui, size: f32, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
    let p = ui.painter();
    let s = Stroke::new(1.3_f32, color);

    let c = rect.center();
    let rad = (size * 0.5) - 2.0;
    p.circle_stroke(c, rad, s);
    let tip = Pos2::new(c.x + rad, c.y - 1.0);
    p.line_segment([tip, pos2(tip.x - 2.5, tip.y - 2.0)], s);
    p.line_segment([tip, pos2(tip.x + 1.5, tip.y - 2.5)], s);
}
