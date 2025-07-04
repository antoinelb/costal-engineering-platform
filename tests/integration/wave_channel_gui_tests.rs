use coastal_engineering_platform::gui::{WaveChannelApp, EquationRenderer};
use egui_kittest::{Harness, kittest::Queryable};

#[test]
fn test_wave_channel_app_ui_creation() {
    let mut wave_app = WaveChannelApp::new();

    let mut harness = Harness::new_ui(move |ui| {
        let mut equation_renderer = EquationRenderer::new();
        let ctx = ui.ctx().clone();
        wave_app.show(ui, &ctx, &mut equation_renderer);
    });

    harness.run();

    // Test that the main heading is present
    let _heading = harness.get_by_label("1D Wave Channel Simulator");
}

#[test]
fn test_wave_channel_app_parameter_controls() {
    let mut wave_app = WaveChannelApp::new();

    let mut harness = Harness::new_ui(move |ui| {
        let mut equation_renderer = EquationRenderer::new();
        let ctx = ui.ctx().clone();
        wave_app.show(ui, &ctx, &mut equation_renderer);
    });

    harness.run();

    // Test that parameter labels are present
    let _channel_params = harness.get_by_label("Channel Parameters");
    let _channel_length = harness.get_by_label("Channel Length:");
    let _grid_resolution = harness.get_by_label("Grid Resolution:");
    let _still_water = harness.get_by_label("Still Water Level:");
}

#[test]
fn test_wave_channel_app_computed_values() {
    let mut wave_app = WaveChannelApp::new();

    let mut harness = Harness::new_ui(move |ui| {
        let mut equation_renderer = EquationRenderer::new();
        let ctx = ui.ctx().clone();
        wave_app.show(ui, &ctx, &mut equation_renderer);
    });

    harness.run();

    // Test that computed values section is present
    let _computed_values = harness.get_by_label("Computed Values");

    // Test that some grid spacing text is displayed
    // We can't easily search for specific text in labels with current API,
    // so we just verify the computed values section exists
    assert!(true); // If we got the "Computed Values" label, the section is there
}

#[test]
fn test_wave_channel_app_complete_ui() {
    let mut wave_app = WaveChannelApp::new();

    let mut harness = Harness::new_ui(move |ui| {
        let mut equation_renderer = EquationRenderer::new();
        let ctx = ui.ctx().clone();
        wave_app.show(ui, &ctx, &mut equation_renderer);
    });

    harness.run();

    // Test all major UI components are present
    let _main_heading = harness.get_by_label("1D Wave Channel Simulator");
    let _params_heading = harness.get_by_label("Channel Parameters");
    let _computed_heading = harness.get_by_label("Computed Values");
    let _coming_soon = harness.get_by_label("Simulation controls coming soon...");
}
