mod history_card;
mod llm_adapters;
mod node;
mod prompt;
mod snarl_viewer;

use std::{collections::VecDeque, sync::Arc};

use eframe::egui::{Color32, CornerRadius, Frame, Margin, Shadow, Stroke, Ui};
use egui::Pos2;
use egui_file_dialog::FileDialog;
use egui_snarl::{
    ui::{NodeLayout, PinPlacement, SnarlStyle},
    Snarl,
};
use llm_adapters::LLMAdapters;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use super::NavigationController;
use crate::{colors, framework::Page};

use history_card::history_card;
use node::NodeOfThought;
use snarl_viewer::MindViewer;

#[derive(Serialize, Deserialize)]
struct SnarlWrapper {
    #[serde(serialize_with = "crate::serde_utils::arc_rwlock_serde")]
    #[serde(deserialize_with = "crate::serde_utils::arc_rwlock_deserialize")]
    snarl: Arc<RwLock<Snarl<NodeOfThought>>>,
    viewer: MindViewer,
}

pub struct MindPage {
    file_dialog: FileDialog,
    history: VecDeque<SnarlWrapper>,
}

impl MindPage {
    pub fn new() -> Self {
        let mut history = VecDeque::new();
        history.push_back(SnarlWrapper {
            snarl: Arc::new(RwLock::new(custom_snarl_default())),
            viewer: MindViewer::new(
                Arc::new(RwLock::new(custom_snarl_default())),
                Arc::new(RwLock::new(LLMAdapters::new())),
            ),
        });
        Self {
            file_dialog: FileDialog::new(),
            history,
        }
    }
}

impl Default for MindPage {
    fn default() -> Self {
        let snarl = custom_snarl_default();
        let snarl = Arc::new(RwLock::new(snarl));
        let adapter = Arc::new(RwLock::new(LLMAdapters::new()));

        let mut history = VecDeque::new();
        history.push_back(SnarlWrapper {
            snarl: snarl.clone(),
            viewer: MindViewer::new(snarl, adapter),
        });

        Self {
            history: VecDeque::new(),
            file_dialog: FileDialog::new(),
        }
    }
}

impl Page for MindPage {
    fn side_panel(&mut self, ui: &mut Ui, _: &mut eframe::Frame, _: &mut NavigationController) {
        ui.set_width(ui.available_width());
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (index, snarl_wapper) in self.history.iter().enumerate() {
                let snarl = snarl_wapper.snarl.read();
                let node = snarl.nodes().next().unwrap();
                history_card(ui, &node.concept.core, index == 0);
            }
        });
    }

    fn main(
        &mut self,
        ui: &mut Ui,
        _frame: &mut eframe::Frame,
        _nav_controller: &mut NavigationController,
    ) {
        let snarl_wrapper = self.history.front_mut().unwrap();
        snarl_wrapper.snarl.write().show(
            &mut snarl_wrapper.viewer,
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
