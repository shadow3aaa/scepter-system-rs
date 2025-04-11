use egui::{Frame, Response, RichText, Sense, TextStyle, Ui, UiBuilder};

use crate::colors::{conatiner_background, primer};

use super::llm_adapters::LLMAdapterWrapper;

pub fn mind_history_card(ui: &mut Ui, label: &str, selected: bool) -> Response {
    let dark_mode = ui.style().visuals.dark_mode;

    let filled = if selected {
        primer(conatiner_background(dark_mode), dark_mode)
    } else {
        conatiner_background(dark_mode)
    };

    ui.scope_builder(UiBuilder::new().sense(Sense::click()), |ui| {
        Frame::new()
            .corner_radius(ui.style().visuals.widgets.noninteractive.corner_radius)
            .inner_margin(6)
            .fill(filled)
            .show(ui, |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
                ui.set_width(ui.available_width());
                let label = if label.trim().is_empty() {
                    "New Mind Flow"
                } else {
                    label
                };
                ui.label(RichText::new(label).text_style(TextStyle::Button));
            })
    })
    .response
}

pub fn llm_adapter_card<F>(
    ui: &mut Ui,
    adapter: &LLMAdapterWrapper,
    selected: bool,
    on_delete: F,
) -> Response
where
    F: FnOnce(),
{
    let dark_mode = ui.style().visuals.dark_mode;

    let filled = if selected {
        primer(conatiner_background(dark_mode), dark_mode)
    } else {
        conatiner_background(dark_mode)
    };

    ui.scope_builder(UiBuilder::new().sense(Sense::click()), |ui| {
        let model_name = match adapter {
            LLMAdapterWrapper::Ollama(adapter) => &adapter.model,
            LLMAdapterWrapper::OpenAI(adapter) => &adapter.model,
        };

        let desc_type = match adapter {
            LLMAdapterWrapper::Ollama(_) => "ollama",
            LLMAdapterWrapper::OpenAI(_) => "openai compatible",
        };

        Frame::new()
            .corner_radius(ui.style().visuals.widgets.noninteractive.corner_radius)
            .inner_margin(6)
            .fill(filled)
            .show(ui, |ui| {
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                ui.set_width(ui.available_width());
                ui.label(RichText::new(model_name).text_style(TextStyle::Button));
                ui.label(RichText::new(desc_type).text_style(TextStyle::Small));
                if icon_button!(ui, "delete", false, 13.0).clicked() {
                    on_delete();
                }
            })
    })
    .response
}
