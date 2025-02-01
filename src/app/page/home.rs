use eframe::egui::{Frame, Ui};

use crate::app::font::{heading_text, label_text};

use super::{NavigationController, Page, mind::MindPage};

#[derive(Default)]
pub struct HomePage;

impl Page for HomePage {
    fn ui(&mut self, ui: &mut Ui, nav_controller: &mut NavigationController) {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            ui.heading(heading_text("The Scepter System"));
            ui.add_space(50.0);
            Frame::none().outer_margin(5.0).show(ui, |ui| {
                ui.style_mut().visuals.button_frame = false;
                if ui.button(label_text("Mind")).clicked() {
                    let page = MindPage::default();
                    nav_controller.push(Box::new(page));
                };
                ui.button(label_text("Settings")).clicked();
                if ui.button(label_text("About")).clicked() {}
            });
        });
    }
}
