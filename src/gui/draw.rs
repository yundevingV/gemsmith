use super::cards::{self, CardPaintCtx};
use super::theme;
use super::tokens::{self, border};
use eframe::egui;
use gemsmith::card::Card;
use gemsmith::game::Action;
use gemsmith::gem::{Gem, GemCounts};
use gemsmith::locale::Locale;
use gemsmith::patron::Patron;
use std::f32::consts::TAU;

#[derive(Clone, Copy, Default)]
pub struct GemChipOpts {
    pub selected: bool,
    pub disabled: bool,
}

pub fn gem_fill(g: Gem) -> egui::Color32 {
    tokens::gem_primary(g)
}

pub fn gem_dark(g: Gem) -> egui::Color32 {
    let (_, dark) = tokens::gem_gradient(g);
    dark
}

fn oct_points(center: egui::Pos2, radius: f32) -> [egui::Pos2; 8] {
    std::array::from_fn(|i| {
        let a = TAU * i as f32 / 8.0 - TAU / 8.0;
        center + egui::vec2(radius * a.cos(), radius * a.sin())
    })
}

pub(crate) fn paint_gem_shape(
    painter: &egui::Painter,
    center: egui::Pos2,
    size: f32,
    g: Gem,
    opts: GemChipOpts,
) {
    let mut alpha = if opts.disabled { 0.45 } else { 1.0 };
    if opts.disabled {
        // grayscale-ish via reduced saturation
        alpha = 0.5;
    }
    let r = size * 0.38;
    let (light, dark) = tokens::gem_gradient(g);
    let fill = if opts.disabled {
        egui::Color32::from_gray(90)
    } else {
        light.lerp_to_gamma(dark, 0.45)
    };

    if matches!(g, Gem::Diamond) {
        let w = r * 1.05;
        let h = r * 1.25;
        let pts = [
            center + egui::vec2(0.0, -h),
            center + egui::vec2(w, 0.0),
            center + egui::vec2(0.0, h * 0.55),
            center + egui::vec2(-w, 0.0),
        ];
        painter.add(egui::Shape::convex_polygon(
            pts.to_vec(),
            fill.gamma_multiply(alpha),
            egui::Stroke::new(1.5_f32, border::gold_accent().gamma_multiply(0.85 * alpha)),
        ));
        return;
    }

    let pts = oct_points(center, r);
    painter.add(egui::Shape::convex_polygon(
        pts.to_vec(),
        fill.gamma_multiply(alpha),
        egui::Stroke::new(
            2.0_f32,
            border::gold_accent().gamma_multiply(0.75 * alpha),
        ),
    ));
    painter.add(egui::Shape::convex_polygon(
        oct_points(center + egui::vec2(-r * 0.15, -r * 0.2), r * 0.35)
            .to_vec(),
        light.gamma_multiply(0.35 * alpha),
        egui::Stroke::NONE,
    ));
    let glow = tokens::gem_glow_alpha(g) * alpha;
    if opts.selected || !opts.disabled {
        painter.circle_filled(
            center,
            r * 1.05,
            gem_fill(g).gamma_multiply(glow * 0.15),
        );
    }
}

pub fn draw_gem_chip(
    ui: &mut egui::Ui,
    g: Gem,
    size: f32,
    label: Option<&str>,
    sense: egui::Sense,
    opts: GemChipOpts,
) -> egui::Response {
    let lift = if opts.selected { 8.0 } else { 0.0 };
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(size, size + lift), sense);
    let hover = if sense.contains(egui::Sense::click()) && !opts.disabled {
        cards::card_hover_anim(ui, resp.id, resp.hovered() || opts.selected)
    } else {
        0.0
    };
    let y_off = -lift - hover * 3.0;
    let bounce = rect.translate(egui::vec2(0.0, y_off));
    if ui.is_rect_visible(bounce) {
        let painter = ui.painter();
        if opts.selected {
            painter.circle_filled(
                bounce.center(),
                size * 0.55,
                border::gold_accent().gamma_multiply(0.25),
            );
        }
        paint_gem_shape(painter, bounce.center(), size + hover * 3.0, g, opts);
        let text_color = if matches!(g, Gem::Diamond | Gem::Gold) {
            gem_dark(g)
        } else {
            egui::Color32::WHITE
        };
        painter.text(
            bounce.center(),
            egui::Align2::CENTER_CENTER,
            g.label(),
            egui::FontId::proportional(size * 0.32),
            text_color.gamma_multiply(if opts.disabled { 0.5 } else { 1.0 }),
        );
        if let Some(t) = label {
            painter.text(
                egui::pos2(bounce.center().x, bounce.max.y + 2.0),
                egui::Align2::CENTER_TOP,
                t,
                theme::font_body(11.0),
                ui.visuals().text_color(),
            );
        }
    }
    if sense.contains(egui::Sense::click()) && !opts.disabled {
        resp.on_hover_cursor(egui::CursorIcon::PointingHand)
    } else {
        resp
    }
}

