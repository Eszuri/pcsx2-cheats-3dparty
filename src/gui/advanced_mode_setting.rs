use eframe::egui::{CursorIcon::PointingHand, Ui};
use std::sync::atomic::{AtomicBool, Ordering};

pub fn advanced_mode_setting(ui: &mut Ui) {
    static IS_ACTIVE: AtomicBool = AtomicBool::new(false);
    let mut active = IS_ACTIVE.load(Ordering::Relaxed);
    if ui
        .checkbox(&mut active, "Always freeze all cheat")
        .on_hover_cursor(PointingHand)
        .changed()
    {
        IS_ACTIVE.store(active, Ordering::Relaxed);
    }

    if IS_ACTIVE.load(Ordering::Relaxed) {
        println!("active")
    } else {
        println!("not active")
    }
}
