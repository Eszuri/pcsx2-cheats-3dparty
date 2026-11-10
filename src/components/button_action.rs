use eframe::egui::{Button, Color32, RichText, Ui};

use crate::ipc::connect_pine::rewrite_value;

pub fn switch_freeze(ui: &mut Ui, label: &str) -> bool {
    let id = ui.next_auto_id();
    let mut is_active = ui.data_mut(|d| *d.get_temp_mut_or_default(id));
    let bg = if is_active {
        Color32::from_rgb(34, 127, 94)
        // execute address

        // execute address end
    } else {
        Color32::from_rgb(75, 85, 99)
    };
    let btn = Button::new(RichText::new(format!("{label}")).color(Color32::WHITE)).fill(bg);
    if ui.add_sized([130.0, 32.0], btn).clicked() {
        is_active = !is_active;
        ui.data_mut(|d| d.insert_temp(id, is_active));
    }
    is_active
}

pub fn apply_cheat(ui: &mut Ui, address: u32, value: u32, freeze: bool, text: &str) {
    if ui
        .add(Button::new(RichText::new(text).size(16.0)))
        .clicked()
    {
        rewrite_value(address, value, freeze);
    }
}
