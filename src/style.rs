use eframe::egui::{self, RichText};

pub fn base_style(text: &str) -> RichText {
    RichText::new(text)
        .size(18.0)
        .family(egui::FontFamily::Monospace)
}
