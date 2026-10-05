use eframe::egui;

use lab1::app::App;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Planetary Colonies CA")
            .with_inner_size([1600.0, 1000.0])
            .with_min_inner_size([1100.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Planetary Colonies CA",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
