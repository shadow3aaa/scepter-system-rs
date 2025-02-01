use eframe::{
    egui::{Color32, Frame, Id, InnerResponse, Pos2, Rect, ScrollArea, Stroke, Ui, Window},
    emath::Rot2,
};
use uuid::Uuid;

use crate::app::font::{body_text, label_text};

use super::{NavigationController, Page};

pub struct MindPage {
    root_node: NodeOfThought,
}

impl Default for MindPage {
    fn default() -> Self {
        let root_node = NodeOfThought::new("Core Conecpt".to_string(), true, String::new());
        Self { root_node }
    }
}

impl Page for MindPage {
    fn ui(&mut self, ui: &mut Ui, _nav_controller: &mut NavigationController) {
        ScrollArea::both().show(ui, |ui| {
            show_node(ui, &mut self.root_node, None);
        });
    }
}

fn show_node(ui: &mut Ui, node: &mut NodeOfThought, parent_rect: Option<Rect>) {
    let response = node.ui(ui).unwrap().response;
    let current_rect = response.rect;

    if let Some(parent_rect) = parent_rect {
        let parent_pos = parent_rect.center_bottom();
        let current_pos = current_rect.center_top();
        directional_arrow(ui, parent_pos, current_pos, Stroke::new(1.0, Color32::GRAY));
    }

    node.childs.retain(|child| !child.drop_me);

    for child in node.childs() {
        show_node(ui, child, Some(current_rect));
    }
}

struct NodeOfThought {
    id: Uuid,
    is_root: bool,
    drop_me: bool,
    title: Option<String>,
    concept: String,
    clarification: String,
    childs: Vec<NodeOfThought>,
}

impl NodeOfThought {
    fn new(title: impl Into<Option<String>>, is_root: bool, concept: String) -> Self {
        Self {
            id: Uuid::new_v4(),  // Unique ID(in most time)
            is_root,
            drop_me: false,
            title: title.into(),
            concept,
            clarification: String::new(),
            childs: Vec::new(),
        }
    }

    fn ui(&mut self, ui: &mut Ui) -> Option<InnerResponse<Option<()>>> {
        Window::new(if let Some(title) = &self.title {
            format!("{}({})", self.concept, title)
        } else {
            self.concept.clone()
        })
        .title_bar(false)
        .id(Id::new(self.id))
        .show(ui.ctx(), |ui| {
            Frame::none().outer_margin(5.0).show(ui, |ui| {
                ui.label(label_text("Concept"));

                ui.vertical_centered_justified(|ui| {
                    if self.childs.is_empty() {
                        ui.text_edit_multiline(&mut self.concept);
                    } else {
                        ui.horizontal(|ui| {
                            ui.label(body_text(&self.concept));
                        });
                    }
                });

                ui.add_space(5.0);
                if !self.concept.trim().is_empty() {
                    ui.label(label_text("Clarification"));
                    ui.add_space(5.0);
                    ui.vertical_centered_justified(|ui| {
                        ui.text_edit_multiline(&mut self.clarification);
                    });
                }

                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    if !self.concept.trim().is_empty() {
                        if ui.button(label_text("Divergence")).clicked() {
                            // TODO: Call LLM to generate new nodes
                        }

                        ui.add_space(5.0);

                        if ui.button(label_text("Divergence(Manually)")).clicked() {
                            self.add_child(NodeOfThought::new(None, false, String::new()));
                        }
                    }

                    ui.add_space(5.0);

                    if !self.is_root && ui.button(label_text("Forget")).clicked() {
                        self.drop_me = true;
                    }
                });
            });
        })
    }

    fn childs(&mut self) -> impl IntoIterator<Item = &mut NodeOfThought> {
        &mut self.childs
    }

    fn add_child(&mut self, child: NodeOfThought) {
        self.childs.push(child);
    }
}

fn directional_arrow(ui: &mut Ui, start: Pos2, end: Pos2, stroke: impl Into<Stroke>) {
    const TIP_LENGTH: f32 = 6.0;
    const ARROW_SPACING: f32 = 8.0;

    let vec = end - start;
    let dir = vec.normalized();

    let stroke = stroke.into();
    let painter = ui.painter();

    let total_length = vec.length();

    let num_arrows = (total_length / ARROW_SPACING).floor() as usize;

    for i in 0..num_arrows {
        let position = start + dir * (ARROW_SPACING * (i as f32 + 1.0));

        painter.line_segment(
            [
                position,
                position - TIP_LENGTH * (Rot2::from_angle(std::f32::consts::TAU / 10.0) * dir),
            ],
            stroke,
        );
        painter.line_segment(
            [
                position,
                position - TIP_LENGTH * (Rot2::from_angle(-std::f32::consts::TAU / 10.0) * dir),
            ],
            stroke,
        );
    }
}
