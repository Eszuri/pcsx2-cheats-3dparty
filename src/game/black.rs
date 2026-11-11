use crate::{
    components::button_action::{apply_cheat, apply_cheat_freeze},
    ipc::connect_pine::reading_value,
    style::base_style,
};
use eframe::egui::Ui;

pub fn black_data(ui: &mut Ui) {
    ui.group(|ui| {
        // address Health player
        ui.label(base_style("Address Health Player"));
        ui.separator();
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            apply_cheat_freeze(ui, 0x005A8DA8, 1144750080, "Apply Freeze");
            apply_cheat(ui, 0x005A8DA8, 1144750080, "Apply Cheat\n(one - time)");
        });
        ui.add_space(5.0);
        ui.label(format!("4 Bytes: {}", reading_value(0x005A8DA8, false)));
        ui.label(format!("Float: {}", reading_value(0x005A8DA8, true)));
    });
}
