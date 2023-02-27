use std::{
    sync::mpsc::{self, Receiver, SyncSender},
    thread,
};

use bluefruit::bluefruit_reciever;

pub mod bluefruit;
pub mod gui;
pub struct App {
    pub bluefruit_connected: bool,
    pub resistance_value: u8,
    pub logs: String,
    pub rx: Receiver<Message>,
}

impl Default for App {
    fn default() -> Self {
        let (tx, rx) = mpsc::sync_channel(1);
        background_thread(tx);
        Self {
            bluefruit_connected: false,
            resistance_value: 0,
            logs: String::new(),
            rx,
        }
    }
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

pub enum Message {
    ConnectionChanged(bool),
    ResistanceValue(u8),
    Log(String),
}
