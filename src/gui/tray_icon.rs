use eframe::egui;
use std::sync::{
    OnceLock,
    atomic::{AtomicBool, Ordering},
};
use tray_icon::{
    Icon, MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent,
    menu::{Menu, MenuEvent, MenuItem},
};

use crate::gui::advanced_mode_setting::CLOSE_TO_TRAY_ICON_IS_RUNNING;

static CTX: OnceLock<egui::Context> = OnceLock::new();
static WINDOW_VISIBLE: AtomicBool = AtomicBool::new(true);

pub fn init_tray(icon: &egui::IconData, start_in_tray: bool) -> TrayIcon {
    WINDOW_VISIBLE.store(!start_in_tray, Ordering::Relaxed);

    let tray_img = Icon::from_rgba(icon.rgba.clone(), icon.width, icon.height).unwrap();
    let menu = Menu::new();
    let _ = menu.append(&MenuItem::new("Close App", true, None));

    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_menu_on_left_click(false)
        .with_tooltip("PCSX2 Cheats 3dParty")
        .with_icon(tray_img)
        .build()
        .unwrap();

    TrayIconEvent::set_event_handler(Some(|e| {
        if let TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } = e
        {
            if let Some(ctx) = CTX.get() {
                let show = !WINDOW_VISIBLE.load(Ordering::Relaxed);
                WINDOW_VISIBLE.store(show, Ordering::Relaxed);
                ctx.send_viewport_cmd(egui::ViewportCommand::Visible(show));
                if show {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                }
            }
        }
    }));

    MenuEvent::set_event_handler(Some(|_| std::process::exit(0)));

    return tray;
}

pub fn handle_tray(ui: &egui::Ui, start_in_tray: bool) {
    static FIRST_FRAME: AtomicBool = AtomicBool::new(true);
    let _ = CTX.set(ui.ctx().clone());

    if FIRST_FRAME.swap(false, Ordering::Relaxed) && start_in_tray {
        ui.ctx()
            .send_viewport_cmd(egui::ViewportCommand::Visible(false));
    }

    if CLOSE_TO_TRAY_ICON_IS_RUNNING.load(Ordering::Relaxed) {
        if ui.input(|i| i.viewport().close_requested()) {
            WINDOW_VISIBLE.store(false, Ordering::Relaxed);
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::CancelClose);
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Visible(false));
        }
    }
}
