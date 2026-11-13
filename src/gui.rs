use crate::{
    game::{black::black_data, data_games::DATA_GAMES},
    ipc::{check_pcsx2_process::check_pcsx2_process, connect_pine::check_games_is_running},
};
use eframe::{
    NativeOptions,
    egui::{self, CentralPanel, Color32, RichText, viewport},
};

pub fn main_gui() {
    let global_style = egui::Frame::NONE
        .fill(Color32::from_rgb(24, 24, 27))
        .inner_margin(20.0);

    let title_style = egui::Frame::NONE
        .fill(egui::Color32::from_rgb(39, 39, 42))
        .stroke(egui::Stroke::new(1.0, egui::Color32::GRAY))
        .inner_margin(24.0);

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
    })
    .expect("Failed to start Application");
}
