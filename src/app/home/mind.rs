mod llm_adapters;
mod node;
mod prompt;
mod snarl_viewer;

use std::sync::Arc;

use eframe::egui::{Color32, CornerRadius, Frame, Margin, Shadow, Stroke, Ui};
use egui::Pos2;
use egui_file_dialog::FileDialog;
use egui_snarl::{
    ui::{NodeLayout, PinPlacement, SnarlStyle},
    Snarl,
};
use llm_adapters::ollama_adapter::OllamaAdapter;
use parking_lot::RwLock;

use super::{NavigationController, Page};
use crate::colors;

use node::NodeOfThought;
use snarl_viewer::MindViewer;

pub struct MindPage {
    snarl: Arc<RwLock<Snarl<NodeOfThought>>>,
    viewer: MindViewer,
    file_dialog: FileDialog,
}

impl MindPage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for MindPage {
    fn default() -> Self {
        let snarl = custom_snarl_default();
        let snarl = Arc::new(RwLock::new(snarl));
        let adapter = Arc::new(RwLock::new(OllamaAdapter::new(None)));

        Self {
            viewer: MindViewer::new(snarl.clone(), adapter),
            snarl,
            file_dialog: FileDialog::new(),
        }
    }
}

impl Page for MindPage {
    fn main(
        &mut self,
        ui: &mut Ui,
        _frame: &mut eframe::Frame,
        _nav_controller: &mut NavigationController,
    ) {
        self.snarl.write().show(
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

pub fn custom_snarl_default() -> Snarl<NodeOfThought> {
    let mut snarl = Snarl::new();
    snarl.insert_node(Pos2::new(0.0, 0.0), NodeOfThought::new(true));
    snarl
}
