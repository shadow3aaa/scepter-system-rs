use eframe::egui::{Frame, Ui};

use crate::app::font::{heading_text, super_label_text};

use super::{mind::MindPage, NavigationController, Page};

#[derive(Default)]
pub struct HomePage;

impl Page for HomePage {
    fn main_ui(&mut self, ui: &mut Ui, nav_controller: &mut NavigationController) {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            ui.heading(heading_text("The Scepter System"));
            ui.add_space(50.0);
            Frame::NONE.outer_margin(5).show(ui, |ui| {
                ui.style_mut().visuals.button_frame = false;
                if ui.button(super_label_text("Mind")).clicked() {
                    let page = MindPage::default();
                    nav_controller.push(Box::new(page));
                }
                ui.button(super_label_text("Settings")).clicked();
                if ui.button(super_label_text("About")).clicked() {}
            });
        });
    }
}
