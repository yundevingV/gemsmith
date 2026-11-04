mod app;
mod card_scenes;
mod cards;
mod draw;
mod fonts;
mod gem_pick;
mod layout;
mod purchase_fx;
mod theme;
mod tokens;

pub fn run() -> eframe::Result<()> {
    eprintln!(
        "Gemsmith: starting window (1180×760). \
         If nothing appears, check Mission Control / other monitor."
    );
    let opts = eframe::NativeOptions {
        centered: true,
        persist_window: false,
        renderer: eframe::Renderer::Glow,
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Gemsmith")
            .with_inner_size([1180.0, 760.0])
            .with_min_inner_size([640.0, 520.0])
            .with_active(true)
            .with_visible(true),
        ..Default::default()
    };
    eframe::run_native(
        "Gemsmith",
        opts,
        Box::new(|cc| Ok(Box::new(app::GemsmithApp::new(cc)))),
    )
}
