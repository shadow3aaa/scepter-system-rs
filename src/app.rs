mod font;
mod llama_wrapper;
mod navigation_controller;
mod page;

use eframe::{
    egui::{self, Theme},
    CreationContext, Storage,
};
use egui_material_icons::icons::{ICON_ARROW_BACK, ICON_DARK_MODE, ICON_HOME, ICON_LIGHT_MODE};

use font::{label_text, set_font, COLOR_DISABLED};
use navigation_controller::NavigationController;
use page::home::HomePage;

pub struct App {
    nav_controller: Box<NavigationController>,
}

impl App {
    pub fn setup(context: &CreationContext<'_>) -> Self {
        set_font(&context.egui_ctx);
        Self {
            nav_controller: NavigationController::new(Box::new(HomePage)),
        }
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn Storage) {
        for page in self.nav_controller.pages_mut() {
            page.save(storage);
        }
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.style_mut().visuals.button_frame = false;
                let is_root_page = self.nav_controller.len() == 1;

                let home_button_label = if is_root_page {
                    label_text(ICON_HOME).color(COLOR_DISABLED)
                } else {
                    label_text(ICON_HOME)
                };

                let back_button_label = if is_root_page {
                    label_text(ICON_ARROW_BACK).color(COLOR_DISABLED)
                } else {
                    label_text(ICON_ARROW_BACK)
                };

                let theme_button_label = if ui.style_mut().visuals.dark_mode {
                    label_text(ICON_LIGHT_MODE)
                } else {
                    label_text(ICON_DARK_MODE)
                };

                if ui.button(home_button_label).clicked() && !is_root_page {
                    self.nav_controller
                        .set_current_page(Box::new(HomePage), frame.storage_mut().unwrap());
                }

                if ui.button(back_button_label).clicked() && !is_root_page {
                    self.nav_controller.pop(frame.storage_mut().unwrap());
                }

                if ui.button(theme_button_label).clicked() {
                    ctx.set_theme(if ui.style_mut().visuals.dark_mode {
                        Theme::Light
                    } else {
                        Theme::Dark
                    });
                }

                self.nav_controller.top_panel_ui(ui, frame);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().debug.show_unaligned = false;
            self.nav_controller.main_ui(ui, frame);
        });
    }
}
