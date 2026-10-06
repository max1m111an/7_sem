use eframe::egui;

use lab1::app::App;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Игра «Жизнь»")
            .with_inner_size([960.0, 600.0])
            .with_min_inner_size([780.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Игра «Жизнь»",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
