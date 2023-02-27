#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use app::App;

fn main() {
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "Project Respirate",
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
    .unwrap();
}
