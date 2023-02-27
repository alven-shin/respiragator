#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use app::App;
use tracing_subscriber::FmtSubscriber;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .without_time()
        .with_target(false)
        .with_line_number(true)
        .with_file(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

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
