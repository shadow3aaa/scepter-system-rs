mod cards;
mod llm_adapters;
mod node;
mod prompt;
mod snarl_viewer;

use std::{collections::VecDeque, sync::Arc};

use eframe::{
    egui::{Color32, CornerRadius, Frame, Margin, Shadow, Stroke, Ui},
    get_value, set_value,
};
use egui::{Button, Layout, Modal, Pos2};
use egui_snarl::{
    ui::{NodeLayout, PinPlacement, SnarlStyle},
    Snarl,
};
use llm_adapters::{
    ollama_adapter::OllamaAdapter, openai_adapter::OpenAIAdapter, LLMAdapterWrapper, LLMAdapters,
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use super::{font::label_text, NavigationController};
use crate::{colors, framework::Page};

use cards::{llm_adapter_card, mind_history_card};
use node::NodeOfThought;
use snarl_viewer::MindViewer;

#[derive(Serialize, Deserialize)]
struct SnarlWrapper {
    #[serde(serialize_with = "crate::serde_utils::arc_rwlock_serde")]
    #[serde(deserialize_with = "crate::serde_utils::arc_rwlock_deserialize")]
    snarl: Arc<RwLock<Snarl<NodeOfThought>>>,
    viewer: MindViewer,
}

#[derive(Default)]
struct AddLLMProvierConfig {
    open_modal: bool,
    provider: Option<LLMAdapterWrapper>,
}

#[derive(Serialize, Deserialize)]
pub struct MindPage {
    side_panel_state: SidePanelState,
    history: VecDeque<SnarlWrapper>,
    #[serde(serialize_with = "crate::serde_utils::arc_rwlock_serde")]
    #[serde(deserialize_with = "crate::serde_utils::arc_rwlock_deserialize")]
    adapters: Arc<RwLock<LLMAdapters>>,
    #[serde(skip)]
    add_llm_provider_temp: AddLLMProvierConfig,
}

impl Default for MindPage {
    fn default() -> Self {
        let snarl = custom_snarl_default();
        let snarl = Arc::new(RwLock::new(snarl));
        let adapter = Arc::new(RwLock::new(LLMAdapters::new()));

        let mut history = VecDeque::new();
        history.push_back(SnarlWrapper {
            snarl: snarl.clone(),
            viewer: MindViewer::new(snarl, adapter.clone()),
        });

        Self {
            side_panel_state: SidePanelState::Mind,
            history,
            adapters: adapter,
            add_llm_provider_temp: AddLLMProvierConfig::default(),
        }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
enum SidePanelState {
    Settings,
    Mind,
}

impl MindPage {
    fn add_llm_adapter_dialog(&mut self, ui: &Ui) {
        if Modal::new("add_llm_adapter".into())
            .show(ui.ctx(), |ui| {
                ui.set_width(500.0);
                ui.set_height(300.0);
                ui.label("Add LLM Adapter");
                ui.menu_button(
                    format!(
                        "Select provider: {}",
                        self.add_llm_provider_temp
                            .provider
                            .as_ref()
                            .map_or("None", |provider| {
                                match provider {
                                    LLMAdapterWrapper::Ollama(_) => "ollama",
                                    LLMAdapterWrapper::OpenAI(_) => "openai compatible",
                                }
                            })
                    ),
                    |ui| {
                        if ui.button("ollama").clicked() {
                            self.add_llm_provider_temp.provider = Some(LLMAdapterWrapper::Ollama(
                                OllamaAdapter::new("llama3.3".to_string(), None),
                            ));
                            ui.close_menu();
                        }

                        if ui.button("openai compatible").clicked() {
                            self.add_llm_provider_temp.provider = Some(LLMAdapterWrapper::OpenAI(
                                OpenAIAdapter::new("gpt-4o".to_string(), None, String::new()),
                            ));
                            ui.close_menu();
                        }
                    },
                );

                if let Some(ref mut provider) = self.add_llm_provider_temp.provider {
                    match provider {
                        LLMAdapterWrapper::Ollama(adapter) => {
                            ui.label("Model");
                            ui.text_edit_singleline(&mut adapter.model);
                        }
                        LLMAdapterWrapper::OpenAI(adapter) => {
                            ui.label("base url");
                            ui.text_edit_singleline(&mut adapter.base_url);
                            ui.label("Model");
                            ui.text_edit_singleline(&mut adapter.model);
                            ui.label("API Key");
                            ui.text_edit_singleline(&mut adapter.api_key);
                        }
                    }
                }

                if ui.button("save").clicked() && self.add_llm_provider_temp.provider.is_some() {
                    self.adapters
                        .write()
                        .adapters
                        .push_front(self.add_llm_provider_temp.provider.take().unwrap());
                    self.add_llm_provider_temp.open_modal = false;
                }
            })
            .should_close()
        {
            self.add_llm_provider_temp.open_modal = false;
        }
    }
}

impl Page for MindPage {
    fn save(&self, storage: &mut dyn eframe::Storage) {
        set_value(storage, "mind_page", self);
    }

    fn on_enter(&mut self, storage: &mut dyn eframe::Storage) {
        if let Some(mind_page) = get_value(storage, "mind_page") {
            *self = mind_page;
        }
    }

    fn side_panel(&mut self, ui: &mut Ui, _: &mut eframe::Frame, _: &mut NavigationController) {
        ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
            ui.vertical(|ui| {
                if icon_button!(
                    ui,
                    "mind",
                    self.side_panel_state == SidePanelState::Mind,
                    24.0
                )
                .clicked()
                {
                    self.side_panel_state = SidePanelState::Mind;
                }

                if icon_button!(
                    ui,
                    "settings",
                    self.side_panel_state == SidePanelState::Settings,
                    24.0
                )
                .clicked()
                {
                    self.side_panel_state = SidePanelState::Settings;
                }
            });
            ui.separator();
            ui.vertical(|ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| match self.side_panel_state {
                        SidePanelState::Settings => {
                            ui.horizontal(|ui| {
                                ui.label(label_text("Settings"));
                                if ui
                                    .add(Button::new("+").fill(Color32::TRANSPARENT).frame(false))
                                    .clicked()
                                {
                                    self.add_llm_provider_temp.open_modal = true;
                                }

                                if self.add_llm_provider_temp.open_modal {
                                    self.add_llm_adapter_dialog(ui);
                                }
                            });

                            let mut new_selected_index = None;
                            let mut remove_index = None;
                            for (index, adapter) in self.adapters.read().adapters.iter().enumerate()
                            {
                                if llm_adapter_card(ui, adapter, index == 0, || {
                                    remove_index = Some(index);
                                })
                                .clicked()
                                {
                                    new_selected_index = Some(index);
                                }
                            }

                            if let Some(index) = remove_index {
                                self.adapters.write().adapters.remove(index);
                            }

                            if let Some(index) = new_selected_index {
                                if index != 0 {
                                    self.adapters.write().set_current_adapter(index);
                                }
                            }
                        }
                        SidePanelState::Mind => {
                            ui.horizontal(|ui| {
                                ui.label(label_text("History"));
                                if ui
                                    .add(Button::new("+").fill(Color32::TRANSPARENT).frame(false))
                                    .clicked()
                                {
                                    let snarl = Arc::new(RwLock::new(custom_snarl_default()));
                                    self.history.push_front(SnarlWrapper {
                                        snarl: snarl.clone(),
                                        viewer: MindViewer::new(snarl, self.adapters.clone()),
                                    });
                                }
                            });

                            let mut new_selected_index = None;
                            for (index, snarl_wapper) in self.history.iter().enumerate() {
                                let snarl = snarl_wapper.snarl.read();
                                let node = snarl.nodes().next().unwrap();
                                if mind_history_card(ui, &node.concept.core, index == 0).clicked() {
                                    new_selected_index = Some(index);
                                }
                            }

                            if let Some(index) = new_selected_index {
                                if index != 0 {
                                    self.history.swap(0, index);
                                }
                            }
                        }
                    });
            });
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
