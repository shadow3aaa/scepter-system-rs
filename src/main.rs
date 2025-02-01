#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;

use app::App;
use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1080.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "The Scepter System",
        options,
        Box::new(|context| {
            // This gives us image support:
            egui_extras::install_image_loaders(&context.egui_ctx);

            let app = App::setup(context);
            Ok(Box::new(app))
        }),
    )
}
