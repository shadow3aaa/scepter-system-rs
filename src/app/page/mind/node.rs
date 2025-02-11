use eframe::egui::{Frame, TextEdit, Ui};
use egui_snarl::{NodeId, Snarl};
use serde::{Deserialize, Serialize};

use crate::{app::font::label_text, colors};

#[derive(Serialize, Deserialize)]
pub struct NodeOfThought {
    pub is_root: bool,
    pub concept: String,
    pub clarification: String,
    pub childs: Vec<NodeId>,
}

impl NodeOfThought {
    pub const fn new(is_root: bool) -> Self {
        Self {
            is_root,
            concept: String::new(),
            clarification: String::new(),
            childs: Vec::new(),
        }
    }

    pub fn connect(&mut self, node: NodeId) {
        self.childs.push(node);
    }

    pub fn disconnect(&mut self, node: NodeId) {
        self.childs.retain(|&n| n != node);
    }

    pub fn ui(&mut self, ui: &mut Ui, scale: f32) {
        Frame::NONE.outer_margin(5.0 * scale).show(ui, |ui| {
            ui.vertical(|ui| {
                ui.label(label_text("Concept"));
                TextEdit::multiline(&mut self.concept)
                    .background_color(colors::editor(ui.style().visuals.dark_mode))
                    .margin(ui.spacing().item_spacing)
                    .show(ui);

                ui.add_space(5.0);
                if !self.concept.trim().is_empty() {
                    ui.label(label_text("Clarification"));
                    ui.add_space(5.0);

                    TextEdit::multiline(&mut self.clarification)
                        .background_color(colors::editor(ui.style().visuals.dark_mode))
                        .margin(ui.spacing().item_spacing)
                        .show(ui);
                }
            });
        });
    }
}

pub fn remove_nodes(snarl: &mut Snarl<NodeOfThought>, nodeid: NodeId) {
    for child in snarl[nodeid].childs.clone() {
        remove_nodes(snarl, child);
    }

    snarl.remove_node(nodeid);
}