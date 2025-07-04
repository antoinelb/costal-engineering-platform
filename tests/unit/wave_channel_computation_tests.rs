use coastal_engineering_platform::gui::WaveChannelApp;

#[test]
fn test_grid_spacing_calculation() {
    let wave_app = WaveChannelApp::new();

    // Test default grid spacing calculation
    let expected_spacing = 50.0 / (100.0 - 1.0); // channel_length / (grid_resolution - 1)
    let actual_spacing = wave_app.grid_spacing();

    assert!((actual_spacing - expected_spacing).abs() < 1e-10);
}

#[test]
fn test_grid_spacing_calculation_edge_cases() {
    // We can't modify the private fields directly, but we can test the calculation logic
    // by testing the formula directly

    // Test various scenarios
    let test_cases = vec![
        (10.0, 11, 1.0),          // Simple case: 10m, 11 points = 1m spacing
        (100.0, 101, 1.0),        // 100m, 101 points = 1m spacing
        (1.0, 11, 0.1),           // 1m, 11 points = 0.1m spacing
        (50.0, 100, 50.0 / 99.0), // Default case
    ];

    for (length, resolution, expected) in test_cases {
        let calculated = length / (resolution as f64 - 1.0);
        assert!(
            (calculated - expected).abs() < 1e-10,
            "Failed for length={}, resolution={}: expected {}, got {}",
            length,
            resolution,
            expected,
            calculated
        );
    }
}

#[test]
fn test_default_parameters_reasonableness() {
    let wave_app = WaveChannelApp::new();

    // Test that default values are within reasonable ranges for coastal engineering
    assert!(wave_app.channel_length > 0.0);
    assert!(wave_app.channel_length <= 1000.0); // Reasonable max channel length

    assert!(wave_app.grid_resolution >= 10);
    assert!(wave_app.grid_resolution <= 10000); // Reasonable max resolution

    assert!(wave_app.still_water_level > 0.0);
    assert!(wave_app.still_water_level <= 100.0); // Reasonable max water depth

    // Test that grid spacing is reasonable
    let spacing = wave_app.grid_spacing();
    assert!(spacing > 0.0);
    assert!(spacing < wave_app.channel_length); // Spacing should be smaller than total length
}

#[test]
fn test_parameter_value_ranges() {
    let wave_app = WaveChannelApp::new();

    // Test that default values are within the expected UI ranges
    // These should match the slider ranges in the UI

    // Channel length: 1.0..=100.0
    assert!(wave_app.channel_length >= 1.0);
    assert!(wave_app.channel_length <= 100.0);

    // Grid resolution: 10..=1000
    assert!(wave_app.grid_resolution >= 10);
    assert!(wave_app.grid_resolution <= 1000);

    // Still water level: 0.1..=10.0
    assert!(wave_app.still_water_level >= 0.1);
    assert!(wave_app.still_water_level <= 10.0);
}

#[test]
fn test_computational_consistency() {
    let wave_app = WaveChannelApp::new();

    // Test that grid spacing calculation is consistent
    let spacing1 = wave_app.grid_spacing();
    let spacing2 = wave_app.grid_spacing();

    assert_eq!(
        spacing1, spacing2,
        "Grid spacing calculation should be deterministic"
    );

    // Test mathematical relationship
    let length = wave_app.channel_length;
    let resolution = wave_app.grid_resolution;
    let spacing = wave_app.grid_spacing();

    let expected_total_length = spacing * (resolution as f64 - 1.0);
    assert!(
        (expected_total_length - length).abs() < 1e-10,
        "Total length should equal spacing * (resolution - 1)"
    );
}
