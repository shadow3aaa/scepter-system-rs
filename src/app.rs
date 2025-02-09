mod font;
mod page;

use eframe::{
    egui::{self, Theme, Ui},
    CreationContext,
};

use egui_material_icons::icons::{ICON_ARROW_BACK, ICON_DARK_MODE, ICON_HOME, ICON_LIGHT_MODE};
use font::{label_text, set_font, COLOR_DISABLED};
use page::{home::HomePage, NavigationController};

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

    fn show_page(&mut self, ui: &mut Ui) {
        self.nav_controller.ui(ui);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                    self.nav_controller.set_current_page(Box::new(HomePage));
                }

                if ui.button(back_button_label).clicked() && !is_root_page {
                    self.nav_controller.pop();
                }

                if ui.button(theme_button_label).clicked() {
                    ctx.set_theme(if ui.style_mut().visuals.dark_mode {
                        Theme::Light
                    } else {
                        Theme::Dark
                    });
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().debug.show_unaligned = false;
            self.show_page(ui);
        });
    }
}
