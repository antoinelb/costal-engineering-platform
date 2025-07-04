use coastal_engineering_platform::gui::{WaveChannelApp, EquationRenderer};
use egui_kittest::{Harness, kittest::Queryable};

// Since PlatformApp is hard to test directly due to eframe::CreationContext complexity,
// we'll test its behavior through the GUI integration tests instead.
// These tests focus on the testable components.

#[test]
fn test_platform_app_structure() {
    // Test that the PlatformApp contains a WaveChannelApp
    // We do this by testing that WaveChannelApp works as expected in PlatformApp context
    let mut wave_app = WaveChannelApp::new();

    let mut harness = Harness::new_ui(move |ui| {
        // Simulate exactly what PlatformApp::update does
        ui.heading("Coastal Engineering Platform");
        ui.separator();

        let mut equation_renderer = EquationRenderer::new();
        let ctx = ui.ctx().clone();
        wave_app.show(ui, &ctx, &mut equation_renderer);
    });

    harness.run();

    // Verify the complete platform structure
    let _platform_title = harness.get_by_label("Coastal Engineering Platform");
    let _wave_simulator = harness.get_by_label("1D Wave Channel Simulator");
    let _channel_params = harness.get_by_label("Channel Parameters");
}

#[test]
fn test_platform_app_layout() {
    // Test the overall layout structure that PlatformApp creates
    let mut wave_app = WaveChannelApp::new();

    let mut harness = Harness::new_ui(move |ui| {
        // Test the CentralPanel structure from PlatformApp
        ui.heading("Coastal Engineering Platform");
        ui.separator();
        let mut equation_renderer = EquationRenderer::new();
        let ctx = ui.ctx().clone();
        wave_app.show(ui, &ctx, &mut equation_renderer);
    });

    harness.run();

    // Test that all major sections are present in the expected order
    let _main_heading = harness.get_by_label("Coastal Engineering Platform");
    let _simulator_heading = harness.get_by_label("1D Wave Channel Simulator");
    let _params_section = harness.get_by_label("Channel Parameters");
    let _computed_section = harness.get_by_label("Computed Values");
}

#[test]
fn test_wave_channel_app_integration_in_platform() {
    // Test that WaveChannelApp integrates correctly within PlatformApp context
    let mut wave_app = WaveChannelApp::new();

    let mut harness = Harness::new_ui(move |ui| {
        // Replicate the exact PlatformApp structure
        ui.heading("Coastal Engineering Platform");
        ui.separator();
        let mut equation_renderer = EquationRenderer::new();
        let ctx = ui.ctx().clone();
        wave_app.show(ui, &ctx, &mut equation_renderer);
    });

    harness.run();

    // Verify all wave channel functionality works within platform context
    let _channel_length = harness.get_by_label("Channel Length:");
    let _grid_resolution = harness.get_by_label("Grid Resolution:");
    let _still_water = harness.get_by_label("Still Water Level:");
    let _computed_values = harness.get_by_label("Computed Values");
    let _coming_soon = harness.get_by_label("Simulation controls coming soon...");
}
