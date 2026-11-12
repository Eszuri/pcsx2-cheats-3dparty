use crate::ipc::connect_pine::{check_games_is_running, rewrite_value, rewrite_value_freeze};
use eframe::egui::{Button, Color32, CursorIcon, Id, RichText, Ui};
use std::sync::Mutex;

static LIST_BUTTON_ACTION: Mutex<Vec<(String, Id)>> = Mutex::new(Vec::new());

pub fn apply_cheat_freeze(
    ui: &mut Ui,
    address: u32,
    value: u32,
    serial_game: &str,
    text: &str,
) -> bool {
    let (_, serial) = check_games_is_running();
    let id: Id = ui.make_persistent_id((address, serial));
    let mut is_active = ui.data_mut(|d| *d.get_persisted_mut_or_default(id));
    let bg = if is_active {
        rewrite_value_freeze(address, value, is_active);
        Color32::from_rgb(34, 127, 94)
    } else {
        rewrite_value_freeze(address, value, is_active);
        Color32::from_rgb(75, 85, 99)
    };
    let btn = Button::new(
        RichText::new(format!("{text}"))
            .color(Color32::WHITE)
            .size(16.5),
    )
    .fill(bg)
    .corner_radius(6.0);

    if let Ok(mut list) = LIST_BUTTON_ACTION.lock() {
        if !list.iter().any(|(_, existing_id)| *existing_id == id) {
            list.push((serial_game.to_string(), id));
        }
    }

    if ui
        .add_sized([150.0, 40.0], btn)
        .on_hover_cursor(CursorIcon::PointingHand)
        .clicked()
    {
        is_active = !is_active;
        ui.data_mut(|d| d.insert_persisted(id, is_active));
    }
    return is_active;
}

pub fn apply_cheat(ui: &mut Ui, address: u32, value: u32, text: &str) {
    let btn = Button::new(RichText::new(text).color(Color32::WHITE).size(15.0))
        .corner_radius(6.0)
        .fill(Color32::from_rgb(75, 85, 99));

    let btn2 = ui
        .add_sized([150.0, 40.0], btn)
        .on_hover_cursor(CursorIcon::PointingHand);

    if !ui.is_enabled() && btn2.contains_pointer() {
        ui.ctx().set_cursor_icon(CursorIcon::NotAllowed);
    }

    if btn2.clicked() {
        rewrite_value(address, value);
    }
}

pub fn apply_cheat_freeze_all(ui: &mut Ui, serial_game: &str, aktif: bool) {
    let list = LIST_BUTTON_ACTION.lock().unwrap().clone();
    ui.data_mut(|d| {
        for (g, id) in list {
            if g == serial_game {
                d.insert_persisted(id, aktif);
            }
        }
    });
}
