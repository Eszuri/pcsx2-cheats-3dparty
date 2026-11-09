use eframe::egui::{self, Button, Color32, RichText};

pub fn switch_button(ui: &mut egui::Ui, label: &str) -> bool {
    let id = ui.next_auto_id();
    let mut is_active = ui.data_mut(|d| *d.get_temp_mut_or_default(id));
    let bg = if is_active {
        Color32::from_rgb(34, 127, 94)
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
