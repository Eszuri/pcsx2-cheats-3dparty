use crate::{
    components::button_action::{apply_cheat, apply_cheat_freeze, apply_cheat_freeze_all},
    ipc::connect_pine::{check_games_is_running, reading_value},
    style::base_style,
};
use eframe::egui::Ui;

fn serial_game() -> String {
    let (_, serial) = check_games_is_running();
    return serial;
}

fn xyz(ui: &mut Ui, title: &str, address: u32, value: u32) {
    let reading_value_decimal_bytes = reading_value(address, false);
    let reading_value_decimal_float = reading_value(address, true);

    let reading_value_hexadecimal_bytes = reading_value_decimal_bytes.parse::<u32>().unwrap_or(0);
    let reading_value_hexadecimal_float = reading_value_decimal_float.parse::<f32>().unwrap_or(0.0);

    ui.group(|ui| {
        ui.label(base_style(title));
        ui.separator();
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            let freeze_condition =
                apply_cheat_freeze(ui, address, value, &serial_game(), "Apply Freeze");
            ui.add_enabled_ui(!freeze_condition, |ui| {
                apply_cheat(ui, address, value, "Apply Cheat\n(one - time)");
            });
        });
        ui.add_space(5.0);
        ui.label(format!(
            "4 Bytes: {reading_value_decimal_bytes} | {reading_value_hexadecimal_bytes:08X}"
        ));
        ui.label(format!(
            "Float: {} | {:08X}",
            reading_value_decimal_float,
            reading_value_hexadecimal_float.to_bits()
        ));
    });
}

pub fn black_data(ui: &mut Ui) {
    if ui.button("test on all freeze").clicked() {
        apply_cheat_freeze_all(ui, &serial_game(), true);
    };
    if ui.button("test off all freeze").clicked() {
        apply_cheat_freeze_all(ui, &serial_game(), false);
    };
    xyz(ui, "Address Health Player", 0x005A8DA8, 1144750080);
    xyz(ui, "Infinite grenade", 0x001551B0, 0000000000);
}
