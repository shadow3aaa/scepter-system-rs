mod navigation_controller;
mod page;

pub use navigation_controller::NavigationController;
pub use page::Page;

macro_rules! icon {
    ($icon:expr, $dark_mode:expr) => {{
        if $dark_mode {
            egui::include_image!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/dark/",
                $icon,
                ".svg"
            ))
        } else {
            egui::include_image!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/light/",
                $icon,
                ".svg"
            ))
        }
    }};
}

macro_rules! icon_button {
    ($ui:expr, $icon:expr, $frame:expr, $size:expr) => {{
        use egui::{Button, Image, Vec2};

        let mut button = Button::image(
            Image::new(icon!($icon, $ui.style().visuals.dark_mode))
                .fit_to_exact_size(Vec2::new($size, $size)),
        );
        if !$frame {
            button = button.fill(egui::Color32::TRANSPARENT);
        }
        $ui.add(button)
    }};
}
