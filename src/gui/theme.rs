use super::tokens::{border, bg};
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
    v.interact_cursor = Some(egui::CursorIcon::PointingHand);
    v.widgets.inactive.expansion = 0.0;
    ctx.set_visuals(v);

    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(10.0, 8.0);
    style.spacing.button_padding = egui::vec2(14.0, 7.0);
    ctx.set_style(style);
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

/// Soft spotlight on the play surface so panels do not sit on flat void.
pub fn play_table_atmosphere(painter: &egui::Painter, rect: egui::Rect) {
    painter.circle_filled(
        rect.center(),
        rect.width().min(rect.height()) * 0.45,
        bg_felt().gamma_multiply(0.2),
    );
    let top = egui::Rect::from_min_max(rect.min, egui::pos2(rect.max.x, rect.min.y + 72.0));
    painter.rect_filled(top, 0.0, egui::Color32::BLACK.gamma_multiply(0.28));
    let bot = egui::Rect::from_min_max(egui::pos2(rect.min.x, rect.max.y - 56.0), rect.max);
    painter.rect_filled(bot, 0.0, egui::Color32::BLACK.gamma_multiply(0.22));
}

pub fn gold_button(ui: &mut egui::Ui, label: &str, small: bool) -> egui::Response {
    let pad = if small {
        egui::vec2(10.0, 4.0)
    } else {
        egui::vec2(16.0, 8.0)
    };
    let size = if small { 12.0 } else { 15.0 };
    ui.add(
        egui::Button::new(egui::RichText::new(label).font(font_body(size)).strong())
            .fill(bg_felt())
            .stroke(egui::Stroke::new(1.5_f32, accent_gold()))
            .corner_radius(6.0)
            .min_size(pad),
    )
}

pub fn tier_badge(ui: &mut egui::Ui, label: &str, tier: u8) {
    let c = tier_accent(tier);
    egui::Frame::new()
        .fill(c.gamma_multiply(0.22))
        .stroke(egui::Stroke::new(1.0_f32, c.gamma_multiply(0.85)))
        .corner_radius(4.0)
        .inner_margin(egui::vec2(6.0, 3.0))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(label)
                    .font(font_body(11.0))
                    .strong()
                    .color(c),
            );
        });
}

pub fn board_panel(ui: &mut egui::Ui, title: &str, inner: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(bg_felt())
        .stroke(egui::Stroke::new(2.0_f32, brass()))
        .corner_radius(12.0)
        .inner_margin(14.0)
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(title)
                    .font(font_display(19.0))
                    .color(accent_gold()),
            );
            ui.add_space(2.0);
            let line_w = ui.available_width() * 0.4;
            let y = ui.min_rect().bottom() - 2.0;
            ui.painter().line_segment(
                [
                    egui::pos2(ui.min_rect().left(), y),
                    egui::pos2(ui.min_rect().left() + line_w, y),
                ],
                egui::Stroke::new(1.5_f32, accent_gold().gamma_multiply(0.55)),
            );
            ui.add_space(8.0);
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
