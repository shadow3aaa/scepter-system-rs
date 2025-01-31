use std::sync::Arc;

use eframe::egui::{self, Context};

macro_rules! add_font {
    ($fonts: expr, $ftype: literal) => {
        $fonts.font_data.insert(
            format!("MapleMono-{}", $ftype),
            Arc::new(egui::FontData::from_static(include_bytes!(
                concat!("../../MapleMono-CN/MapleMonoCN-", $ftype, ".ttf")
            ))),
        );
    };
}

pub fn set_font(context: &Context) {
    let mut fonts = egui::FontDefinitions::default();

    add_font!(fonts, "Regular");
    add_font!(fonts, "Bold");
    add_font!(fonts, "Light");
    add_font!(fonts, "Medium");
    add_font!(fonts, "SemiBold");
    add_font!(fonts, "Thin");
    add_font!(fonts, "BoldItalic");
    add_font!(fonts, "Italic");
    add_font!(fonts, "LightItalic");
    add_font!(fonts, "MediumItalic");
    add_font!(fonts, "SemiBoldItalic");
    add_font!(fonts, "ThinItalic");
    add_font!(fonts, "ExtraBold");
    add_font!(fonts, "ExtraBoldItalic");
    add_font!(fonts, "ExtraLight");
    add_font!(fonts, "ExtraLightItalic");

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "MapleMono-Regular".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("MapleMono-Regular".to_owned());

    context.set_fonts(fonts);
}