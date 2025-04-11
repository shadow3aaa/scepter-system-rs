use std::{collections::HashMap, sync::Arc, thread};

use eframe::egui::{Pos2, Ui};
use egui_snarl::{
    ui::{AnyPins, PinInfo, SnarlPin, SnarlViewer, WireStyle},
    InPin, InPinId, NodeId, OutPin, OutPinId, Snarl,
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use crate::{app::font::body_text, colors};

use super::{
    custom_snarl_default,
    llm_adapters::LLMAdapters,
    node::{remove_nodes, NodeOfThought},
    prompt::{self, Concept, ConceptStreamParser},
};

#[derive(Serialize, Deserialize)]
pub struct MindViewer {
    #[serde(serialize_with = "crate::serde_utils::arc_rwlock_serde")]
    #[serde(deserialize_with = "crate::serde_utils::arc_rwlock_deserialize")]
    pub snarl: Arc<RwLock<Snarl<NodeOfThought>>>,
    #[serde(skip)]
    pub adapters: Option<Arc<RwLock<LLMAdapters>>>,
}

impl MindViewer {
    pub fn new(snarl: Snarl<NodeOfThought>, adapter: Arc<RwLock<LLMAdapters>>) -> Self {
        Self {
            snarl: Arc::new(RwLock::new(snarl)),
            adapters: Some(adapter),
        }
    }
}

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
    ) -> impl SnarlPin + 'static {
        pin_style(ui.style().visuals.dark_mode)
    }

    fn outputs(&mut self, node: &NodeOfThought) -> usize {
        usize::from(!node.concept.core.trim().is_empty())
    }

    fn show_output(
        &mut self,
        _pin: &OutPin,
        ui: &mut Ui,
        _scale: f32,
        _snarl: &mut Snarl<NodeOfThought>,
    ) -> impl SnarlPin + 'static {
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
            let parent_core = snarl[node].concept.core.clone();
            let parent_clarification = snarl[node].concept.clarification.clone();

            let messages = prompt::divergence(&Concept {
                core: parent_core,
                clarification: parent_clarification,
            });

            {
                let snarl = self.snarl.clone();
                let adapter = self.adapters.as_ref().unwrap().clone();

                thread::spawn(move || {
                    let Some(rx) = adapter.read().chat(messages) else {
                        return;
                    };

                    let mut parser = ConceptStreamParser::new();
                    let mut y_offset = 100.0;
                    let base_pos = snarl.read()[node].rect.center();

                    let mut new_nodes: HashMap<usize, NodeId> = HashMap::new();

                    while let Ok(content) = rx.recv() {
                        for (index, concept) in parser.push_chunk(&content).iter().enumerate() {
                            if let Some(node) = new_nodes.get(&index) {
                                snarl.write()[*node].concept.core.clone_from(&concept.core);
                                snarl.write()[*node]
                                    .concept
                                    .clarification
                                    .clone_from(&concept.clarification);
                            } else {
                                let new_node = snarl.write().insert_node(
                                    Pos2::new(base_pos.x, base_pos.y + y_offset),
                                    NodeOfThought::new(false),
                                );

                                let out_pin = OutPinId { node, output: 0 };
                                let in_pin = InPinId {
                                    node: new_node,
                                    input: 0,
                                };
                                snarl.write()[node].connect(new_node);
                                snarl.write().connect(out_pin, in_pin);
                                y_offset += 80.0;
                                new_nodes.insert(index, new_node);
                            }
                        }
                    }
                });
            }

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
            AnyPins::Out(out_pin_ids) => !snarl[out_pin_ids[0].node].concept.core.trim().is_empty(),
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
                ui.set_min_width(100.0);
                ui.label("Add node");
                ui.separator();
                if ui.button("Divergence").clicked() {
                    let node = snarl.insert_node(pos, NodeOfThought::new(false));
                    let id = InPinId { node, input: 0 };

                    self.connect(&snarl.out_pin(src_pin[0]), &snarl.in_pin(id), snarl);
                    ui.close_menu();
                }
            }
        }
    }

    fn has_graph_menu(&mut self, _pos: Pos2, _snarl: &mut Snarl<NodeOfThought>) -> bool {
        true
    }

    fn show_graph_menu(
        &mut self,
        _pos: Pos2,
        ui: &mut Ui,
        _scale: f32,
        snarl: &mut Snarl<NodeOfThought>,
    ) {
        if ui.button("Reset").clicked() {
            *snarl = custom_snarl_default();
            ui.close_menu();
        }
    }
}

fn pin_style(dark_mode: bool) -> PinInfo {
    PinInfo::circle()
        .with_wire_style(WireStyle::AxisAligned {
            corner_radius: 25.0,
        })
        .with_fill(colors::pin(dark_mode))
}
