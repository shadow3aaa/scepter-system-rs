mod node;
mod snarl_viewer;

use std::path::Path;

use eframe::{
    egui::{
        Color32, CornerRadius, Frame, Margin, Shadow, Stroke, Ui,
    },
    get_value, set_value, Storage,
};
use egui_file_dialog::FileDialog;
use egui_snarl::{
    ui::{NodeLayout, PinPlacement, SnarlStyle},
    Snarl,
};
use serde::{Deserialize, Serialize};

use super::{NavigationController, Page};
use crate::{
    app::{
        font::label_text,
        llama_wrapper::{params::Params, Llama, ModelInfo},
    },
    colors::{self},
};

use node::NodeOfThought;
use snarl_viewer::{custom_snarl_default, MindViewer};

#[derive(Serialize, Deserialize)]
pub struct MindPage {
    snarl: Snarl<NodeOfThought>,
    #[serde(skip)]
    file_dialog: FileDialog,
    viewer: MindViewer,
    llama: Llama,
}

impl MindPage {
    pub fn new(storage: &dyn Storage) -> Self {
        get_value(storage, "MindPage").unwrap_or_default()
    }
}

impl Default for MindPage {
    fn default() -> Self {
        Self {
            snarl: custom_snarl_default(),
            file_dialog: FileDialog::new(),
            viewer: MindViewer,
            llama: Llama::new(),
        }
    }
}

impl Page for MindPage {
    fn save(&self, storage: &mut dyn eframe::Storage) {
        set_value(storage, "MindPage", &self);
    }

    fn top_panel_ui(
        &mut self,
        ui: &mut Ui,
        _frame: &mut eframe::Frame,
        _nav_controller: &mut NavigationController,
    ) {
        ui.menu_button(
            label_text(
                self.llama
                    .get_current_model_name()
                    .unwrap_or_else(|| "Choose a model before you start".to_string()),
            ),
            |ui| {
                ui.set_min_width(220.0);

                if ui.button("add model from file").clicked() {
                    self.file_dialog.pick_file();
                }

                if self.llama.loaded() && ui.button("set current model as default").clicked() {
                    // TODO: set current model as default
                    ui.close_menu();
                }

                if !self.llama.is_empty() {
                    ui.separator();
                }

                let model_button = |ui: &mut Ui,
                                    llama: &mut Llama,
                                    path: &Path,
                                    model: &ModelInfo,
                                    choosed: bool| {
                    let mut response = ui.button(model.name.to_string());
                    if choosed {
                        response = response.highlight();
                    }

                    if response.clicked() && !choosed {
                        llama.load_model(path, Params::default());
                        ui.close_menu();
                    }
                };

                if let Some((path, model)) = self
                    .llama
                    .current_model()
                    .map(|(path, model)| (path.to_path_buf(), model.clone()))
                {
                    model_button(ui, &mut self.llama, &path, &model, true);
                }

                let models: Vec<_> = self
                    .llama
                    .models()
                    .into_iter()
                    .map(|(path, model)| (path.clone(), model.clone()))
                    .collect();
                for (path, model) in models {
                    if let Some((current_model_path, _)) = self.llama.current_model() {
                        if path == current_model_path {
                            continue;
                        }
                    }

                    model_button(ui, &mut self.llama, &path, &model, false);
                }
            },
        );

        self.file_dialog.update(ui.ctx());

        if let Some(path) = self.file_dialog.take_picked() {
            self.llama.load_model(path, Params::default());
        }
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
