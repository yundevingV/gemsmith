use super::draw::{self, GemChipOpts};
use super::theme;
use super::tokens::{border, bg};
use eframe::egui;
use gemsmith::card::Card;
use gemsmith::gem::{Gem, GemCounts};
use gemsmith::locale::Locale;

pub const MARKET_CARD_W: f32 = 152.0;
pub const MARKET_CARD_H: f32 = 252.0;
pub const MINI_CARD_W: f32 = 56.0;
pub const MINI_CARD_H: f32 = 100.0;

fn header_h(rect: egui::Rect) -> f32 {
    (rect.height() * 0.152).clamp(16.0, 34.0)
}

fn cost_slot_count(cost: &GemCounts) -> usize {
    Gem::BASIC.iter().filter(|g| cost.get(**g) > 0).count()
}

fn cost_grid(n: usize) -> (usize, usize) {
    match n {
        1 => (1, 1),
        2 => (2, 1),
        3 => (3, 1),
        4 => (4, 1),
        _ => (3, 2),
    }
}

fn footer_h(rect: egui::Rect, cost_slots: usize) -> f32 {
    let (_, rows) = cost_grid(cost_slots.max(1));
    let row_h = 18.0;
    (row_h * rows as f32 + 8.0)
        .clamp(24.0, 44.0)
        .min(rect.height() * 0.26)
}

pub struct CardPaintCtx<'a> {
    pub time: f32,
    pub hover: f32,
    pub phase: f32,
    pub affordable: f32,
    pub reserved_seal: bool,
    pub loc: &'a Locale,
    pub cost: &'a GemCounts,
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
    let n_cost = cost_slot_count(ctx.cost);
    let hh = header_h(rect);
    let fh = footer_h(rect, n_cost);
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
    let inner = art.shrink(2.0);
    let clipped = painter.with_clip_rect(inner);
    super::card_scenes::paint_scene(&clipped, inner, tier, card.art, ctx.time);
}

fn paint_cost_footer(painter: &egui::Painter, rect: egui::Rect, ctx: &CardPaintCtx<'_>) {
    let gems: Vec<(Gem, u8)> = Gem::BASIC
        .iter()
        .filter_map(|&g| {
            let n = ctx.cost.get(g);
            (n > 0).then_some((g, n))
        })
        .collect();
    let fh = footer_h(rect, gems.len());
    let footer = egui::Rect::from_min_max(
        egui::pos2(rect.min.x, rect.max.y - fh),
        rect.max,
    );
    painter.rect_filled(footer, 0.0, egui::Color32::from_rgb(22, 20, 17));
    painter.line_segment(
        [footer.left_top(), footer.right_top()],
        egui::Stroke::new(1.0_f32, theme::brass().gamma_multiply(0.5)),
    );
    if gems.is_empty() {
        return;
    }

    let inner = footer.shrink2(egui::vec2(5.0, 4.0));
    let p = painter.with_clip_rect(inner);
    let (cols, rows) = cost_grid(gems.len());
    let cell_w = inner.width() / cols as f32;
    let cell_h = inner.height() / rows as f32;
    let chip = (cell_w * 0.42).clamp(10.0, 19.0).min(cell_h * 0.72);

    for (i, (g, n)) in gems.iter().enumerate() {
        let col = i % cols;
        let row = i / cols;
        let cell = egui::Rect::from_min_max(
            egui::pos2(inner.min.x + cell_w * col as f32, inner.min.y + cell_h * row as f32),
            egui::pos2(inner.min.x + cell_w * (col + 1) as f32, inner.min.y + cell_h * (row + 1) as f32),
        );
        let cy = cell.center().y;
        let gem_c = egui::pos2(cell.min.x + cell_w * 0.28, cy);
        draw::paint_gem_shape(&p, gem_c, chip, *g, GemChipOpts::default());
        p.text(
            egui::pos2(gem_c.x + chip * 0.52, cy),
            egui::Align2::LEFT_CENTER,
            format!("{n}"),
            theme::font_body(12.0),
            theme::accent_gold(),
        );
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
    draw::paint_gem_shape(painter, badge_c, hh * 0.88, card.bonus, GemChipOpts::default());
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
