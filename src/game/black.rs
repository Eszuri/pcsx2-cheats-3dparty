use crate::{
    components::button_action::{apply_cheat, switch_freeze},
    style::base_style,
};
use eframe::egui::Ui;

pub fn black_data(ui: &mut Ui) {
    ui.vertical_centered(|ui: &mut Ui| ui.heading("GAME"));
    ui.group(|ui| {
        // address Health player
        ui.label(base_style("Address Health Player"));
        ui.horizontal(|ui| {
            switch_freeze(ui, "Frezze");
            apply_cheat(ui, 0x005A8DA8, 1144750080, false, "Apply Cheat");
        });
    });
}
