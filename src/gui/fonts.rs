use eframe::egui::{self, FontData, FontDefinitions, FontFamily};
use std::sync::Arc;

const DISPLAY_FONT: &str = "/System/Library/Fonts/NewYork.ttf";

const CJK_FONT_PATHS: &[(&str, &str)] = &[
    (
        "apple_sd_gothic",
        "/System/Library/Fonts/AppleSDGothicNeo.ttc",
    ),
    (
        "hiragino_sans_gb",
        "/System/Library/Fonts/Hiragino Sans GB.ttc",
    ),
    (
        "arial_unicode",
        "/System/Library/Fonts/Supplemental/Arial Unicode.ttf",
    ),
];

pub fn setup(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    let mut cjk: Vec<String> = Vec::new();

    if let Ok(bytes) = std::fs::read(DISPLAY_FONT) {
        fonts.font_data.insert(
            "display_serif".into(),
            Arc::new(FontData::from_owned(bytes)),
        );
        fonts.families.insert(
            FontFamily::Name("display".into()),
            vec!["display_serif".into()],
        );
    }

    for (key, path) in CJK_FONT_PATHS {
        let Ok(bytes) = std::fs::read(path) else {
            continue;
        };
        fonts
            .font_data
            .insert(key.to_string(), Arc::new(FontData::from_owned(bytes)));
        cjk.push(key.to_string());
    }

    if cjk.is_empty() {
        eprintln!("Gemsmith: no CJK fonts found — UI may show □ for Korean/Japanese/Chinese.");
    } else {
        for family in [FontFamily::Proportional, FontFamily::Monospace] {
            if let Some(list) = fonts.families.get_mut(&family) {
                for (i, name) in cjk.iter().enumerate() {
                    list.insert(i, name.clone());
                }
            }
        }
        if let Some(display) = fonts.families.get_mut(&FontFamily::Name("display".into())) {
            for name in cjk.iter().rev() {
                display.push(name.clone());
            }
        }
    }

    ctx.set_fonts(fonts);
}
