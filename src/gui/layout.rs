use eframe::egui;

pub struct PlayLayout {
    pub narrow: bool,
    pub content_w: f32,
    pub main_w: f32,
    pub side_w: f32,
}

impl PlayLayout {
    pub fn measure(ui: &egui::Ui) -> Self {
        let content_w = ui.available_width();
        let narrow = content_w < 920.0;
        if narrow {
            Self {
                narrow: true,
                content_w,
                main_w: content_w,
                side_w: content_w,
            }
        } else {
            let side_w = (content_w * 0.30).clamp(280.0, 380.0);
            let main_w = content_w - side_w - 12.0;
            Self {
                narrow: false,
                content_w,
                main_w,
                side_w,
            }
        }
    }
}

pub fn tier_row_needs_scroll(available: f32) -> bool {
    available < 360.0
}
