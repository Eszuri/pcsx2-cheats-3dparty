use std::sync::atomic::AtomicBool;

use crate::{
    game::{black::black_data, data_games::DATA_GAMES},
    gui::advanced_mode,
    ipc::{check_pcsx2_process::check_pcsx2_process, connect_pine::check_games_is_running},
};
use eframe::{
    NativeOptions,
    egui::{self, CentralPanel, Color32, CursorIcon::PointingHand, Layout, RichText, viewport},
};

pub static ADVANCED_MODE_IS_RUNNING: AtomicBool = AtomicBool::new(false);

pub fn main_gui() {
    let global_style = egui::Frame::NONE
        .fill(Color32::from_rgb(24, 24, 27))
        .inner_margin(20.0);

    let title_style = egui::Frame::NONE
        .fill(egui::Color32::from_rgb(39, 39, 42))
        .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
        .inner_margin(egui::Margin {
            left: 5,
            right: 5,
            top: 20,
            bottom: 5,
        });

    let render_glow = NativeOptions {
        persist_window: true,
        viewport: viewport::ViewportBuilder::default()
            .with_inner_size([460.0, 600.0])
            .with_min_inner_size([360.0, 600.0])
            .with_max_inner_size([760.0, 600.0])
            .with_maximize_button(false),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_ui_native("PCSX2 Cheats 3dParty", render_glow, move |ctx, _frame| {
        CentralPanel::default().frame(global_style).show(ctx, |ui| {
            let (games_is_running, serial) = check_games_is_running();
            ui.vertical_centered(|ui| {
                title_style.show(ui, |ui| {
                    ui.label(
                        RichText::new("PCSX2 CHEATS")
                            .size(24.0)
                            .color(Color32::GREEN)
                            .strong(),
                    );
                    if let Some(x) = DATA_GAMES.iter().find(|y| y.serial == serial) {
                        ui.label(format!(
                            "{} | {} | {} | {}",
                            x.title, x.serial, x.region, x.crc
                        ));
                    } else {
                        ui.label("No game detected yet");
                    }

                    ui.add_space(15.0);
                    ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                        ui.spacing_mut().button_padding = egui::vec2(18.0, 12.0);
                        ui.visuals_mut().widgets.inactive.bg_stroke =
                            egui::Stroke::new(1.0, egui::Color32::GRAY);
                        let x = ui
                            .button(RichText::new("advanced menu >>"))
                            .on_hover_and_drag_cursor(PointingHand);

                        if x.clicked() {
                            ADVANCED_MODE_IS_RUNNING
                                .store(true, std::sync::atomic::Ordering::Relaxed);
                            ui.ctx().send_viewport_cmd_to(
                                egui::ViewportId::from_hash_of("advanced_menu_window"),
                                egui::ViewportCommand::Focus,
                            );
                        }
                    });
                });
            });
            ui.separator();
            // dinamic data
            if check_pcsx2_process() {
                if games_is_running {
                    if serial == "SLUS-21376" {
                        black_data(ui);
                    }
                }
            }

            // dinamic data end
            ui.ctx().request_repaint();
        });
        advanced_mode::advanced_mode(ctx);
    })
    .expect("Failed to start Application");
}
