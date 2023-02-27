use eframe::{
    egui::{self, Sense},
    epaint::Vec2,
};

use crate::{App, Message};

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_messages();

        egui::CentralPanel::default().show(ctx, |ui| {
            let r = self.resistance_value as f32 * 10.;
            let size = Vec2::splat(2.0 * r + 5.0);
            ui.centered_and_justified(|ui| {
                let (rect, _response) = ui.allocate_at_least(size, Sense::hover());
                ui.painter()
                    .circle_filled(rect.center(), r, ui.visuals().text_color());
            });
        });
        ctx.request_repaint();
    }
}

impl App {
    fn check_messages(&mut self) {
        if let Ok(message) = self.rx.try_recv() {
            match message {
                Message::ConnectionChanged(is_connected) => (),
                Message::ResistanceValue(new_value) => {
                    dbg!(new_value);
                    self.resistance_value = new_value;
                }
            }
        }
    }
}
