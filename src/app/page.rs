pub mod home;
pub mod mind;

use eframe::{egui::Ui, Frame, Storage};

use super::navigation_controller::NavigationController;

pub trait Page {
    /// Called on shutdown, and perhaps at regular intervals. Allows us to save state.
    fn save(&self, storage: &mut dyn Storage) {
        let _ = storage;
    }

    fn on_enter(&self, storage: &mut dyn Storage) {
        self.save(storage);
    }

    fn on_exit(&self, storage: &mut dyn Storage) {
        self.save(storage);
    }

    /// Provides some top panel tools
    fn top_panel_ui(
        &mut self,
        ui: &mut Ui,
        frame: &mut Frame,
        nav_controller: &mut NavigationController,
    ) {
        let _ = (ui, frame, nav_controller);
    }

    /// Main of the page
    fn main_ui(
        &mut self,
        ui: &mut Ui,
        frame: &mut Frame,
        nav_controller: &mut NavigationController,
    );
}
