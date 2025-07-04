use eframe::egui;

mod wave_channel;
pub use wave_channel::WaveChannelApp;

pub struct PlatformApp {
    wave_channel_app: WaveChannelApp,
}

impl PlatformApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            wave_channel_app: WaveChannelApp::new(),
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

                    self.wave_channel_app.show(ui);
                });
        });
    }
}
