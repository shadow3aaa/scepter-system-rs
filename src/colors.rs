use eframe::egui::Color32;

pub fn primer(color: Color32, dark_mode: bool) -> Color32 {
    let adjustment_factor = |c: u8| -> u8 {
        if dark_mode {
            f32::from(c) * 0.8
        } else {
            f32::from(c) * 1.2
        }
        .clamp(0.0, 255.0) as u8
    };

    Color32::from_rgb(
        adjustment_factor(color.r()),
        adjustment_factor(color.g()),
        adjustment_factor(color.b()),
    )
}

pub const fn conatiner_background(dark_mode: bool) -> Color32 {
    if dark_mode {
        // #424242
        Color32::from_rgb(66, 66, 66)
    } else {
        // #EEEEEE
        Color32::from_rgb(238, 238, 238)
    }
}

pub const fn pin(dark_mode: bool) -> Color32 {
    if dark_mode {
        // #FFEB3B
        Color32::from_rgb(255, 235, 59)
    } else {
        // #B0BEC5
        Color32::from_rgb(176, 190, 197)
    }
}

pub const fn editor(dark_mode: bool) -> Color32 {
    if dark_mode {
        // #212121
        Color32::from_rgb(33, 33, 33)
    } else {
        // #FAFAFA
        Color32::from_rgb(250, 250, 250)
    }
}
