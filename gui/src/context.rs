use eframe::{
    Frame,
    egui::{
        Context,
        CentralPanel
    }
};

#[derive(Default)]
pub struct HistoryContext {}

impl eframe::App for HistoryContext {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("History");
        });
    }
}