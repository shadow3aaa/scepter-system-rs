mod font;
mod page;

use eframe::{
    CreationContext,
    egui::{self, Ui},
};

use font::set_font;
use page::{NavigationController, Page, home::HomePage};

pub struct App {
    nav_controller: NavigationController,
}

impl App {
    pub fn setup(context: &CreationContext<'_>) -> Self {
        set_font(&context.egui_ctx);
        Self {
            nav_controller: NavigationController::new(Box::new(HomePage::default())),
        }
    }

    fn show_page(&mut self, ui: &mut Ui) {
        self.nav_controller.current_page().ui(ui);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_page(ui);
        });
    }
}
