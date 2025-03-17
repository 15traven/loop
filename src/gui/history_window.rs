use eframe::{
    egui::{
        self, CentralPanel, Context, 
        Margin, TopBottomPanel, Vec2, 
        ViewportBuilder
    }, Frame, NativeOptions
};
use native_dialog::{
    MessageDialog, 
    MessageType
};

use crate::{history::{load, clear}, types::HistoryRecord};
use crate::gui::history_table::HistoryTable;
struct HistoryWindow {
    data: Vec<HistoryRecord>,
    is_sorted: bool,
    sort_button_text: String
}

impl Default for HistoryWindow {
    fn default() -> Self {
        let mut data: Vec<HistoryRecord> = load().unwrap_or_else(|_| Vec::new());
        data.sort_by_cached_key(|x| std::cmp::Reverse(x.timestamp));

        HistoryWindow { 
            data,
            is_sorted: true,
            sort_button_text: "⬆".to_string()
        }
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
            .frame(egui::containers::Frame::new()
                .inner_margin(Margin::symmetric(4, 6))
            )
            .min_height(40.0)
            .show(ctx, |ui| {
                ui.horizontal_top(|ui| {
                    ui.allocate_ui(
                        Vec2::new(ui.style().spacing.item_spacing.x, 0.0), 
                        |ui| {
                            if ui.button(&self.sort_button_text).clicked() {
                                if self.is_sorted {
                                    self.data.sort_by_cached_key(|x| x.timestamp);
                                    self.sort_button_text = "⬇".to_string();
                                } else {
                                    self.data.sort_by_cached_key(|x| std::cmp::Reverse(x.timestamp));
                                    self.sort_button_text = "⬆".to_string();
                                }

                                self.is_sorted = !self.is_sorted;
                            }
                        });
                    ui.vertical_centered_justified(|ui| {
                        if ui.button("Clear history").clicked() {
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
                });
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