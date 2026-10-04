mod app;
mod config;
mod sim;
mod ui;

use app::App;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Planetary Colonies CA",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
