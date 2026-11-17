use std::{
    env::current_exe,
    error::Error,
    sync::atomic::{AtomicBool, Ordering},
};

use crate::{
    components::button_action::apply_cheat_freeze_all, ipc::connect_pine::check_games_is_running,
};
use auto_launch::{AutoLaunch, WindowsEnableMode};
use eframe::egui::{CursorIcon::PointingHand, Id, Ui};

pub static CLOSE_TO_TRAY_ICON_IS_RUNNING: AtomicBool = AtomicBool::new(false);
static STARTUP_ARGS_BOOL: AtomicBool = AtomicBool::new(false);

fn auto_startup(active: bool) -> Result<(), Box<dyn Error>> {
    let path = current_exe()?;
    let app_path = path.to_str().ok_or("Path PCSX2 Cheats 3dParty not found")?;

    let args: &[&str] = if STARTUP_ARGS_BOOL.load(Ordering::Relaxed) {
        &["--tray"]
    } else {
        &[]
    };

    let auto = AutoLaunch::new(
        "PCSX2 Cheats 3dParty",
        app_path,
        WindowsEnableMode::CurrentUser,
        args,
    );

    if active {
        auto.enable()?;
    } else {
        auto.disable()?;
    }

    Ok(())
}

pub fn advanced_mode_setting(ui: &mut Ui) {
    let (_, serial) = check_games_is_running();
    let id: Id = ui.make_persistent_id("always_freeze_all");
    let id_2: Id = ui.make_persistent_id("auto startup");
    let id_2a: Id = ui.make_persistent_id("Minimize to tray icon on starts up");
    let id_3: Id = ui.make_persistent_id("close to tray icon");
    let mut is_active: bool = ui.data_mut(|d| *d.get_persisted_mut_or_default(id));
    let mut is_active_2: bool = ui.data_mut(|d| *d.get_persisted_mut_or_default(id_2));
    let mut is_active_2a: bool = ui.data_mut(|d| *d.get_persisted_mut_or_default(id_2a));
    let mut is_active_3: bool = ui.data_mut(|d| *d.get_persisted_mut_or_default(id_3));
    // always_freeze_all
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
    // end

    // auto startup
    if ui
        .checkbox(
            &mut is_active_2,
            "Auto open application when your computer starts up",
        )
        .on_hover_cursor(PointingHand)
        .changed()
    {
        ui.data_mut(|d| d.insert_persisted(id_2, is_active_2));
    }

    if is_active_2 {
        let _ = auto_startup(true);
        ui.horizontal(|ui| {
            ui.add_space(24.0);
            if ui
                .checkbox(&mut is_active_2a, "Minimize to tray icon")
                .on_hover_cursor(PointingHand)
                .changed()
            {
                ui.data_mut(|d| d.insert_persisted(id_2a, is_active_2a));
                STARTUP_ARGS_BOOL.store(is_active_2a, Ordering::Relaxed);
            }
        });
    } else {
        let _ = auto_startup(false);
    }

    // end

    // close to tray icon
    if ui
        .checkbox(&mut is_active_3, "Close app to tray icon")
        .on_hover_cursor(PointingHand)
        .changed()
    {
        ui.data_mut(|d| d.insert_persisted(id_3, is_active_3));
        CLOSE_TO_TRAY_ICON_IS_RUNNING.store(is_active_3, Ordering::Relaxed);
    }
}
