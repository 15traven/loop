use eframe::{
    egui::{
        self, Button, CentralPanel, 
        Color32, Context, Margin, 
        RichText, Stroke, TopBottomPanel, 
        ViewportBuilder
    }, Frame, NativeOptions
};
use native_dialog::{
    MessageDialog, 
    MessageType
};

use crate::{load, clear, types::HistoryRecord};
use super::table::HistoryTable;
struct HistoryWindow {
    data: Vec<HistoryRecord>
}

impl Default for HistoryWindow {
    fn default() -> Self {
        let mut data: Vec<HistoryRecord> = load().unwrap_or_else(|_| Vec::new());
        data.sort_by_cached_key(|x| std::cmp::Reverse(x.timestamp));

        HistoryWindow { data }
    }
}

impl eframe::App for HistoryWindow {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let mut table = HistoryTable::default();

            if self.data.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.label("No history found");
                });
            } else {
                ui.vertical(|ui| {
                    ui.set_height(ui.available_height() - 36.0);

                    table.render(ui, self.data.clone());
                });
            }
        });

        TopBottomPanel::bottom("bottom_panel")
            .frame(egui::containers::Frame::none()
                .inner_margin(Margin::symmetric(4.0, 6.0))
            )
            .min_height(40.0)
            .show(ctx, |ui| {
                if ui.add_sized(
                    [342.0, 32.0],
                    Button::new(
                        RichText::new("Clear history")
                            .heading()
                            .size(14.0)
                        )
                        .fill(Color32::TRANSPARENT)
                        .stroke(Stroke::default())
                ).clicked() {
                    if !self.data.is_empty() {
                        let confirm = MessageDialog::new()
                            .set_type(MessageType::Warning)
                            .set_text("Do you want to clear history?")
                            .show_confirm()
                            .unwrap();
                        
                        if confirm {
                            let _ = clear();
                            self.data = load().unwrap_or_else(|_| Vec::new());
                        }
                    }
                }
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