pub fn cost_row(ui: &mut egui::Ui, cost: &GemCounts, wallet: Option<(&GemCounts, &GemCounts)>) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        for g in Gem::BASIC {
            let n = cost.get(g);
            if n > 0 {
                let ok = wallet.map_or(true, |(t, b)| {
                    t.get(g).saturating_add(b.get(g)) >= n
                });
                ui.horizontal(|ui| {
                    draw_gem_chip(
                        ui,
                        g,
                        20.0,
                        None,
                        egui::Sense::hover(),
                        GemChipOpts {
                            disabled: !ok,
                            ..Default::default()
                        },
                    );
                    ui.label(
                        egui::RichText::new(format!("{n}"))
                            .font(theme::font_body(13.0))
                            .color(theme::accent_gold()),
                    );
                });
            }
        }
    });
}

pub fn bank_row(
    ui: &mut egui::Ui,
    bank: &GemCounts,
    pick: &[Gem],
    interactive: bool,
    held_tokens: u8,
    gem_size: f32,
    row_h: f32,
) -> Option<Gem> {
    const COLS: f32 = 6.0;
    let row_w = ui.available_width();
    let cell_w = (row_w / COLS).max(44.0);
    let mut clicked = None;

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        for g in Gem::BASIC {
            if bank_cell(
                ui,
                g,
                bank.get(g),
                cell_w,
                row_h,
                gem_size,
                pick,
                bank,
                interactive,
                held_tokens,
            ) {
                clicked = Some(g);
            }
        }
        bank_cell(
            ui,
            Gem::Gold,
            bank.gold,
            cell_w,
            row_h,
            gem_size,
            pick,
            bank,
            false,
            held_tokens,
        );
    });
    clicked
}

fn bank_cell(
    ui: &mut egui::Ui,
    g: Gem,
    count: u8,
    w: f32,
    h: f32,
    gem: f32,
    pick: &[Gem],
    bank: &GemCounts,
    interactive: bool,
    held_tokens: u8,
) -> bool {
    let mut hit = false;
    ui.allocate_ui_with_layout(
        egui::vec2(w, h),
        egui::Layout::top_down(egui::Align::Center),
        |ui| {
            ui.add_space(2.0);
            let n = super::gem_pick::pick_count(pick, g);
            let can_click = interactive
                && g.is_basic()
                && count > 0
                && super::gem_pick::can_toggle_pick(pick, g, bank, held_tokens);
            let sense = if can_click {
                egui::Sense::click()
            } else {
                egui::Sense::hover()
            };
            let count_lbl = (n > 0).then(|| format!("×{n}"));
            let disabled = count == 0 || (interactive && g.is_basic() && !can_click && n == 0);
            let r = draw_gem_chip(
                ui,
                g,
                gem,
                count_lbl.as_deref(),
                sense,
                GemChipOpts {
                    selected: n > 0,
                    disabled,
                    ..Default::default()
                },
            );
            if can_click && r.clicked() {
                hit = true;
            }
            ui.label(
                egui::RichText::new(format!("{count}"))
                    .font(theme::font_body(13.0))
                    .color(theme::accent_gold()),
            );
        },
    );
    hit
}

fn paint_card_face(
    ui: &egui::Ui,
    rect: egui::Rect,
    tier: u8,
    c: &Card,
    loc: &Locale,
    hover: f32,
    phase: f32,
    affordable: f32,
    reserved_seal: bool,
    cost: &GemCounts,
) {
    let time = ui.input(|i| i.time) as f32;
    let ctx = CardPaintCtx {
        time,
        hover,
        phase,
        affordable,
        reserved_seal,
        loc,
        cost,
    };
    cards::paint_development_card(ui.painter(), rect, tier, c, &ctx);
}

