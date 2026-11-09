use crate::game::black::black_data;
use eframe::{
    NativeOptions,
    egui::{self, CentralPanel, Color32, RichText},
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
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_ui_native("PCSX2 Cheats 3dParty", render_glow, move |ctx, _frame| {
        CentralPanel::default().frame(global_style).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                title_style.show(ui, |ui| {
                    ui.label(
                        RichText::new("PCSX2 CHEATS")
                            .size(24.0)
                            .color(Color32::GREEN)
                            .strong(),
                    );
                });
            });
            ui.separator();
            // dinamic data
            black_data(ui);
            // dinamic data end
        });
    })
    .expect("Failed to start Application");
}
