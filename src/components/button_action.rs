use crate::{
    ipc::connect_pine::{
        check_games_is_running, reading_value, rewrite_value, rewrite_value_freeze,
    },
    style::base_style,
};
use eframe::egui::{Button, Color32, CursorIcon, Id, RichText, Ui};
use std::sync::Mutex;

static LIST_BUTTON_ACTION: Mutex<Vec<(String, Id)>> = Mutex::new(Vec::new());

fn serial_game() -> String {
    let (_, serial) = check_games_is_running();
    return serial;
}

pub fn apply_cheat_freeze(
    ui: &mut Ui,
    address: u32,
    value: u32,
    serial_game: &str,
    text: &str,
) -> bool {
    let id: Id = ui.make_persistent_id((address, serial_game));
    let mut is_active = ui.data_mut(|d| *d.get_persisted_mut_or_default(id));
    rewrite_value_freeze(address, value, is_active);
    let bg = if is_active {
        Color32::from_rgb(34, 127, 94)
    } else {
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

pub fn apply_cheat_freeze_multiple_address(
    ui: &mut Ui,
    targets: &[(u32, u32)],
    serial_game: &str,
    text: &str,
) -> bool {
    let id: Id = ui.make_persistent_id((targets, serial_game));
    let mut is_active = ui.data_mut(|d| *d.get_persisted_mut_or_default(id));
    let bg = if is_active {
        for &(addr, val) in targets {
            rewrite_value_freeze(addr, val, is_active);
            rewrite_value(addr, val);
        }
        Color32::from_rgb(34, 127, 94)
    } else {
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

pub fn apply_cheat_multiple_address(ui: &mut Ui, targets: &[(u32, u32)], text: &str) {
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
        for &(addr, val) in targets {
            rewrite_value(addr, val);
        }
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

pub fn group_cheats_normal(ui: &mut Ui, title: &str, address: u32, value: u32) {
    let reading_value_decimal_bytes = reading_value(address, false);
    let reading_value_decimal_float = reading_value(address, true);

    let reading_value_hexadecimal_bytes = reading_value_decimal_bytes.parse::<u32>().unwrap_or(0);
    let reading_value_hexadecimal_float = reading_value_decimal_float.parse::<f32>().unwrap_or(0.0);

    ui.group(|ui| {
        ui.label(base_style(title));
        ui.separator();
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            let freeze_condition =
                apply_cheat_freeze(ui, address, value, &serial_game(), "Apply Freeze");
            ui.add_enabled_ui(!freeze_condition, |ui| {
                apply_cheat(ui, address, value, "Apply Cheat\n(one - time)");
            });
        });
        ui.add_space(5.0);
        ui.label(format!(
            "4 Bytes: {reading_value_decimal_bytes} | {reading_value_hexadecimal_bytes:08X}"
        ));
        ui.label(format!(
            "Float: {} | {:08X}",
            reading_value_decimal_float,
            reading_value_hexadecimal_float.to_bits()
        ));
    });
}

pub fn group_cheats_normal_multiple_address(ui: &mut Ui, title: &str, targets: &[(u32, u32)]) {
    ui.group(|ui| {
        ui.label(base_style(title));
        ui.separator();
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            let freeze_condition =
                apply_cheat_freeze_multiple_address(ui, targets, &serial_game(), "Apply Freeze");
            ui.add_enabled_ui(!freeze_condition, |ui| {
                apply_cheat_multiple_address(ui, targets, "Apply Cheat\n(one - time)");
            });
        });
        ui.add_space(5.0);
        ui.label(format!("No information for multiple address"));
    });
}
