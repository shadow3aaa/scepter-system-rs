use eframe::egui::Color32;

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
