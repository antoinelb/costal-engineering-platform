// Legacy GUI tests - moved to organized structure
// See tests/unit/ and tests/integration/ for current tests

use coastal_engineering_platform::gui::{WaveChannelApp, EquationRenderer};
use egui_kittest::{Harness, kittest::Queryable};

#[test]
fn test_wave_channel_app_creation_legacy() {
    let _wave_app = WaveChannelApp::new();
    // Test that the WaveChannelApp can be created without panicking
}

#[test]
fn test_wave_channel_app_ui_components_legacy() {
    let mut wave_app = WaveChannelApp::new();

    let mut harness = Harness::new_ui(move |ui| {
        let mut equation_renderer = EquationRenderer::new();
        let ctx = ui.ctx().clone();
        wave_app.show(ui, &ctx, &mut equation_renderer);
    });

    harness.run();

    // Test that the UI contains expected text - get_by_label will panic if not found
    let _content = harness.get_by_label("1D Wave Channel Simulator");
}

#[test]
fn test_wave_channel_app_parameter_ui() {
    let mut wave_app = WaveChannelApp::new();

    let mut harness = Harness::new_ui(move |ui| {
        let mut equation_renderer = EquationRenderer::new();
        let ctx = ui.ctx().clone();
        wave_app.show(ui, &ctx, &mut equation_renderer);
    });

    harness.run();

    // Test that parameter controls are present
    let _channel_params = harness.get_by_label("Channel Parameters");
    let _computed_values = harness.get_by_label("Computed Values");
}
