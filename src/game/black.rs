use crate::{components::button_action::switch_button, style::base_style};
use eframe::egui::Ui;

pub fn black_data(ui: &mut Ui) {
    ui.group(|ui| {
        ui.label(base_style("Address Health Player"));
        ui.horizontal(|ui| {
            switch_button(ui, "Frezze");
            switch_button(ui, "Apply");
        });
    });
}
