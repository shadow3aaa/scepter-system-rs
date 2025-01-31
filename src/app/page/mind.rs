use eframe::egui::{Ui, Window};

use super::{NavigationController, Page};

#[derive(Default)]
pub struct MindPage {}

impl Page for MindPage {
    fn ui(&mut self, ui: &mut Ui, nav_controller: &mut NavigationController) {
        Window::new("Mind").show(ui.ctx(), |ui| {
            ui.label("Mind Page");
        });
    }
}
