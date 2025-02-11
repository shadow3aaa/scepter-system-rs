use eframe::egui::{Pos2, Ui};
use egui_snarl::{ui::{AnyPins, PinInfo, SnarlViewer, WireStyle}, InPin, InPinId, NodeId, OutPin, Snarl};

use crate::{app::font::body_text, colors};

use super::node::{remove_nodes, NodeOfThought};


pub struct MindViewer;

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
    
    fn node_frame(
        &mut self,
        default: eframe::egui::Frame,
        node: egui_snarl::NodeId,
        inputs: &[InPin],
        outputs: &[OutPin],
        snarl: &Snarl<NodeOfThought>,
    ) -> eframe::egui::Frame {
        let _ = (node, inputs, outputs, snarl);
        default
    }
    
    fn header_frame(
        &mut self,
        default: eframe::egui::Frame,
        node: egui_snarl::NodeId,
        inputs: &[InPin],
        outputs: &[OutPin],
        snarl: &Snarl<NodeOfThought>,
    ) -> eframe::egui::Frame {
        let _ = (node, inputs, outputs, snarl);
        default
    }
    
    fn has_node_style(
        &mut self,
        node: egui_snarl::NodeId,
        inputs: &[InPin],
        outputs: &[OutPin],
        snarl: &Snarl<NodeOfThought>,
    ) -> bool {
        let _ = (node, inputs, outputs, snarl);
        false
    }
    
    fn apply_node_style(
        &mut self,
        style: &mut eframe::egui::Style,
        node: egui_snarl::NodeId,
        inputs: &[InPin],
        outputs: &[OutPin],
        snarl: &Snarl<NodeOfThought>,
    ) {
        let _ = (style, node, inputs, outputs, snarl);
    }
    
    fn node_layout(
        &mut self,
        default: egui_snarl::ui::NodeLayout,
        node: egui_snarl::NodeId,
        inputs: &[InPin],
        outputs: &[OutPin],
        snarl: &Snarl<NodeOfThought>,
    ) -> egui_snarl::ui::NodeLayout {
        let _ = (node, inputs, outputs, snarl);
        default
    }
    
    fn show_header(
        &mut self,
        node: egui_snarl::NodeId,
        inputs: &[InPin],
        outputs: &[OutPin],
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<NodeOfThought>,
    ) {
        let _ = (inputs, outputs, scale);
        ui.label(self.title(&snarl[node]));
    }
    
    fn has_footer(&mut self, node: &NodeOfThought) -> bool {
        let _ = node;
        false
    }
    
    fn show_footer(
        &mut self,
        node: egui_snarl::NodeId,
        inputs: &[InPin],
        outputs: &[OutPin],
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<NodeOfThought>,
    ) {
        let _ = (node, inputs, outputs, ui, scale, snarl);
    }
    
    fn final_node_rect(
        &mut self,
        node: egui_snarl::NodeId,
        ui_rect: eframe::egui::Rect,
        graph_rect: eframe::egui::Rect,
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<NodeOfThought>,
    ) {
        let _ = (node, ui_rect, graph_rect, ui, scale, snarl);
    }
    
    fn has_on_hover_popup(&mut self, node: &NodeOfThought) -> bool {
        let _ = node;
        false
    }
    
    fn show_on_hover_popup(
        &mut self,
        node: egui_snarl::NodeId,
        inputs: &[InPin],
        outputs: &[OutPin],
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<NodeOfThought>,
    ) {
        let _ = (node, inputs, outputs, ui, scale, snarl);
    }
    
    fn has_wire_widget(&mut self, from: &egui_snarl::OutPinId, to: &egui_snarl::InPinId, snarl: &Snarl<NodeOfThought>) -> bool {
        let _ = (from, to, snarl);
        false
    }
    
    fn show_wire_widget(
        &mut self,
        from: &OutPin,
        to: &InPin,
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<NodeOfThought>,
    ) {
        let _ = (from, to, ui, scale, snarl);
    }
    
    fn has_graph_menu(&mut self, pos: eframe::egui::Pos2, snarl: &mut Snarl<NodeOfThought>) -> bool {
        let _ = (pos, snarl);
        false
    }
    
    fn show_graph_menu(&mut self, pos: eframe::egui::Pos2, ui: &mut Ui, scale: f32, snarl: &mut Snarl<NodeOfThought>) {
        let _ = (pos, ui, scale, snarl);
    }
    
    fn drop_outputs(&mut self, pin: &OutPin, snarl: &mut Snarl<NodeOfThought>) {
        snarl.drop_outputs(pin.id);
    }
    
    fn drop_inputs(&mut self, pin: &InPin, snarl: &mut Snarl<NodeOfThought>) {
        snarl.drop_inputs(pin.id);
    }
    
    fn draw_input_pin(
        &mut self,
        pin: &InPin,
        pin_info: &PinInfo,
        pos: eframe::egui::Pos2,
        size: f32,
        snarl_style: &egui_snarl::ui::SnarlStyle,
        style: &eframe::egui::Style,
        painter: &eframe::egui::Painter,
        scale: f32,
        snarl: &Snarl<NodeOfThought>,
    ) -> eframe::egui::Color32 {
        let _ = (pin, snarl);
    
        pin_info.draw(pos, size, snarl_style, style, painter, scale)
    }
    
    fn draw_output_pin(
        &mut self,
        pin: &OutPin,
        pin_info: &PinInfo,
        pos: eframe::egui::Pos2,
        size: f32,
        snarl_style: &egui_snarl::ui::SnarlStyle,
        style: &eframe::egui::Style,
        painter: &eframe::egui::Painter,
        scale: f32,
        snarl: &Snarl<NodeOfThought>,
    ) -> eframe::egui::Color32 {
        let _ = (pin, snarl);
    
        pin_info.draw(pos, size, snarl_style, style, painter, scale)
    }
    
    fn draw_background(
        &mut self,
        background: Option<&egui_snarl::ui::BackgroundPattern>,
        viewport: &egui_snarl::ui::Viewport,
        snarl_style: &egui_snarl::ui::SnarlStyle,
        style: &eframe::egui::Style,
        painter: &eframe::egui::Painter,
        snarl: &Snarl<NodeOfThought>,
    ) {
        let _ = snarl;
    
        if let Some(background) = background {
            background.draw(viewport, snarl_style, style, painter);
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
