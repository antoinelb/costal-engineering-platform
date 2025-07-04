use eframe::egui;

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
            ui.heading("Coastal Engineering Platform");
            ui.separator();
            
            self.wave_channel_app.show(ui);
        });
    }
}

pub struct WaveChannelApp {
}

impl WaveChannelApp {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("1D Wave Channel Simulator");
        ui.label("Wave simulation module - coming soon");
    }
}