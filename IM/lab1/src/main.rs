mod app;
mod config;
mod sim;
mod ui;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "КА Фито–Зоо", // Переведённый заголовок окна
        options,
        Box::new(|cc| Ok(Box::new(app::PhytoZooApp::new(cc)))),
    )
}
