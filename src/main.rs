mod app;
pub mod core;
pub mod ui;

use eframe::egui;
use crate::core::config::{WINDOW_HEIGHT, WINDOW_WIDTH};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
        ..Default::default()
    };

    eframe::run_native(
        "Shady_LLM",
        options,
        Box::new(|cc| Box::new(app::ShadyApp::new(cc))),
    )
}