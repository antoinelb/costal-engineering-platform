use coastal_engineering_platform::gui::WaveChannelApp;

#[test]
fn test_direct_field_access() {
    let mut wave_app = WaveChannelApp::new();

    // Test that we can directly access and modify fields
    assert_eq!(wave_app.channel_length, 50.0);
    assert_eq!(wave_app.grid_resolution, 100);
    assert_eq!(wave_app.still_water_level, 2.0);

    // Test field modification
    wave_app.channel_length = 25.0;
    wave_app.grid_resolution = 50;
    wave_app.still_water_level = 1.5;

    assert_eq!(wave_app.channel_length, 25.0);
    assert_eq!(wave_app.grid_resolution, 50);
    assert_eq!(wave_app.still_water_level, 1.5);
}

#[test]
fn test_grid_spacing_updates_with_parameters() {
    let mut wave_app = WaveChannelApp::new();

    // Test initial grid spacing
    let initial_spacing = wave_app.grid_spacing();
    assert_eq!(initial_spacing, 50.0 / 99.0); // 50m / (100-1) points

    // Change channel length and verify grid spacing updates
    wave_app.channel_length = 100.0;
    let new_spacing = wave_app.grid_spacing();
    assert_eq!(new_spacing, 100.0 / 99.0);

    // Change grid resolution and verify grid spacing updates
    wave_app.grid_resolution = 51;
    let newer_spacing = wave_app.grid_spacing();
    assert_eq!(newer_spacing, 100.0 / 50.0); // 100m / (51-1) points = 2.0
}

#[test]
fn test_parameter_edge_cases() {
    let mut wave_app = WaveChannelApp::new();

    // Test minimum values (matching UI ranges)
    wave_app.channel_length = 1.0;
    wave_app.grid_resolution = 10;
    wave_app.still_water_level = 0.1;

    let spacing = wave_app.grid_spacing();
    assert_eq!(spacing, 1.0 / 9.0); // 1m / (10-1) points
    assert!(spacing > 0.0);

    // Test maximum values (matching UI ranges)
    wave_app.channel_length = 100.0;
    wave_app.grid_resolution = 1000;
    wave_app.still_water_level = 10.0;

    let max_spacing = wave_app.grid_spacing();
    assert_eq!(max_spacing, 100.0 / 999.0);
    assert!(max_spacing > 0.0);
}

#[test]
fn test_computational_precision() {
    let mut wave_app = WaveChannelApp::new();

    // Test with precise values that should give exact results
    wave_app.channel_length = 10.0;
    wave_app.grid_resolution = 11; // Should give exactly 1.0m spacing

    let spacing = wave_app.grid_spacing();
    assert_eq!(spacing, 1.0);

    // Test another precise case
    wave_app.channel_length = 20.0;
    wave_app.grid_resolution = 21; // Should give exactly 1.0m spacing

    let spacing2 = wave_app.grid_spacing();
    assert_eq!(spacing2, 1.0);
}

#[test]
fn test_parameter_independence() {
    let mut wave_app = WaveChannelApp::new();

    // Test that modifying one parameter doesn't affect others
    let original_resolution = wave_app.grid_resolution;
    let original_water_level = wave_app.still_water_level;

    wave_app.channel_length = 75.0;

    assert_eq!(wave_app.grid_resolution, original_resolution);
    assert_eq!(wave_app.still_water_level, original_water_level);
    assert_eq!(wave_app.channel_length, 75.0);
}

#[test]
fn test_struct_field_types() {
    let wave_app = WaveChannelApp::new();

    // Test that field types are correct
    let _length: f64 = wave_app.channel_length;
    let _resolution: usize = wave_app.grid_resolution;
    let _water_level: f64 = wave_app.still_water_level;

    // Test that grid_spacing returns f64
    let _spacing: f64 = wave_app.grid_spacing();

    assert!(true); // If compilation succeeds, types are correct
}
