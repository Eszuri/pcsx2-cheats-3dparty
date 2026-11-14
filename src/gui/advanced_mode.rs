use crate::gui::{advanced_mode_setting, gui::ADVANCED_MODE_IS_RUNNING};
use eframe::egui::{
    self, Button, CentralPanel, CursorIcon::PointingHand, Ui, ViewportBuilder, ViewportId,
};
use std::sync::atomic::AtomicU8;

static MENU_ACTIVE: AtomicU8 = AtomicU8::new(0);

pub fn advanced_mode(ui: &mut Ui) {
    if ADVANCED_MODE_IS_RUNNING.load(std::sync::atomic::Ordering::Relaxed) {
        ui.ctx().show_viewport_immediate(
            ViewportId::from_hash_of("advanced_menu_window"),
            ViewportBuilder::default()
                .with_title("Advanced Menu")
                .with_inner_size([360.0, 260.0]),
            |ui, _class| {
                CentralPanel::default().show(ui, |ui| {
                    let tab = MENU_ACTIVE.load(std::sync::atomic::Ordering::Relaxed);
                    let menu = ["Settings", "Guide", "-"];
                    ui.columns(3, |cols| {
                        for (i, name) in menu.iter().enumerate() {
                            let active = tab == i as u8;
                            let bg = if active {
                                egui::Color32::from_rgb(75, 80, 104)
                            } else {
                                egui::Color32::from_rgb(45, 52, 64)
                            };
                            let btn = Button::new(
                                egui::RichText::new(*name)
                                    .color(egui::Color32::WHITE)
                                    .size(14.0),
                            )
                            .fill(bg)
                            .corner_radius(4.0);
                            if cols[i]
                                .add_sized([cols[i].available_width(), 32.0], btn)
                                .on_hover_cursor(PointingHand)
                                .clicked()
                            {
                                MENU_ACTIVE.store(i as u8, std::sync::atomic::Ordering::Relaxed);
                            }
                        }
                    });
                    ui.add_space(6.0);
                    ui.separator();
                    ui.add_space(6.0);
                    match MENU_ACTIVE.load(std::sync::atomic::Ordering::Relaxed) {
                        0 => {
                            advanced_mode_setting::advanced_mode_setting(ui);
                        }
                        1 => {
                            ui.label("Soon");
                        }
                        _ => {
                            ui.label("Soon");
                        }
                    }
                });
                if ui.input(|i| i.viewport().close_requested()) {
                    ADVANCED_MODE_IS_RUNNING.store(false, std::sync::atomic::Ordering::Relaxed);
                }
            },
        );
    }
}
