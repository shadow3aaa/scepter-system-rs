use eframe::egui::{Frame, Ui};

use crate::app::font::{body_text, heading_text};

use super::Page;

#[derive(Default)]
pub struct HomePage {}

impl Page for HomePage {
    fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            ui.heading(heading_text("The Scepter System"));
            ui.add_space(50.0);
            Frame::none().outer_margin(5.0).show(ui, |ui| {
                ui.style_mut().visuals.button_frame = false;
                ui.button(body_text("Mind")).clicked();
                ui.button(body_text("Settings")).clicked();
                if ui.button(body_text("About")).clicked() {}
            });
        });
    }
}
