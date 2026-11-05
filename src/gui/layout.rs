use super::cards;
use eframe::egui;

#[derive(Clone, Copy)]
pub struct CardMetrics {
    pub market_w: f32,
    pub market_h: f32,
    pub mini_w: f32,
    pub mini_h: f32,
}

pub struct PlayLayout {
    pub narrow: bool,
    pub short: bool,
    pub content_w: f32,
    pub market_w: f32,
    pub side_w: f32,
    pub cards: CardMetrics,
    pub bank_gem: f32,
    pub bank_row_h: f32,
    pub tier_spacing: f32,
    pub hide_take_help: bool,
    pub log_scroll_h: f32,
}

const COL_GAP: f32 = 10.0;
const TIER_BTN_ROW: f32 = 34.0;

impl PlayLayout {
    pub fn measure(ui: &egui::Ui) -> Self {
        let content_w = ui.available_width();
        let content_h = ui.available_height().max(480.0);
        let narrow = content_w < 720.0;
        let short = content_h < 760.0;

        let (market_w, side_w) = if narrow {
            (content_w, content_w)
        } else {
            let side = content_w / 3.0;
            let market = content_w * 2.0 / 3.0 - COL_GAP;
            (market.max(320.0), side.max(240.0))
        };

        let tier_spacing = if short { 5.0 } else { 10.0 };
        let w_scale = ((market_w - 54.0) / (3.0 * cards::MARKET_CARD_W + 32.0)).max(0.72);
        let tier_target_h = if short {
            ((content_h - 90.0) / 3.25).max(cards::MARKET_CARD_H * 0.78)
        } else {
            cards::MARKET_CARD_H + 44.0
        };
        let h_scale = (tier_target_h - TIER_BTN_ROW) / cards::MARKET_CARD_H;
        let scale = w_scale.min(h_scale).clamp(0.78, 1.15);

        let cards = CardMetrics {
            market_w: cards::MARKET_CARD_W * scale,
            market_h: cards::MARKET_CARD_H * scale,
            mini_w: cards::MINI_CARD_W * scale,
            mini_h: cards::MINI_CARD_H * scale,
        };

        Self {
            narrow,
            short,
            content_w,
            market_w,
            side_w,
            bank_gem: 40.0,
            bank_row_h: 68.0,
            tier_spacing,
            hide_take_help: short,
            log_scroll_h: if short { 140.0 } else { 200.0 },
            cards,
        }
    }
}

pub fn tier_row_needs_scroll(available: f32, card_w: f32) -> bool {
    available < card_w * 3.0 + 48.0
}
