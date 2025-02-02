use eframe::egui::{Align, Color32, Layout, Pos2, TextEdit, Ui, pos2};
use egui_snarl::{
    InPin, InPinId, NodeId, OutPin, Snarl,
    ui::{AnyPins, PinInfo, SnarlStyle, SnarlViewer, WireStyle},
};

use super::{NavigationController, Page};
use crate::app::font::label_text;

pub struct MindPage {
    snarl: Snarl<NodeOfThought>,
    viewer: MindViewer,
}

impl Default for MindPage {
    fn default() -> Self {
        let mut snarl = Snarl::new();
        snarl.insert_node(pos2(0.0, 0.0), NodeOfThought::new(true));
        Self {
            snarl,
            viewer: MindViewer,
        }
    }
}

impl Page for MindPage {
    fn ui(&mut self, ui: &mut Ui, _nav_controller: &mut NavigationController) {
        self.snarl
            .show(&mut self.viewer, &SnarlStyle::default(), "MinePage", ui);
    }
}

struct MindViewer;

impl SnarlViewer<NodeOfThought> for MindViewer {
    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<NodeOfThought>) {
        snarl[from.id.node].connect(to.id.node);
        snarl.connect(from.id, to.id);
    }

    fn disconnect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<NodeOfThought>) {
        remove_nodes(snarl, to.id.node);
        snarl[from.id.node].disconnect(to.id.node);
    }

    fn title(&mut self, node: &NodeOfThought) -> String {
        if node.is_root {
            "Root Node Of Thought".to_string()
        } else {
            "Node of Thought".to_string()
        }
    }

    fn inputs(&mut self, node: &NodeOfThought) -> usize {
        usize::from(!node.is_root)
    }

    fn show_input(
        &mut self,
        _pin: &egui_snarl::InPin,
        ui: &mut Ui,
        _scale: f32,
        _snarl: &mut Snarl<NodeOfThought>,
    ) -> PinInfo {
        pin_style(ui.style().visuals.dark_mode)
    }

    fn outputs(&mut self, _node: &NodeOfThought) -> usize {
        1
    }

    fn show_output(
        &mut self,
        pin: &egui_snarl::OutPin,
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<NodeOfThought>,
    ) -> PinInfo {
        ui.with_layout(Layout::top_down(Align::Max), |ui| {
            snarl[pin.id.node].ui(ui);
        });
        pin_style(ui.style().visuals.dark_mode)
    }

    fn has_graph_menu(&mut self, _pos: Pos2, _snarl: &mut Snarl<NodeOfThought>) -> bool {
        true
    }

    fn show_graph_menu(
        &mut self,
        pos: Pos2,
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<NodeOfThought>,
    ) {
        if ui.button("Add Node").clicked() {
            snarl.insert_node(pos, NodeOfThought::new(false));
        }
    }

    fn has_node_menu(&mut self, _node: &NodeOfThought) -> bool {
        true
    }

    fn show_node_menu(
        &mut self,
        _node: egui_snarl::NodeId,
        _inputs: &[egui_snarl::InPin],
        _outputs: &[egui_snarl::OutPin],
        ui: &mut Ui,
        _scale: f32,
        _snarl: &mut Snarl<NodeOfThought>,
    ) {
        if ui.button("Divergence").clicked() {
            // TODO: Implement divergence
            ui.close_menu();
        }
    }

    fn has_dropped_wire_menu(
        &mut self,
        _src_pins: AnyPins,
        _snarl: &mut Snarl<NodeOfThought>,
    ) -> bool {
        true
    }

    fn show_dropped_wire_menu(
        &mut self,
        pos: Pos2,
        ui: &mut Ui,
        _scale: f32,
        src_pins: AnyPins,
        snarl: &mut Snarl<NodeOfThought>,
    ) {
        match src_pins {
            AnyPins::In(_) => {
                ui.close_menu();
            }
            AnyPins::Out(src_pin) => {
                ui.label("Add node");
                if ui.button("Divergence(Manually)").clicked() {
                    let node = snarl.insert_node(pos, NodeOfThought::new(false));
                    let id = InPinId { node, input: 0 };

                    self.connect(&snarl.out_pin(src_pin[0]), &snarl.in_pin(id), snarl);
                    ui.close_menu();
                }
            }
        }
    }
}

fn remove_nodes(snarl: &mut Snarl<NodeOfThought>, nodeid: NodeId) {
    for child in snarl[nodeid].childs.clone() {
        remove_nodes(snarl, child);
    }

    snarl.remove_node(nodeid);
}

fn pin_style(dark_mode: bool) -> PinInfo {
    if dark_mode {
        PinInfo::circle()
            .with_wire_style(WireStyle::AxisAligned {
                corner_radius: 25.0,
            })
            .with_fill(Color32::GOLD)
    } else {
        PinInfo::circle()
            .with_wire_style(WireStyle::AxisAligned {
                corner_radius: 25.0,
            })
            .with_fill(Color32::GRAY)
    }
}

struct NodeOfThought {
    is_root: bool,
    concept: String,
    clarification: String,
    childs: Vec<NodeId>,
}

impl NodeOfThought {
    const fn new(is_root: bool) -> Self {
        Self {
            is_root,
            concept: String::new(),
            clarification: String::new(),
            childs: Vec::new(),
        }
    }

    fn connect(&mut self, node: NodeId) {
        self.childs.push(node);
    }

    fn disconnect(&mut self, node: NodeId) {
        self.childs.retain(|&n| n != node);
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.label(label_text("Concept"));
        TextEdit::multiline(&mut self.concept)
            .margin(ui.spacing().item_spacing)
            .show(ui);

        ui.add_space(5.0);
        if !self.concept.trim().is_empty() {
            ui.label(label_text("Clarification"));
            ui.add_space(5.0);

            TextEdit::multiline(&mut self.clarification)
                .margin(ui.spacing().item_spacing)
                .show(ui);
        }
    }
}
