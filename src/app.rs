mod font;

use eframe::{egui::{self, Context}, epaint::text::FontInsert, CreationContext};
use font::set_font;

pub struct App {
}

impl App {
    pub fn setup(context: &CreationContext<'_>) -> Self {
        set_font(&context.egui_ctx);

        Self {
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("The Scepter System");
            ui.label("Welcome to The Scepter System!");
        });
    }
}