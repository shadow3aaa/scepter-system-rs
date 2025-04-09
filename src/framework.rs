mod navigation_controller;
mod page;

pub use navigation_controller::NavigationController;
pub use page::Page;

macro_rules! icon {
    ($icon:expr, $dark_mode:expr) => {{
        if $dark_mode {
            egui::include_image!(concat!("../../assets/dark/", $icon, ".svg"))
        } else {
            egui::include_image!(concat!("../../assets/light/", $icon, ".svg"))
        }
    }};
}
