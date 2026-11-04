use super::draw::{self, GemChipOpts};
use super::theme;
use super::tokens::{border, bg};
use eframe::egui;
use gemsmith::card::Card;
use gemsmith::gem::{Gem, GemCounts};
use gemsmith::locale::Locale;

pub const MARKET_CARD_W: f32 = 104.0;
pub const MARKET_CARD_H: f32 = 198.0;
pub const MINI_CARD_W: f32 = 56.0;
pub const MINI_CARD_H: f32 = 100.0;

fn header_h(rect: egui::Rect) -> f32 {
    (rect.height() * 0.152).clamp(16.0, 32.0)
}

fn footer_h(rect: egui::Rect) -> f32 {
    (rect.height() * 0.205).clamp(20.0, 42.0)
}

pub struct CardPaintCtx<'a> {
    pub time: f32,
    pub hover: f32,
    pub phase: f32,
    pub affordable: f32,
    pub reserved_seal: bool,
    pub loc: &'a Locale,
    pub cost: &'a GemCounts,
    pub wallet: Option<(&'a GemCounts, &'a GemCounts)>,
}

pub fn card_hover_anim(ui: &egui::Ui, id: egui::Id, hovered: bool) -> f32 {
    ui.ctx()
        .animate_bool_responsive(id.with("hover"), hovered)
}

pub fn paint_development_card(
    painter: &egui::Painter,
    rect: egui::Rect,
    tier: u8,
    card: &Card,
    ctx: &CardPaintCtx<'_>,
) {
    let lift = ctx.hover * 6.0;
    let rect = rect.translate(egui::vec2(0.0, -lift));
    let pulse = (ctx.time * 2.2 + ctx.phase).sin() * 0.5 + 0.5;

    paint_drop_shadow(painter, rect, ctx.hover, ctx.affordable, pulse);
    painter.rect_filled(rect, 10.0, bg::card_face());
    let ribbon = egui::Rect::from_min_max(rect.min, egui::pos2(rect.max.x, rect.min.y + 5.0));
    painter.rect_filled(ribbon, 10.0, theme::tier_accent(tier));
    painter.rect_filled(
        rect.shrink(3.0),
        8.0,
        bg::card_face().lerp_to_gamma(bg::panel_emerald(), 0.15),
    );
    theme::velvet_overlay(painter, rect.shrink(5.0));
    paint_artwork_zone(painter, rect, tier, card, ctx);
    paint_cost_footer(painter, rect, ctx);
    paint_brass_frame(painter, rect, ctx.hover, ctx.affordable, pulse);
    paint_header_zone(painter, rect, card, ctx);
    if ctx.reserved_seal {
        paint_brass_seal(painter, rect);
    }
    if ctx.affordable > 0.05 {
        let glow = border::gold_accent().gamma_multiply(ctx.affordable * (0.25 + pulse * 0.2));
        painter.rect_stroke(
            rect.expand(2.0 + pulse * 2.0),
            12.0,
            egui::Stroke::new(2.0_f32, glow),
            egui::StrokeKind::Outside,
        );
    }
}

pub fn paint_brass_corners(painter: &egui::Painter, rect: egui::Rect) {
    let s = 12.0;
    let stroke = egui::Stroke::new(2.0_f32, theme::brass());
    let corners = [
        (rect.left_top(), egui::vec2(s, 0.0), egui::vec2(0.0, s)),
        (rect.right_top(), egui::vec2(-s, 0.0), egui::vec2(0.0, s)),
        (rect.left_bottom(), egui::vec2(s, 0.0), egui::vec2(0.0, -s)),
        (rect.right_bottom(), egui::vec2(-s, 0.0), egui::vec2(0.0, -s)),
    ];
    for (o, dx, dy) in corners {
        painter.line_segment([o, o + dx], stroke);
        painter.line_segment([o, o + dy], stroke);
    }
}

fn paint_drop_shadow(
    painter: &egui::Painter,
    rect: egui::Rect,
    hover: f32,
    affordable: f32,
    pulse: f32,
) {
    let spread = 6.0 + hover * 8.0 + affordable * pulse * 4.0;
    let shadow = rect.translate(egui::vec2(0.0, 4.0 + hover * 2.0)).expand(spread);
    painter.rect_filled(shadow, 12.0, egui::Color32::BLACK.gamma_multiply(0.55));
}

