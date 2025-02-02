use std::sync::Arc;

use eframe::egui::{self, Color32, Context, RichText, TextStyle};

pub const COLOR_DISABLED: Color32 = Color32::from_rgb(189, 189, 189);

macro_rules! add_ttf {
    ($fonts: expr, $fname: literal, $ftype: literal) => {
        $fonts.font_data.insert(
            concat!($fname, "-", $ftype).to_owned(),
            Arc::new(egui::FontData::from_static(include_bytes!(concat!(
                "../../", $fname, "/", $fname, "-", $ftype, ".ttf"
            )))),
        );
    };
}

pub fn set_font(context: &Context) {
    let mut fonts = egui::FontDefinitions::default();

    add_ttf!(fonts, "MapleMonoCN", "Regular");
    add_ttf!(fonts, "MapleMonoCN", "Bold");
    add_ttf!(fonts, "MapleMonoCN", "Light");
    add_ttf!(fonts, "MapleMonoCN", "Medium");
    add_ttf!(fonts, "MapleMonoCN", "SemiBold");
    add_ttf!(fonts, "MapleMonoCN", "Thin");
    add_ttf!(fonts, "MapleMonoCN", "BoldItalic");
    add_ttf!(fonts, "MapleMonoCN", "Italic");
    add_ttf!(fonts, "MapleMonoCN", "LightItalic");
    add_ttf!(fonts, "MapleMonoCN", "MediumItalic");
    add_ttf!(fonts, "MapleMonoCN", "SemiBoldItalic");
    add_ttf!(fonts, "MapleMonoCN", "ThinItalic");
    add_ttf!(fonts, "MapleMonoCN", "ExtraBold");
    add_ttf!(fonts, "MapleMonoCN", "ExtraBoldItalic");
    add_ttf!(fonts, "MapleMonoCN", "ExtraLight");
    add_ttf!(fonts, "MapleMonoCN", "ExtraLightItalic");
    add_ttf!(fonts, "NotoColorEmoji", "Regular");

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "MapleMonoCN-Regular".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("MapleMonoCN-Regular".to_owned());

    egui_material_icons::add_to_fonts(&mut fonts);

    context.set_fonts(fonts);
}

pub fn label_text(text: impl AsRef<str>) -> RichText {
    RichText::new(text.as_ref()).text_style(TextStyle::Button)
}

pub fn heading_text(text: impl AsRef<str>) -> RichText {
    RichText::new(text.as_ref()).heading().text_style(TextStyle::Heading)
}

pub fn body_text(text: impl AsRef<str>) -> RichText {
    RichText::new(text.as_ref()).text_style(TextStyle::Body)
}
