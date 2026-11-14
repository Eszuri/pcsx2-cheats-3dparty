use crate::{
    components::button_action::apply_cheat_freeze_all, ipc::connect_pine::check_games_is_running,
};
use eframe::egui::{CursorIcon::PointingHand, Id, Ui};

pub fn advanced_mode_setting(ui: &mut Ui) {
    let (_, serial) = check_games_is_running();
    let id: Id = ui.make_persistent_id("always_freeze_all");
    let mut is_active: bool = ui.data_mut(|d| *d.get_persisted_mut_or_default(id));
    if ui
        .checkbox(&mut is_active, "Always freeze all cheat")
        .on_hover_cursor(PointingHand)
        .changed()
    {
        ui.data_mut(|d| d.insert_persisted(id, is_active));
    }

    if is_active {
        apply_cheat_freeze_all(ui, &serial, true);
    }
}
