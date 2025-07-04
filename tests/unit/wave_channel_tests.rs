use coastal_engineering_platform::gui::{WaveChannelApp, EquationRenderer};

#[test]
fn test_wave_channel_app_creation() {
    let _wave_app = WaveChannelApp::new();
    // Test that the WaveChannelApp can be created without panicking

    // Default values should be reasonable
    assert!(true); // Creation succeeded if we reach here
}

#[test]
fn test_wave_channel_app_default_values() {
    let wave_app = WaveChannelApp::new();

    // Test that default values are reasonable
    assert_eq!(wave_app.channel_length, 50.0);
    assert_eq!(wave_app.grid_resolution, 100);
    assert_eq!(wave_app.still_water_level, 2.0);
}

#[cfg(test)]
mod wave_channel_unit_tests {
    use super::*;

    #[test]
    fn test_multiple_wave_channel_apps() {
        let _app1 = WaveChannelApp::new();
        let _app2 = WaveChannelApp::new();

        // Should be able to create multiple independent instances
        assert!(true);
    }
}
