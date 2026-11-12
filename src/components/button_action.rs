use eframe::egui::{Button, Color32, CursorIcon, RichText, Ui};

use crate::ipc::connect_pine::{rewrite_value, rewrite_value_freeze};

pub fn apply_cheat_freeze(ui: &mut Ui, address: u32, value: u32, text: &str) -> bool {
    let id = ui.next_auto_id();
    let mut is_active = ui.data_mut(|d| *d.get_temp_mut_or_default(id));
    let bg = if is_active {
        rewrite_value_freeze(address, value, is_active);
        Color32::from_rgb(34, 127, 94)
    } else {
        rewrite_value_freeze(address, value, is_active);
        Color32::from_rgb(75, 85, 99)
    };
    let btn = Button::new(
        RichText::new(format!("{text}"))
            .color(Color32::WHITE)
            .size(16.5),
    )
    .fill(bg)
    .corner_radius(6.0);

    if ui
        .add_sized([150.0, 40.0], btn)
        .on_hover_cursor(CursorIcon::PointingHand)
        .clicked()
    {
        is_active = !is_active;
        ui.data_mut(|d| d.insert_temp(id, is_active));
    }
    return is_active;
}

pub fn apply_cheat(ui: &mut Ui, address: u32, value: u32, text: &str) {
    let btn = Button::new(RichText::new(text).color(Color32::WHITE).size(15.0))
        .corner_radius(6.0)
        .fill(Color32::from_rgb(75, 85, 99));

    let btn2 = ui
        .add_sized([150.0, 40.0], btn)
        .on_hover_cursor(CursorIcon::PointingHand);

    if !ui.is_enabled() && btn2.contains_pointer() {
        ui.ctx().set_cursor_icon(CursorIcon::NotAllowed);
    }

    if btn2.clicked() {
        rewrite_value(address, value);
    }
}
