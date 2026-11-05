use super::cards;
use super::theme;
use eframe::egui;
use gemsmith::card::Card;
use gemsmith::locale::Locale;

const DURATION: f64 = 1.35;

pub struct PurchaseCelebration {
    pub card: Card,
    pub tier: u8,
    pub start: f64,
}

impl PurchaseCelebration {
    pub fn progress(&self, now: f64) -> f32 {
        ((now - self.start) / DURATION).clamp(0.0, 1.0) as f32
    }

    pub fn finished(&self, now: f64) -> bool {
        now - self.start >= DURATION
    }
}

fn ease_out_back(t: f32) -> f32 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
}

fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}

pub fn paint(ctx: &egui::Context, fx: &PurchaseCelebration, loc: &Locale) {
    let now = ctx.input(|i| i.time);
    let p = fx.progress(now);
    if p >= 1.0 {
        return;
    }

    let enter = ease_out_back((p / 0.45).min(1.0));
    let exit = if p > 0.72 {
        ease_out_cubic((1.0 - p) / 0.28)
    } else {
        1.0
    };
    let alpha = enter * exit;

    egui::Area::new(egui::Id::new("purchase_celebration"))
        .order(egui::Order::Foreground)
        .anchor(egui::Align2::LEFT_TOP, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            let screen = ui.max_rect();
            ui.set_width(screen.width());
            ui.set_height(screen.height());
            let painter = ui.painter();
            painter.rect_filled(
                screen,
                0.0,
                egui::Color32::BLACK.gamma_multiply(0.55 * alpha),
            );

            let cw = cards::MARKET_CARD_W;
            let ch = cards::MARKET_CARD_H;
            let lift = (1.0 - enter) * 120.0;
            let scale = 0.55 + enter * 0.45;
            let center = screen.center() + egui::vec2(0.0, lift);
            let card_rect = egui::Rect::from_center_size(
                center,
                egui::vec2(cw * scale, ch * scale),
            );

            let glow = theme::accent_gold().gamma_multiply(alpha * (0.35 + (p * 8.0).sin().max(0.0) * 0.15));
            painter.rect_filled(
                card_rect.expand(12.0 * scale),
                14.0,
                glow.gamma_multiply(0.25),
            );

            let time = now as f32;
            let ctx_paint = cards::CardPaintCtx {
                time,
                hover: enter,
                phase: 0.0,
                affordable: 1.0,
                reserved_seal: false,
                loc,
                cost: &fx.card.cost,
            };
            cards::paint_development_card(&painter, card_rect, fx.tier, &fx.card, &ctx_paint);

            let title_t = ((p - 0.25) / 0.35).clamp(0.0, 1.0);
            let title_a = ease_out_cubic(title_t) * alpha;
            if title_a > 0.02 {
                painter.text(
                    center + egui::vec2(0.0, card_rect.height() * 0.5 + 28.0),
                    egui::Align2::CENTER_TOP,
                    loc.purchase_complete(),
                    theme::font_display(32.0),
                    theme::accent_gold().gamma_multiply(title_a),
                );
                painter.text(
                    center + egui::vec2(0.0, card_rect.height() * 0.5 + 62.0),
                    egui::Align2::CENTER_TOP,
                    loc.points_gain(fx.card.points),
                    theme::font_body(16.0),
                    egui::Color32::WHITE.gamma_multiply(title_a * 0.85),
                );
            }

            for i in 0..8 {
                let a = fx.start as f32 + i as f32 * 0.07;
                let spark = ((now as f32 - a) * 4.0).sin() * 0.5 + 0.5;
                let ang = std::f32::consts::TAU * i as f32 / 8.0 + p * 2.0;
                let r = 90.0 * enter + spark * 12.0;
                let pt = center + egui::vec2(ang.cos() * r, ang.sin() * r * 0.6);
                painter.circle_filled(
                    pt,
                    3.0 + spark * 2.0,
                    theme::accent_gold().gamma_multiply(alpha * spark * 0.7),
                );
            }
        });
}
