//! Search Server for Roblox game — a native GUI for finding Roblox servers.
//!
//! Paste a game link or a place id, and the app lists that game's public
//! servers ordered by ping so you can join the fastest one.

// On Windows, release builds shouldn't open a console window alongside the GUI.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod domain;
mod i18n;
mod net;
mod ui;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1120.0, 760.0])
            .with_min_inner_size([720.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Search Server for Roblox",
        options,
        Box::new(|cc| Ok(Box::new(ui::App::new(cc)))),
    )
}
