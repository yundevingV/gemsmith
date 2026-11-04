use eframe::egui;
use gemsmith::gem::Gem;

pub fn hex(hex: &str) -> egui::Color32 {
    let h = hex.trim_start_matches('#');
    let v = u32::from_str_radix(h, 16).unwrap_or(0);
    egui::Color32::from_rgb(((v >> 16) & 0xFF) as u8, ((v >> 8) & 0xFF) as u8, (v & 0xFF) as u8)
}

pub mod bg {
    use super::hex;
    use eframe::egui::Color32;

    pub fn base_dark() -> Color32 {
        hex("#0B131F")
    }
    pub fn panel_emerald() -> Color32 {
        hex("#0D221E")
    }
    pub fn surface_velvet() -> Color32 {
        hex("#161B26")
    }
    pub fn card_face() -> Color32 {
        hex("#2A241C")
    }
    pub fn card_mat() -> Color32 {
        hex("#141820")
    }
}

pub mod border {
    use super::hex;
    use eframe::egui::Color32;

    pub fn brass() -> Color32 {
        hex("#B8860B")
    }
    pub fn gold_accent() -> Color32 {
        hex("#F5D061")
    }
}

pub fn gem_primary(g: Gem) -> egui::Color32 {
    match g {
        Gem::Ruby => hex("#E63946"),
        Gem::Sapphire => hex("#1D3557"),
        Gem::Emerald => hex("#2A9D8F"),
        Gem::Diamond => hex("#E0E1DD"),
        Gem::Onyx => hex("#2B2D42"),
        Gem::Gold => hex("#E9C46A"),
    }
}

pub fn gem_gradient(g: Gem) -> (egui::Color32, egui::Color32) {
    match g {
        Gem::Ruby => (hex("#FF6B6B"), hex("#990012")),
        Gem::Sapphire => (hex("#457B9D"), hex("#0A192F")),
        Gem::Emerald => (hex("#52B788"), hex("#134035")),
        Gem::Diamond => (hex("#FFFFFF"), hex("#9A8C98")),
        Gem::Onyx => (hex("#4A4E69"), hex("#12131C")),
        Gem::Gold => (hex("#FFE6A7"), hex("#B58A29")),
    }
}

pub fn gem_glow_alpha(g: Gem) -> f32 {
    match g {
        Gem::Ruby => 0.5,
        Gem::Sapphire => 0.5,
        Gem::Emerald => 0.5,
        Gem::Diamond => 0.6,
        Gem::Onyx => 0.4,
        Gem::Gold => 0.7,
    }
}
