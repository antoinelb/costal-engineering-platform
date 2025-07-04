mod gui;

use eframe::egui;
use gui::PlatformApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Coastal Engineering Platform"),
        ..Default::default()
    };

    eframe::run_native(
        "Coastal Engineering Platform",
        options,
        Box::new(|cc| Ok(Box::new(PlatformApp::new(cc)))),
    )
}
