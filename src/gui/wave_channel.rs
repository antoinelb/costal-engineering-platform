use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use super::equations::EquationRenderer;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WaterDepthRegime {
    Shallow,
    Intermediate,
    Deep,
}

pub struct WaveChannelApp {
    pub channel_length: f64,
    pub grid_resolution: usize,
    pub still_water_level: f64,
    pub surface_elevation: Vec<f64>, // Water surface elevation (for future wave animation)
    pub wave_height: f64,            // Wave height (H)
    pub wave_period: f64,            // Wave period (T)
    pub number_of_waves: usize,      // Number of waves to generate
}

impl Default for WaveChannelApp {
    fn default() -> Self {
        Self::new()
    }
}

impl WaveChannelApp {
    pub fn new() -> Self {
        let grid_resolution = 100;
        Self {
            channel_length: 50.0,                          // Default 50m channel
            grid_resolution,                               // Default 100 grid points
            still_water_level: 2.0,                        // Default 2m water depth
            surface_elevation: vec![0.0; grid_resolution], // Initialize with still water
            wave_height: 0.5,                              // Default 0.5m wave height
            wave_period: 4.0,                              // Default 4s wave period
            number_of_waves: 50,                           // Default 50 waves
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

    fn info_button(ui: &mut egui::Ui, tooltip_text: &str) {
        ui.add_space(5.0);
        if ui.small_button("?").on_hover_text(tooltip_text).clicked() {
            // Button click handled by hover tooltip
        }
    }

    fn equation_info_button(ui: &mut egui::Ui, ctx: &egui::Context, equation_renderer: &mut EquationRenderer, equation_id: &str, tooltip_text: &str) {
        equation_renderer.equation_tooltip_button(ctx, ui, equation_id, tooltip_text);
    }

    fn classify_water_depth(h: f64, wavelength: f64) -> WaterDepthRegime {
        let ratio = h / wavelength;
        if ratio < 1.0 / 20.0 {
            WaterDepthRegime::Shallow
        } else if ratio > 0.5 {
            WaterDepthRegime::Deep
        } else {
            WaterDepthRegime::Intermediate
        }
    }

    fn calculate_wavelength_adaptive(period: f64, depth: f64, gravity: f64) -> f64 {
        // Start with shallow water approximation
        let wavelength = period * (gravity * depth).sqrt();
        
        // Check regime and refine calculation
        let regime = Self::classify_water_depth(depth, wavelength);
        
        match regime {
            WaterDepthRegime::Shallow => {
                // Already calculated correctly
                wavelength
            }
            WaterDepthRegime::Deep => {
                // Deep water formula: L = gT²/(2π)
                gravity * period * period / (2.0 * std::f64::consts::PI)
            }
            WaterDepthRegime::Intermediate => {
                // Iterative solution of dispersion relation
                // L = (gT²/(2π)) * tanh(2πh/L)
                let mut l_new = gravity * period * period / (2.0 * std::f64::consts::PI); // Deep water guess
                
                for _ in 0..20 { // Max 20 iterations
                    let l_old = l_new;
                    let k = 2.0 * std::f64::consts::PI / l_old;
                    let tanh_kh = (k * depth).tanh();
                    l_new = (gravity * period * period / (2.0 * std::f64::consts::PI)) * tanh_kh;
                    
                    // Check convergence
                    if (l_new - l_old).abs() < 1e-6 {
                        break;
                    }
                }
                
                l_new
            }
        }
    }

    fn calculate_celerity_adaptive(period: f64, depth: f64, gravity: f64) -> f64 {
        let wavelength = Self::calculate_wavelength_adaptive(period, depth, gravity);
        wavelength / period
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

    pub fn show(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, equation_renderer: &mut EquationRenderer) {
        // Use full width available
        ui.allocate_ui_with_layout(
            [ui.available_width(), 0.0].into(),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                ui.heading("1D Wave Channel Simulator");
                ui.separator();

                // Store previous values to detect changes
                let prev_grid_resolution = self.grid_resolution;

                // Channel parameters section
                ui.heading("Channel Parameters");

                // Channel length control
                ui.horizontal(|ui| {
                    ui.label("Channel Length:");
                    Self::info_button(ui, "The total length of the wave channel domain. Longer channels allow waves to develop fully and reduce boundary effects. Typical values: 50-200m for coastal studies.");
                    ui.add(
                        egui::Slider::new(&mut self.channel_length, 1.0..=200.0)
                            .suffix(" m")
                            .step_by(0.1),
                    );
                });

                // Grid resolution control
                ui.horizontal(|ui| {
                    ui.label("Grid Resolution:");
                    Self::info_button(ui, "Number of computational grid points along the channel. Higher resolution gives better accuracy but increases computation time. Rule of thumb: 20-50 points per wavelength for good accuracy.");
                    ui.add(
                        egui::Slider::new(&mut self.grid_resolution, 10..=2000).suffix(" points"),
                    );
                });

                // Still water level control
                ui.horizontal(|ui| {
                    ui.label("Still Water Level:");
                    Self::info_button(ui, "Mean water depth (h) in the channel. Controls wave speed and breaking characteristics. Shallow water: h < L/20, Deep water: h > L/2, where L is wavelength. Typical coastal depths: 0.5-5m.");
                    ui.add(
                        egui::Slider::new(&mut self.still_water_level, 0.1..=5.0)
                            .suffix(" m")
                            .step_by(0.01),
                    );
                });

                // Update surface elevation if grid resolution changed
                if prev_grid_resolution != self.grid_resolution {
                    self.update_surface_elevation();
                }

                ui.separator();

                // Wave parameters section
                ui.heading("Wave Parameters");

                // Wave height control
                ui.horizontal(|ui| {
                    ui.label("Wave Height (H):");
                    Self::info_button(ui, "Vertical distance from wave trough to wave crest. Determines wave energy (E ∝ H²). For linear waves, amplitude a = H/2. Breaking occurs when H/h ≈ 0.78 (depth-limited breaking).");
                    ui.add(
                        egui::Slider::new(&mut self.wave_height, 0.01..=5.0)
                            .suffix(" m")
                            .step_by(0.01),
                    );
                });

                // Wave period control
                ui.horizontal(|ui| {
                    ui.label("Wave Period (T):");
                    Self::info_button(ui, "Time interval between successive wave crests passing a fixed point. Related to frequency by f = 1/T. Determines wavelength through dispersion relation. Typical ocean waves: T = 4-20s, wind waves: T = 1-8s.");
                    ui.add(
                        egui::Slider::new(&mut self.wave_period, 0.1..=20.0)
                            .suffix(" s")
                            .step_by(0.1),
                    );
                });

                // Number of waves control
                ui.horizontal(|ui| {
                    ui.label("Number of Waves:");
                    Self::info_button(ui, "Total number of wave cycles to simulate. Determines simulation duration: t_sim = N × T. More waves show steady-state behavior and wave interactions. Typical studies use 10-50 waves for analysis.");
                    ui.add(egui::Slider::new(&mut self.number_of_waves, 1..=1000).suffix(" waves"));
                });

                ui.separator();

                // Computed values section
                ui.heading("Computed Values");

                // Grid spacing
                ui.horizontal(|ui| {
                    ui.label(format!("Grid Spacing (Δx): {:.3} m", self.grid_spacing()));
                    Self::info_button(ui, "Distance between computational grid points. Formula: Δx = L/(N-1) where L is channel length and N is grid resolution. Smaller spacing improves accuracy but increases computational cost.");
                });

                // Wave properties
                let wave_frequency = 1.0 / self.wave_period;
                let angular_frequency = 2.0 * std::f64::consts::PI * wave_frequency;
                let gravity = 9.81;
                let wavelength = Self::calculate_wavelength_adaptive(self.wave_period, self.still_water_level, gravity);
                let celerity = Self::calculate_celerity_adaptive(self.wave_period, self.still_water_level, gravity);
                let water_regime = Self::classify_water_depth(self.still_water_level, wavelength);

                ui.horizontal(|ui| {
                    ui.label(format!("Wave Frequency (f): {:.3} Hz", wave_frequency));
                    Self::equation_info_button(ui, ctx, equation_renderer, "wave_frequency", "Number of wave cycles per second. Formula: f = 1/T where T is wave period. Fundamental parameter in wave kinematics and energy calculations. Units: Hertz (Hz) or cycles per second.");
                });
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "Angular Frequency (ω): {:.3} rad/s",
                        angular_frequency
                    ));
                    Self::equation_info_button(ui, ctx, equation_renderer, "angular_frequency", "Angular frequency in radians per second. Formula: ω = 2πf = 2π/T. Used in wave equations and dispersion relations. Relates linear frequency to circular motion representation.");
                });
                // Water depth regime classification
                ui.horizontal(|ui| {
                    let regime_text = match water_regime {
                        WaterDepthRegime::Shallow => "Shallow Water",
                        WaterDepthRegime::Intermediate => "Intermediate Water", 
                        WaterDepthRegime::Deep => "Deep Water",
                    };
                    ui.label(format!("Water Depth Regime: {}", regime_text));
                    Self::info_button(ui, "Classification based on h/L ratio. Shallow: h/L < 1/20 (non-dispersive), Deep: h/L > 1/2 (fully dispersive), Intermediate: 1/20 ≤ h/L ≤ 1/2 (transitional). Determines which wave theory applies.");
                });

                ui.horizontal(|ui| {
                    ui.label(format!(
                        "Wave Celerity (c): {:.3} m/s",
                        celerity
                    ));
                    let (equation_id, celerity_tooltip) = match water_regime {
                        WaterDepthRegime::Shallow => ("shallow_water_celerity", "Shallow water celerity: c = √(gh). Independent of wave period."),
                        WaterDepthRegime::Deep => ("deep_water_celerity", "Deep water celerity: c = gT/(2π). Proportional to wave period (dispersive)."),
                        WaterDepthRegime::Intermediate => ("shallow_water_celerity", "Intermediate water celerity: c = L/T using full dispersion relation. Transitional between shallow and deep water behavior."),
                    };
                    Self::equation_info_button(ui, ctx, equation_renderer, equation_id, celerity_tooltip);
                });
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "Wavelength (L): {:.3} m",
                        wavelength
                    ));
                    let (equation_id, wavelength_tooltip) = match water_regime {
                        WaterDepthRegime::Shallow => ("shallow_water_wavelength", "Shallow water wavelength: L = T√(gh). Independent of wave height, depends only on period and depth."),
                        WaterDepthRegime::Deep => ("deep_water_wavelength", "Deep water wavelength: L = gT²/(2π). Depends only on period, independent of depth."),
                        WaterDepthRegime::Intermediate => ("dispersion_relation", "Intermediate water wavelength from full dispersion relation: ω² = gk·tanh(kh). Solved iteratively for accurate results."),
                    };
                    Self::equation_info_button(ui, ctx, equation_renderer, equation_id, wavelength_tooltip);
                });

                ui.separator();

                // Wave channel visualization
                ui.heading("Channel Visualization");

                let (water_surface, channel_bottom, _channel_walls) = self.generate_plot_data();

                // Get available width and use most of it for the plot
                let available_width = ui.available_width();
                let plot_width = (available_width - 40.0).max(400.0); // Leave some margin, minimum 400px

                Plot::new("wave_channel")
                    .height(350.0)
                    .width(plot_width)
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
            },
        );
    }
}