fn paint_brass_frame(
    painter: &egui::Painter,
    rect: egui::Rect,
    hover: f32,
    affordable: f32,
    pulse: f32,
) {
    let w = 2.0 + hover * 0.5 + affordable * pulse * 0.5;
    painter.rect_stroke(
        rect,
        10.0,
        egui::Stroke::new(w, theme::brass()),
        egui::StrokeKind::Inside,
    );
    paint_brass_corners(painter, rect);
}

fn paint_artwork_zone(
    painter: &egui::Painter,
    rect: egui::Rect,
    tier: u8,
    card: &Card,
    ctx: &CardPaintCtx<'_>,
) {
    let hh = header_h(rect);
    let fh = footer_h(rect);
    let art = egui::Rect::from_min_max(
        rect.min + egui::vec2(6.0, hh + 2.0),
        rect.max - egui::vec2(6.0, fh + 2.0),
    );
    painter.rect_filled(art, 6.0, bg::card_mat());
    painter.rect_stroke(
        art,
        6.0,
        egui::Stroke::new(1.0_f32, theme::brass().gamma_multiply(0.45)),
        egui::StrokeKind::Inside,
    );
    super::card_scenes::paint_scene(painter, art.shrink(2.0), tier, card.art, ctx.time);
}

fn paint_cost_footer(painter: &egui::Painter, rect: egui::Rect, ctx: &CardPaintCtx<'_>) {
    let fh = footer_h(rect);
    let footer = egui::Rect::from_min_max(
        egui::pos2(rect.min.x, rect.max.y - fh),
        rect.max,
    );
    painter.rect_filled(footer, 0.0, egui::Color32::from_rgb(22, 20, 17));
    painter.line_segment(
        [footer.left_top(), footer.right_top()],
        egui::Stroke::new(1.0_f32, theme::brass().gamma_multiply(0.5)),
    );
    let chip = (fh * 0.42).clamp(14.0, 20.0);
    let mut x = footer.min.x + 6.0;
    let cy = footer.center().y;
    for g in Gem::BASIC {
        let n = ctx.cost.get(g);
        if n == 0 {
            continue;
        }
        draw::paint_gem_shape(
            painter,
            egui::pos2(x + chip * 0.45, cy),
            chip,
            g,
            GemChipOpts::default(),
        );
        painter.text(
            egui::pos2(x + chip * 0.9, cy + chip * 0.35),
            egui::Align2::CENTER_TOP,
            format!("{n}"),
            theme::font_body((chip * 0.55).clamp(9.0, 11.0)),
            theme::accent_gold(),
        );
        x += chip + 8.0;
    }
}

fn paint_header_zone(
    painter: &egui::Painter,
    rect: egui::Rect,
    card: &Card,
    ctx: &CardPaintCtx<'_>,
) {
    let hh = header_h(rect);
    let header = egui::Rect::from_min_max(rect.min, egui::pos2(rect.max.x, rect.min.y + hh));
    painter.rect_filled(header, 0.0, egui::Color32::from_rgb(24, 21, 18));
    let pts_size = (hh * 0.75).clamp(16.0, 26.0);
    painter.text(
        header.left_center() + egui::vec2(8.0, 0.0),
        egui::Align2::LEFT_CENTER,
        ctx.loc.points_gain(card.points),
        theme::font_display(pts_size),
        theme::accent_gold(),
    );
    let badge_c = header.right_center() + egui::vec2(-18.0, 0.0);
    draw::paint_gem_shape(painter, badge_c, hh * 0.85, card.bonus, GemChipOpts::default());
}

fn paint_brass_seal(painter: &egui::Painter, rect: egui::Rect) {
    let c = rect.left_top() + egui::vec2(18.0, 14.0);
    painter.circle_filled(c, 10.0, theme::brass().gamma_multiply(0.85));
    painter.circle_stroke(c, 10.0, egui::Stroke::new(1.5_f32, theme::accent_gold()));
    painter.text(
        c,
        egui::Align2::CENTER_CENTER,
        "R",
        theme::font_display(11.0),
        bg::base_dark(),
    );
}

pub fn menu_gem_phase(index: usize, time: f32) -> f32 {
    (time * 1.6 + index as f32 * 0.55).sin()
}
