use eframe::{egui::Ui, Frame, Storage};

use super::navigation_controller::NavigationController;

pub trait Page {
    /// Called on shutdown, and perhaps at regular intervals. Allows us to save state.
    fn save(&self, storage: &mut dyn Storage) {
        let _ = storage;
    }

    fn on_enter(&mut self, storage: &mut dyn Storage) {
        let _ = storage;
    }

    fn on_exit(&mut self, storage: &mut dyn Storage) {
        self.save(storage);
    }

    /// Provides some top panel tools
    fn top_panel(
        &mut self,
        ui: &mut Ui,
        frame: &mut Frame,
        nav_controller: &mut NavigationController,
    ) {
        let _ = (ui, frame, nav_controller);
    }

    fn side_panel(
        &mut self,
        ui: &mut Ui,
        frame: &mut Frame,
        nav_controller: &mut NavigationController,
    ) {
        let _ = (ui, frame, nav_controller);
    }

    fn show_side_panel(&self) -> bool {
        true
    }

    /// Main of the page
    fn main(&mut self, ui: &mut Ui, frame: &mut Frame, nav_controller: &mut NavigationController);
}
