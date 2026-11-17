use crate::{components::button_action::group_cheats_normal, game::data_games::DATA_GAMES};
use eframe::egui::Ui;

pub fn black_data(ui: &mut Ui) {
    let data = &DATA_GAMES[0].ee_ram;

    group_cheats_normal(ui, data[0].title, data[0].address, data[0].value); // infinite health
    group_cheats_normal(ui, data[1].title, data[1].address, data[1].value); // infinite grenade
    group_cheats_normal(ui, data[2].title, data[2].address, data[2].value); // increase ammo
}
