use crate::{
    components::button_action::{apply_cheat, apply_cheat_freeze},
    ipc::connect_pine::reading_value,
    style::base_style,
};
use eframe::egui::Ui;

fn xyz(ui: &mut Ui, title: &str, address: u32, value: u32) {
    ui.group(|ui| {
        ui.label(base_style(title));
        ui.separator();
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            apply_cheat_freeze(ui, address, value, "Apply Freeze");
            apply_cheat(ui, address, value, "Apply Cheat\n(one - time)");
        });
        ui.add_space(5.0);
        ui.label(format!("4 Bytes: {}", reading_value(address, false)));
        ui.label(format!("Float: {}", reading_value(address, true)));
    });
}

pub fn black_data(ui: &mut Ui) {
    xyz(ui, "Address Health Player", 0x005A8DA8, 1144750080);
    xyz(ui, "Infinite grenade", 0x001551B0, 0000000000);
}
