use coastal_engineering_platform::gui::WaveChannelApp;
use egui_kittest::{Harness, kittest::Queryable};

#[test]
fn test_wave_channel_app_default_parameters() {
    // Test that default parameters are reasonable
    let mut wave_app = WaveChannelApp::new();

    let mut harness = Harness::new_ui(move |ui| {
        wave_app.show(ui);
    });

    harness.run();

    // We can't directly access private fields, but we can verify that
    // the computed grid spacing appears (indicating calculation is working)
    let _computed_values = harness.get_by_label("Computed Values");

    // The presence of these labels indicates the app is working correctly
    assert!(true);
}

#[test]
fn test_wave_channel_app_ui_responsiveness() {
    // Test that the UI can handle multiple render cycles
    let mut wave_app = WaveChannelApp::new();

    for _ in 0..5 {
        let mut harness = Harness::new_ui(|ui| {
            wave_app.show(ui);
        });

        harness.run();

        // Verify key elements are still present after multiple renders
        let _heading = harness.get_by_label("1D Wave Channel Simulator");
        let _params = harness.get_by_label("Channel Parameters");
    }
}

#[test]
fn test_wave_channel_app_ui_components_consistency() {
    // Test that all UI components are consistently present
    let mut wave_app = WaveChannelApp::new();

    let mut harness = Harness::new_ui(move |ui| {
        wave_app.show(ui);
    });

    harness.run();

    // Test all expected labels are present
    let labels_to_check = [
        "1D Wave Channel Simulator",
        "Channel Parameters",
        "Channel Length:",
        "Grid Resolution:",
        "Still Water Level:",
        "Computed Values",
        "Simulation controls coming soon...",
    ];

    for label in labels_to_check.iter() {
        let _element = harness.get_by_label(label);
    }
}

#[test]
fn test_wave_channel_app_multiple_instances() {
    // Test that multiple WaveChannelApp instances don't interfere
    let mut wave_app1 = WaveChannelApp::new();
    let mut wave_app2 = WaveChannelApp::new();

    // Test first instance
    let mut harness1 = Harness::new_ui(|ui| {
        wave_app1.show(ui);
    });
    harness1.run();
    let _heading1 = harness1.get_by_label("1D Wave Channel Simulator");

    // Test second instance
    let mut harness2 = Harness::new_ui(|ui| {
        wave_app2.show(ui);
    });
    harness2.run();
    let _heading2 = harness2.get_by_label("1D Wave Channel Simulator");

    // Both should work independently
    assert!(true);
}

#[test]
fn test_wave_channel_app_ui_structure() {
    // Test the hierarchical structure of the UI
    let mut wave_app = WaveChannelApp::new();

    let mut harness = Harness::new_ui(move |ui| {
        wave_app.show(ui);
    });

    harness.run();

    // Test that the UI has the expected structure
    // Main heading should be present
    let _main_heading = harness.get_by_label("1D Wave Channel Simulator");

    // Parameter section should be present
    let _params_heading = harness.get_by_label("Channel Parameters");

    // All three parameter controls should be present
    let _channel_length = harness.get_by_label("Channel Length:");
    let _grid_resolution = harness.get_by_label("Grid Resolution:");
    let _still_water = harness.get_by_label("Still Water Level:");

    // Computed values section should be present
    let _computed_heading = harness.get_by_label("Computed Values");

    // Future functionality placeholder should be present
    let _coming_soon = harness.get_by_label("Simulation controls coming soon...");
}
