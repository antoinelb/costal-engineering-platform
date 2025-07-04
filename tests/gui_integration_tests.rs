// Legacy integration tests - moved to organized structure
// See tests/integration/ for current integration tests

use coastal_engineering_platform::gui::{WaveChannelApp, EquationRenderer};
use egui_kittest::{Harness, kittest::Queryable};

#[test]
fn test_platform_app_integration_legacy() {
    let mut wave_app = WaveChannelApp::new();

    let mut harness = Harness::new_ui(move |ui| {
        // Simulate the platform app structure
        ui.heading("Coastal Engineering Platform");
        ui.separator();

        let mut equation_renderer = EquationRenderer::new();
        let ctx = ui.ctx().clone();
        wave_app.show(ui, &ctx, &mut equation_renderer);
    });

    harness.run();

    // Verify key UI elements are present - get_by_label will panic if not found
    let _platform_title = harness.get_by_label("Coastal Engineering Platform");
    let _wave_simulator = harness.get_by_label("1D Wave Channel Simulator");
}
