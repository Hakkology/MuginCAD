mod commands;
mod model;
mod view;
mod viewmodel;

use eframe::egui;
use view::CadApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("OliveCAD - The Modern Rust CAD"),
        ..Default::default()
    };

    eframe::run_native(
        "rust_cad",
        native_options,
        Box::new(|cc| Ok(Box::new(CadApp::new(cc)))),
    )
}
