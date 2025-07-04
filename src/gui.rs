use eframe::egui;

mod wave_channel;
mod equations;
pub use wave_channel::WaveChannelApp;
pub use equations::EquationRenderer;

pub struct PlatformApp {
    wave_channel_app: WaveChannelApp,
    equation_renderer: EquationRenderer,
}

impl PlatformApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut equation_renderer = EquationRenderer::new();
        if let Err(e) = equation_renderer.load_equations() {
            eprintln!("Failed to load equations: {}", e);
        }
        
        Self {
            wave_channel_app: WaveChannelApp::new(),
            equation_renderer,
        }
    }
}

impl eframe::App for PlatformApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.heading("Coastal Engineering Platform");
                    ui.separator();

                    self.wave_channel_app.show(ui, ctx, &mut self.equation_renderer);
                });
        });
    }
}
