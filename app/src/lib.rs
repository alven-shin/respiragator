use std::{
    sync::mpsc::{self, Receiver, SyncSender},
    thread,
};

use bluefruit::bluefruit_reciever;
use ringbuffer::RingBuffer;

pub mod bluefruit;
pub mod gui;
pub mod ringbuffer;

pub struct App {
    pub bluefruit_connected: bool,
    // pub data: ConstGenericRingBuffer<u8, 1024>,
    pub resistance_data: RingBuffer<1024>,
    pub rx: Receiver<Message>,
}

impl Default for App {
    fn default() -> Self {
        let (tx, rx) = mpsc::sync_channel(1);
        background_thread(tx);
        Self {
            bluefruit_connected: false,
            resistance_data: RingBuffer::new(),
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
    ResistanceData(Vec<u8>),
}
