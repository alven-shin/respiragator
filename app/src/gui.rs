use eframe::{
    egui::{
        self,
        plot::{Line, Plot, PlotPoints},
        RichText, Sense,
    },
    epaint::Vec2,
};

use crate::{App, Message};

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_messages();

        egui::TopBottomPanel::bottom("graph").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let text = format!(
                    "Breaths in the last 10 seconds: {:>2}",
                    self.resistance_data.breaths()
                );
                ui.label(RichText::new(text).size(24.));

                let text = format!(
                    "Breaths per minute: {:>2}",
                    self.resistance_data.breaths() * 6
                );
                ui.label(RichText::new(text).size(24.));
            });
            Plot::new("graph")
                .height(150.)
                .show_x(false)
                // .show_y(false)
                .show_axes([false; 2])
                .allow_drag(false)
                .allow_zoom(false)
                .allow_scroll(false)
                .allow_boxed_zoom(false)
                .show(ui, |plot| {
                    let points: Vec<[f64; 2]> = self
                        .resistance_data
                        .iter()
                        .enumerate()
                        .map(|(idx, val)| [idx as f64, val as f64])
                        .collect();
                    let line = Line::new(points);
                    plot.line(line);

                    let avg = self.resistance_data.average();
                    plot.line(Line::new(PlotPoints::from_explicit_callback(
                        move |_| avg,
                        ..,
                        2,
                    )));
                });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let r = self.resistance_data.last() as f32;
            let size = Vec2::splat(r + 5.0);
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
                Message::ConnectionChanged(_is_connected) => (),
                Message::ResistanceData(data) => {
                    self.resistance_data.extend(data);
                }
            }
        }
    }
}
