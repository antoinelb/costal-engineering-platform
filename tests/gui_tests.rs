use coastal_engineering_platform::gui::WaveChannelApp;
use egui_kittest::{Harness, kittest::Queryable};

#[test]
fn test_wave_channel_app_creation() {
    let _wave_app = WaveChannelApp::new();
    // Test that the WaveChannelApp can be created without panicking
}

#[test]
fn test_wave_channel_app_ui_components() {
    let mut wave_app = WaveChannelApp::new();
    
    let mut harness = Harness::new_ui(move |ui| {
        wave_app.show(ui);
    });

    harness.run();
    
    // Test that the UI contains expected text - get_by_label will panic if not found
    let _content = harness.get_by_label("1D Wave Channel Simulator");
}

#[test]
fn test_wave_channel_app_basic_ui() {
    let mut wave_app = WaveChannelApp::new();
    
    let mut harness = Harness::new_ui(move |ui| {
        wave_app.show(ui);
    });

    harness.run();
    
    // Test that the coming soon text is displayed
    let _coming_soon = harness.get_by_label("Wave simulation module - coming soon");
}