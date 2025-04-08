use egui::{Color32, CornerRadius, Frame, Response, RichText, Shadow, TextStyle, Ui};

pub fn history_card(ui: &mut Ui, label: &str, selected: bool) -> Response {
    let available_width = ui.available_width();
    Frame::NONE
        .corner_radius(CornerRadius::same(10))
        .inner_margin(10)
        .shadow(if selected {
            Shadow {
                offset: [2, 2],                       // 向右下偏移 2 像素
                blur: 0,                              // 模糊程度
                spread: 1,                            // 阴影向四周扩展
                color: Color32::from_black_alpha(40), // 带透明度的黑色
            }
        } else {
            Shadow::NONE
        })
        .show(ui, |ui| {
            ui.set_width(available_width);
            ui.label(RichText::new(label).text_style(TextStyle::Button));
        })
        .response
}
