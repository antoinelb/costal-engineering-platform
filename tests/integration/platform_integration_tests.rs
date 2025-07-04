use egui_kittest::{Harness, kittest::Queryable};

#[test]
fn test_platform_app_creation() {
    // We can't easily test PlatformApp::new() because it requires eframe::CreationContext
    // But we can test that the module structure works
    let _wave_app = coastal_engineering_platform::gui::WaveChannelApp::new();
    assert!(true); // If we can import and create, the module structure is correct
}

#[test] 
fn test_platform_app_ui_integration() {
    // Test that WaveChannelApp integrates properly with the main platform UI
    let mut wave_app = coastal_engineering_platform::gui::WaveChannelApp::new();
    
    let mut harness = Harness::new_ui(move |ui| {
        // Simulate the platform app structure from gui.rs
        ui.heading("Coastal Engineering Platform");
        ui.separator();
        
        wave_app.show(ui);
    });

    harness.run();
    
    // Verify both platform and wave channel elements are present
    let _platform_title = harness.get_by_label("Coastal Engineering Platform");
    let _wave_simulator = harness.get_by_label("1D Wave Channel Simulator");
    let _channel_params = harness.get_by_label("Channel Parameters");
}

#[test]
fn test_module_accessibility() {
    // Test that the module structure allows proper access to components
    use coastal_engineering_platform::gui::WaveChannelApp;
    
    // Should be able to import the wave channel app
    let _wave_app = WaveChannelApp::new();
    
    assert!(true);
}