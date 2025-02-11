mod node;
mod snarl_viewer;

use eframe::{
    egui::{Color32, CornerRadius, Frame, Margin, Shadow, Stroke, Ui},
    get_value, set_value, Storage,
};
use egui_snarl::{
    ui::{NodeLayout, PinPlacement, SnarlStyle},
    Snarl,
};

use super::{NavigationController, Page};
use crate::{
    app::{font::label_text, ollama_wrapper::OllamaWrapper},
    colors,
};

use node::NodeOfThought;
use snarl_viewer::{snarl_default, MindViewer};

pub struct MindPage {
    snarl: Snarl<NodeOfThought>,
    viewer: MindViewer,
    ollama: OllamaWrapper,
}

impl MindPage {
    pub fn new(storage: &dyn Storage) -> Self {
        let snarl: Snarl<NodeOfThought> =
            get_value(storage, "MindPage:snarl").unwrap_or_else(snarl_default);

        Self {
            snarl,
            viewer: MindViewer,
            ollama: OllamaWrapper::new(),
        }
    }
}

impl Page for MindPage {
    fn save(&self, storage: &mut dyn eframe::Storage) {
        set_value(storage, "MindPage:snarl", &self.snarl);
    }

    fn top_panel_ui(
        &mut self,
        ui: &mut Ui,
        _frame: &mut eframe::Frame,
        _nav_controller: &mut NavigationController,
    ) {
        ui.menu_button(
            label_text(
                self.ollama
                    .current_model
                    .as_deref()
                    .unwrap_or("Choose a model before you start"),
            ),
            |ui| {
                ui.set_min_width(200.0);
                if let Some(model) = &self.ollama.current_model {
                    if ui.button("set as default").clicked() {
                        // TODO: set current model as default
                        ui.close_menu();
                    }
                }

                for model in &self.ollama.model_list {
                    if ui.button(model).clicked() {
                        self.ollama.current_model = Some(model.clone());
                        ui.close_menu();
                    }
                }
            },
        );
    }

    fn main_ui(
        &mut self,
        ui: &mut Ui,
        _frame: &mut eframe::Frame,
        _nav_controller: &mut NavigationController,
    ) {
        self.snarl.show(
            &mut self.viewer,
            &snarl_style(ui.style().visuals.dark_mode),
            "MinePage",
            ui,
        );
    }
}

fn snarl_style(dark_mode: bool) -> SnarlStyle {
    let fill = colors::conatiner_background(dark_mode);

    let shadow = Shadow {
        offset: [10, 20],
        blur: 15,
        spread: 0,
        color: Color32::from_black_alpha(25),
    };

    SnarlStyle {
        node_layout: Some(NodeLayout::Sandwich),
        pin_placement: Some(PinPlacement::Outside { margin: 1.0 }),
        pin_size: Some(7.0),
        node_frame: Some(Frame {
            inner_margin: Margin::same(8),
            outer_margin: Margin {
                left: 0,
                right: 0,
                top: 0,
                bottom: 4,
            },
            corner_radius: CornerRadius::same(8),
            fill,
            stroke: Stroke::NONE,
            shadow,
        }),
        bg_frame: Some(Frame {
            inner_margin: Margin::same(2),
            outer_margin: Margin::ZERO,
            corner_radius: CornerRadius::ZERO,
            stroke: Stroke::NONE,
            ..Default::default()
        }),
        ..SnarlStyle::new()
    }
}
