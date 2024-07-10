
use wasm_bindgen::prelude::wasm_bindgen;
pub struct MyApp {
    name: String
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Hello {}", self.name));
        });
    }
 }


impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, name: String) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self { 
            name
        }
    }
}

#[wasm_bindgen]
pub fn start(name: String) {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async move {    
        let _ = eframe::WebRunner::new()
        .start(
            "canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(MyApp::new(cc, name))),
        )
        .await;
    });
}