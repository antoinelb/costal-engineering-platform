use coastal_engineering_platform::gui::WaveChannelApp;
use egui_kittest::{Harness, kittest::Queryable};

#[test]
fn test_platform_app_integration() {
    let mut wave_app = WaveChannelApp::new();
    
    let mut harness = Harness::new_ui(move |ui| {
        // Simulate the platform app structure
        ui.heading("Coastal Engineering Platform");
        ui.separator();
        
        wave_app.show(ui);
    });

    harness.run();
    
    // Verify key UI elements are present - get_by_label will panic if not found
    let _platform_title = harness.get_by_label("Coastal Engineering Platform");
    let _wave_simulator = harness.get_by_label("1D Wave Channel Simulator");
}