pub fn market_card(
    ui: &mut egui::Ui,
    c: &Card,
    tier: u8,
    slot: usize,
    human: bool,
    can_buy: bool,
    pending: &mut Option<Action>,
    loc: &Locale,
    card_w: f32,
    card_h: f32,
) {
    ui.vertical(|ui| {
        ui.set_width(card_w);
        let id = ui.id().with(("market", tier, slot));
        let (rect, resp) = ui.allocate_exact_size(egui::vec2(card_w, card_h), egui::Sense::click());
        let hover = cards::card_hover_anim(ui, id, resp.hovered());
        let afford_anim = if human {
            ui.ctx()
                .animate_bool_responsive(id.with("afford"), can_buy)
        } else {
            0.0
        };
        if ui.is_rect_visible(rect) {
            paint_card_face(
                ui,
                rect,
                tier,
                c,
                loc,
                hover,
                tier as f32 + slot as f32 * 0.3,
                afford_anim,
                false,
                &c.cost,
            );
        }
        if human {
            let _ = resp.on_hover_cursor(egui::CursorIcon::PointingHand);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                ui.add_enabled_ui(can_buy, |ui| {
                    if theme::gold_button(ui, loc.buy(), true).clicked() {
                        *pending = Some(Action::BuyTable { tier, slot });
                    }
                });
                if theme::gold_button(ui, loc.hold(), true).clicked() {
                    *pending = Some(Action::ReserveTable { tier, slot });
                }
            });
        }
    });
}

pub fn patron_tile(ui: &mut egui::Ui, idx: usize, p: &Patron, loc: &Locale, compact: bool) {
    let paint_tile = |ui: &mut egui::Ui, side: f32| {
        let (tile_rect, _) = ui.allocate_exact_size(egui::vec2(side, side), egui::Sense::hover());
        if !ui.is_rect_visible(tile_rect) {
            return;
        }
        let pter = ui.painter();
        pter.rect_filled(tile_rect, 4.0, tokens::bg::panel_emerald());
        pter.rect_stroke(
            tile_rect,
            4.0,
            egui::Stroke::new(2.0_f32, theme::brass()),
            egui::StrokeKind::Inside,
        );
        cards::paint_brass_corners(pter, tile_rect);
        pter.text(
            tile_rect.left_top() + egui::vec2(6.0, 4.0),
            egui::Align2::LEFT_TOP,
            loc.points_fmt(3),
            theme::font_display(if compact { 16.0 } else { 22.0 }),
            theme::accent_gold(),
        );
        pter.text(
            tile_rect.center(),
            egui::Align2::CENTER_CENTER,
            format!("{}", idx + 1),
            theme::font_display(if compact { 14.0 } else { 18.0 }),
            theme::brass(),
        );
    };

    if compact {
        ui.vertical(|ui| {
            ui.set_max_width(72.0);
            paint_tile(ui, 56.0);
            cost_row(ui, &p.requires, None);
        });
        return;
    }

    ui.horizontal(|ui| {
        paint_tile(ui, 88.0);
        ui.vertical(|ui| {
            ui.label(egui::RichText::new(loc.patron_title()).small());
            ui.label(loc.requires());
            cost_row(ui, &p.requires, None);
        });
    });
}

pub fn player_tokens(ui: &mut egui::Ui, tokens: &GemCounts) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        for g in Gem::BASIC {
            let n = tokens.get(g);
            if n > 0 {
                ui.horizontal(|ui| {
                    draw_gem_chip(ui, g, 22.0, None, egui::Sense::hover(), GemChipOpts::default());
                    ui.label(format!("{n}"));
                });
            }
        }
        if tokens.gold > 0 {
            ui.horizontal(|ui| {
                draw_gem_chip(ui, Gem::Gold, 22.0, None, egui::Sense::hover(), GemChipOpts::default());
                ui.label(format!("{}", tokens.gold));
            });
        }
    });
}

pub fn menu_hero(ui: &mut egui::Ui) {
    let time = ui.input(|i| i.time) as f32;
    ui.horizontal_centered(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;
        for (i, g) in Gem::BASIC.iter().enumerate() {
            let bob = cards::menu_gem_phase(i, time) * 4.0;
            ui.allocate_ui_with_layout(
                egui::vec2(40.0, 48.0),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    ui.add_space(2.0 + bob.max(0.0));
                    draw_gem_chip(ui, *g, 36.0, None, egui::Sense::hover(), GemChipOpts::default());
                },
            );
        }
        let bob = cards::menu_gem_phase(5, time) * 4.0;
        ui.allocate_ui_with_layout(
            egui::vec2(40.0, 48.0),
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                ui.add_space(2.0 + bob.max(0.0));
                draw_gem_chip(ui, Gem::Gold, 36.0, None, egui::Sense::hover(), GemChipOpts::default());
            },
        );
    });
}

pub fn mini_card(
    ui: &mut egui::Ui,
    c: &Card,
    loc: &Locale,
    card_w: f32,
    card_h: f32,
) {
    let (rect, resp) = ui.allocate_exact_size(
        egui::vec2(card_w, card_h),
        egui::Sense::hover(),
    );
    let hover = cards::card_hover_anim(ui, resp.id, resp.hovered());
    if ui.is_rect_visible(rect) {
        paint_card_face(
            ui,
            rect,
            c.tier,
            c,
            loc,
            hover,
            c.points as f32,
            0.0,
            true,
            &c.cost,
        );
    }
}
