pub mod bluefruit;

pub struct App {
    pub bluefruit_connected: bool,
    pub resistance_value: u8,
    pub logs: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            bluefruit_connected: false,
            resistance_value: 0,
            logs: String::new(),
        }
    }
}

pub enum Message {
    ConnectionChanged(bool),
    ResistanceValue(u8),
    Log(String),
}
