mod app;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0])
            .with_clear_color(egui::Color32::from_rgb(10, 10, 10)),
        ..Default::default()
    };

    eframe::run_native(
        "Shady_LLM",
        options,
        Box::new(|cc| Box::new(app::ShadyApp::new(cc))),
    )
}