use eframe::egui;

use chess_final::ui::ChessApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Rust Chess")
            .with_inner_size([900.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Chess",
        options,
        Box::new(|cc| Ok(Box::new(ChessApp::new(cc)))),
    )
}
