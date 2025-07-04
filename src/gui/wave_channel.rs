use eframe::egui;

pub struct WaveChannelApp {
    pub channel_length: f64,
    pub grid_resolution: usize,
    pub still_water_level: f64,
}

impl WaveChannelApp {
    pub fn new() -> Self {
        Self {
            channel_length: 50.0,   // Default 50m channel
            grid_resolution: 100,   // Default 100 grid points
            still_water_level: 2.0, // Default 2m water depth
        }
    }

    pub fn grid_spacing(&self) -> f64 {
        self.channel_length / (self.grid_resolution as f64 - 1.0)
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("1D Wave Channel Simulator");
        ui.separator();

        // Channel parameters section
        ui.heading("Channel Parameters");

        // Channel length control
        ui.horizontal(|ui| {
            ui.label("Channel Length:");
            ui.add(
                egui::Slider::new(&mut self.channel_length, 1.0..=100.0)
                    .suffix(" m")
                    .step_by(0.1),
            );
        });

        // Grid resolution control
        ui.horizontal(|ui| {
            ui.label("Grid Resolution:");
            ui.add(egui::Slider::new(&mut self.grid_resolution, 10..=1000).suffix(" points"));
        });

        // Still water level control
        ui.horizontal(|ui| {
            ui.label("Still Water Level:");
            ui.add(
                egui::Slider::new(&mut self.still_water_level, 0.1..=10.0)
                    .suffix(" m")
                    .step_by(0.01),
            );
        });

        ui.separator();

        // Computed values section
        ui.heading("Computed Values");
        // let grid_spacing = self.channel_length / (self.grid_resolution as f64 - 1.0);
        ui.label(format!("Grid Spacing (Δx): {:.3} m", self.grid_spacing()));

        ui.separator();
        ui.label("Simulation controls coming soon...");
    }
}

