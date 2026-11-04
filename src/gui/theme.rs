use super::tokens::{self, border, bg};
use eframe::egui;

pub fn apply(ctx: &egui::Context) {
    let mut v = egui::Visuals::dark();
    v.window_fill = bg_main();
    v.panel_fill = bg_panel();
    v.extreme_bg_color = bg::base_dark();
    v.widgets.noninteractive.bg_fill = bg::surface_velvet();
    v.widgets.inactive.bg_fill = bg::surface_velvet();
    v.widgets.hovered.bg_fill = bg::panel_emerald();
    v.widgets.active.bg_fill = bg::panel_emerald();
    v.selection.bg_fill = border::gold_accent().gamma_multiply(0.35);
    v.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(220, 215, 200);
    ctx.set_visuals(v);
}

pub fn bg_main() -> egui::Color32 {
    bg::base_dark()
}

pub fn bg_panel() -> egui::Color32 {
    bg::surface_velvet()
}

pub fn bg_felt() -> egui::Color32 {
    bg::panel_emerald()
}

pub fn accent_gold() -> egui::Color32 {
    border::gold_accent()
}

pub fn brass() -> egui::Color32 {
    border::brass()
}

pub fn tier_accent(tier: u8) -> egui::Color32 {
    match tier {
        1 => brass().gamma_multiply(0.85),
        2 => egui::Color32::from_rgb(170, 178, 195),
        _ => accent_gold(),
    }
}

pub fn font_display(size: f32) -> egui::FontId {
    egui::FontId::new(size, egui::FontFamily::Name("display".into()))
}

pub fn font_body(size: f32) -> egui::FontId {
    egui::FontId::proportional(size)
}

pub fn velvet_overlay(painter: &egui::Painter, rect: egui::Rect) {
    let step = 6.0;
    let mut y = rect.min.y;
    while y < rect.max.y {
        let stripe = egui::Rect::from_min_max(
            egui::pos2(rect.min.x, y),
            egui::pos2(rect.max.x, (y + step).min(rect.max.y)),
        );
        let alt = ((y - rect.min.y) / step) as i32 % 2 == 0;
        if alt {
            painter.rect_filled(
                stripe,
                0.0,
                egui::Color32::BLACK.gamma_multiply(0.06),
            );
        }
        y += step;
    }
}

pub fn board_panel(ui: &mut egui::Ui, title: &str, inner: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(bg_felt())
        .stroke(egui::Stroke::new(1.5_f32, brass()))
        .corner_radius(10.0)
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(title)
                    .font(font_display(18.0))
                    .color(accent_gold()),
            );
            ui.add_space(6.0);
            inner(ui);
        });
}

pub fn sidebar_panel(ui: &mut egui::Ui, inner: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(bg_panel())
        .stroke(egui::Stroke::new(1.5_f32, brass().gamma_multiply(0.6)))
        .corner_radius(10.0)
        .inner_margin(10.0)
        .show(ui, inner);
}

pub fn player_board_frame(ui: &mut egui::Ui, active: bool, inner: impl FnOnce(&mut egui::Ui)) {
    let stroke = if active {
        egui::Stroke::new(2.0_f32, accent_gold())
    } else {
        egui::Stroke::new(1.0_f32, brass().gamma_multiply(0.5))
    };
    egui::Frame::new()
        .fill(bg::panel_emerald())
        .stroke(stroke)
        .corner_radius(8.0)
        .inner_margin(10.0)
        .show(ui, inner);
}
