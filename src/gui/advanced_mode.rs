use eframe::egui::{self, Ui};

use crate::gui::gui::ADVANCED_MODE_IS_RUNNING;

pub fn advanced_mode(ui: &mut Ui) {
    if ADVANCED_MODE_IS_RUNNING.load(std::sync::atomic::Ordering::Relaxed) {
        ui.ctx().show_viewport_immediate(
            egui::ViewportId::from_hash_of("advanced_window"),
            egui::ViewportBuilder::default()
                .with_title("Advanced Menu")
                .with_inner_size([360.0, 260.0]),
            |ui, _class| {
                egui::CentralPanel::default().show(ui, |ui| {
                    ui.heading("Advanced Menu");
                });
                if ui.input(|i| i.viewport().close_requested()) {
                    ADVANCED_MODE_IS_RUNNING.store(false, std::sync::atomic::Ordering::Relaxed);
                }
            },
        );
    }
}
