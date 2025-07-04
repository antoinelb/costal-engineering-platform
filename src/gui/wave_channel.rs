use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};

pub struct WaveChannelApp {
    pub channel_length: f64,
    pub grid_resolution: usize,
    pub still_water_level: f64,
    pub surface_elevation: Vec<f64>, // Water surface elevation (for future wave animation)
}

impl WaveChannelApp {
    pub fn new() -> Self {
        let grid_resolution = 100;
        Self {
            channel_length: 50.0,                          // Default 50m channel
            grid_resolution,                               // Default 100 grid points
            still_water_level: 2.0,                        // Default 2m water depth
            surface_elevation: vec![0.0; grid_resolution], // Initialize with still water
        }
    }

    pub fn grid_spacing(&self) -> f64 {
        self.channel_length / (self.grid_resolution as f64 - 1.0)
    }

    fn update_surface_elevation(&mut self) {
        // Resize surface elevation vector if grid resolution changed
        if self.surface_elevation.len() != self.grid_resolution {
            self.surface_elevation.resize(self.grid_resolution, 0.0);
        }
    }

    fn generate_plot_data(&self) -> (PlotPoints, PlotPoints, PlotPoints) {
        let x_positions: Vec<f64> = (0..self.grid_resolution)
            .map(|i| i as f64 * self.grid_spacing())
            .collect();

        // Water surface (still water level + surface elevation)
        let water_surface: PlotPoints = x_positions
            .iter()
            .zip(self.surface_elevation.iter())
            .map(|(&x, &eta)| [x, self.still_water_level + eta])
            .collect();

        // Channel bottom (flat bottom at depth 0)
        let channel_bottom: PlotPoints = x_positions.iter().map(|&x| [x, 0.0]).collect();

        // Channel sides (vertical walls at start and end)
        let channel_walls: PlotPoints = vec![
            [0.0, 0.0],
            [0.0, self.still_water_level + 1.0],
            [self.channel_length, self.still_water_level + 1.0],
            [self.channel_length, 0.0],
        ]
        .into();

        (water_surface, channel_bottom, channel_walls)
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("1D Wave Channel Simulator");
        ui.separator();

        // Store previous values to detect changes
        let prev_grid_resolution = self.grid_resolution;

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

        // Update surface elevation if grid resolution changed
        if prev_grid_resolution != self.grid_resolution {
            self.update_surface_elevation();
        }

        ui.separator();

        // Computed values section
        ui.heading("Computed Values");
        ui.label(format!("Grid Spacing (Δx): {:.3} m", self.grid_spacing()));

        ui.separator();

        // Wave channel visualization
        ui.heading("Channel Visualization");

        let (water_surface, channel_bottom, _channel_walls) = self.generate_plot_data();

        Plot::new("wave_channel")
            .height(350.0)
            .width(850.0)
            .view_aspect(2.0)
            .clamp_grid(true)
            .allow_zoom(true)
            .allow_drag(true)
            .allow_scroll(true)
            .allow_boxed_zoom(true)
            .set_margin_fraction([0.0, 0.2].into())
            .x_axis_label("Distance (m)")
            .y_axis_label("Elevation (m)")
            .include_x(0)
            .include_x(self.channel_length)
            .include_y(0)
            .include_y(self.still_water_level)
            .auto_bounds([false, true])
            .show(ui, |plot_ui| {
                // Channel bottom (seabed)
                plot_ui.line(
                    Line::new(channel_bottom)
                        .color(egui::Color32::from_rgb(139, 69, 19)) // Brown for seabed
                        .width(3.0)
                        .name("Channel Bottom"),
                );

                // Water surface
                plot_ui.line(
                    Line::new(water_surface)
                        .color(egui::Color32::from_rgb(30, 144, 255)) // Dodger blue for water
                        .width(2.0)
                        .name("Water Surface"),
                );
            });
    }
}
