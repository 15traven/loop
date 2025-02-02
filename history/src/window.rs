use eframe::{
    Frame, 
    NativeOptions,
    egui::{
        Context,
        CentralPanel,
        ViewportBuilder
    }
};

#[derive(Default)]
struct HistoryWindow {}

impl eframe::App for HistoryWindow {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("History");
        });
    }
}

pub fn show() -> eframe::Result {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([350.0, 450.0])
            .with_resizable(false)
            .with_maximized(false),
        ..Default::default()
    };

    eframe::run_native(
        "Loop - history", 
        options, 
        Box::new(|_| {
            Ok(Box::<HistoryWindow>::default())
        })
    )
}