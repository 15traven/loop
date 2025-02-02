use eframe::{
    NativeOptions,
    egui::ViewportBuilder
};

mod context;
use context::HistoryContext;

pub fn run() -> eframe::Result {
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
            Ok(Box::<HistoryContext>::default())
        })
    )
}