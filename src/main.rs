#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::gui::gui::main_gui;
mod components;
mod game;
mod gui;
mod ipc;
mod style;
fn main() {
    main_gui();
}
