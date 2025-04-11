use std::time::Instant;

use crate::framework::Page;

use super::mind_page::MindPage;

pub struct SplashPage {
    timer: Instant,
    home_page: Option<Box<dyn Page>>,
}

impl Default for SplashPage {
    fn default() -> Self {
        Self {
            timer: Instant::now(),
            home_page: Some(Box::new(MindPage::default())),
        }
    }
}

impl SplashPage {
    fn progress(&self) -> f32 {
        (self.timer.elapsed().as_secs_f32() / 1.0).min(1.0)
    }
}

impl Page for SplashPage {
    fn main(
        &mut self,
        ui: &mut egui::Ui,
        frame: &mut eframe::Frame,
        nav_controller: &mut crate::framework::NavigationController,
    ) {
        ui.horizontal_centered(|ui| {
            ui.vertical_centered_justified(|ui| {
                ui.label("Logging in...");
                ui.label(format!("{:.2}%", self.progress() * 100.0));
            })
        });

        if self.progress() >= 1.0 {
            nav_controller
                .set_current_page(self.home_page.take().unwrap(), frame.storage_mut().unwrap());
        }
    }

    fn show_side_panel(&self) -> bool {
        false
    }
}
