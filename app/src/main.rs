#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::sync::mpsc::{self, SyncSender};
use std::thread;

use app::bluefruit::bluefruit_reciever;
use app::{App, Message};
use egui_macroquad::egui::{self, CentralPanel, ScrollArea};
use egui_macroquad::macroquad;
use egui_macroquad::macroquad::prelude::*;

#[macroquad::main("Project Respirate")]
async fn main() {
    let mut state = App::default();
    let (tx, rx) = mpsc::sync_channel(1);
    background_thread(tx);

    loop {
        if let Ok(message) = rx.try_recv() {
            match message {
                Message::ConnectionChanged(is_connected) => (),
                Message::Log(message) => {
                    state.logs.push_str(&message);
                    state.logs.push_str("\n\n");
                }
                Message::ResistanceValue(new_value) => {
                    state.resistance_value = new_value;
                }
            }
        }
        clear_background(BLACK);

        draw_centered_circle(state.resistance_value);
        draw_egui(&state);
        next_frame().await
    }
}

fn draw_egui(state: &App) {
    egui_macroquad::ui(|ctx| {
        egui::Window::new("Project Respirate").show(ctx, |ui| {
            ui.collapsing("Logs", |ui| {
                ScrollArea::vertical().show(ui, |ui| ui.label(&state.logs));
            });
        });
    });
    egui_macroquad::draw();
}

fn draw_centered_circle(radius: u8) {
    let x_center = screen_width() / 2.;
    let y_center = screen_height() / 2.;
    draw_circle(
        x_center,
        y_center,
        radius as f32 * 10.,
        Color::from_rgba(139, 0, 0, 255),
    );
}

fn background_thread(tx: SyncSender<Message>) {
    thread::spawn(move || {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async { bluefruit_reciever(tx).await });
    });
}
