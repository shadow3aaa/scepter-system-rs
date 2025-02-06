use eframe::egui::{
    Color32, Frame, Margin, Pos2, Rounding, Shadow, Stroke, TextEdit, Ui, pos2, vec2,
};
use egui_snarl::{
    InPin, InPinId, NodeId, OutPin, Snarl,
    ui::{AnyPins, NodeLayout, PinInfo, PinPlacement, SnarlStyle, SnarlViewer, WireStyle},
};

use super::{NavigationController, Page};
use crate::{
    app::font::{body_text, label_text},
    colors,
};

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

fn snarl_style(dark_mode: bool) -> SnarlStyle {
    let fill = colors::conatiner_background(dark_mode);

    let shadow = Shadow {
        offset: vec2(10.0, 20.0),
        blur: 15.0,
        spread: 0.0,
        color: Color32::from_black_alpha(25),
    };

    SnarlStyle {
        node_layout: Some(NodeLayout::Basic),
        pin_placement: Some(PinPlacement::Outside { margin: 1.0 }),
        pin_size: Some(7.0),
        node_frame: Some(Frame {
            inner_margin: Margin::same(8.0),
            outer_margin: Margin {
                left: 0.0,
                right: 0.0,
                top: 0.0,
                bottom: 4.0,
            },
            rounding: Rounding::same(8.0),
            fill,
            stroke: Stroke::NONE,
            shadow,
        }),
        bg_frame: Some(Frame {
            inner_margin: Margin::same(2.0),
            outer_margin: Margin::ZERO,
            rounding: Rounding::ZERO,
            stroke: Stroke::NONE,
            ..Default::default()
        }),
        ..SnarlStyle::new()
    }
}

impl Page for MindPage {
    fn ui(&mut self, ui: &mut Ui, _nav_controller: &mut NavigationController) {
        self.snarl.show(
            &mut self.viewer,
            &snarl_style(ui.style().visuals.dark_mode),
            "MinePage",
            ui,
        );
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

    fn outputs(&mut self, node: &NodeOfThought) -> usize {
        usize::from(!node.concept.trim().is_empty())
    }

    fn show_output(
        &mut self,
        _pin: &OutPin,
        ui: &mut Ui,
        _scale: f32,
        _snarl: &mut Snarl<NodeOfThought>,
    ) -> PinInfo {
        pin_style(ui.style().visuals.dark_mode)
    }

    fn has_body(&mut self, _node: &NodeOfThought) -> bool {
        true
    }

    fn show_body(
        &mut self,
        node: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<NodeOfThought>,
    ) {
        snarl[node].ui(ui, scale);
    }

    fn has_node_menu(&mut self, _node: &NodeOfThought) -> bool {
        true
    }

    fn show_node_menu(
        &mut self,
        node: NodeId,
        inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<NodeOfThought>,
    ) {
        ui.label(body_text("Node Menu"));
        ui.separator();

        if ui.button("Divergence").clicked() {
            // TODO: Implement auto-divergence
            ui.close_menu();
        }

        if !snarl[node].is_root && ui.button("Remove this node").clicked() {
            self.disconnect(&snarl.out_pin(inputs[0].remotes[0]), &inputs[0], snarl);
            ui.close_menu();
        }
    }

    fn has_dropped_wire_menu(
        &mut self,
        src_pins: AnyPins,
        snarl: &mut Snarl<NodeOfThought>,
    ) -> bool {
        match src_pins {
            AnyPins::Out(out_pin_ids) => !snarl[out_pin_ids[0].node].concept.trim().is_empty(),
            AnyPins::In(_) => false,
        }
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
                ui.separator();
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
    PinInfo::circle()
        .with_wire_style(WireStyle::AxisAligned {
            corner_radius: 25.0,
        })
        .with_fill(colors::pin(dark_mode))
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

    fn ui(&mut self, ui: &mut Ui, scale: f32) {
        Frame::none().outer_margin(5.0 * scale).show(ui, |ui| {
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
