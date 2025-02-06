use eframe::{
    Frame, 
    NativeOptions,
    egui::{
        Context,
        CentralPanel,
        ViewportBuilder
    }
};
use crate::{load, types::HistoryRecord};

use super::table::HistoryTable;

struct HistoryWindow {
    data: Vec<HistoryRecord>
}

impl Default for HistoryWindow {
    fn default() -> Self {
        let data: Vec<HistoryRecord> = load().unwrap_or_else(|_| Vec::new());

        HistoryWindow { data }
    }
}

impl eframe::App for HistoryWindow {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let mut table = HistoryTable::default();
            table.render(ui, self.data.clone());
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
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<HistoryWindow>::default())
        })
    )
}