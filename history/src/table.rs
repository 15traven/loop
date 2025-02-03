use eframe::egui::{
    self, 
    Layout, 
    Align,
    widget_text::RichText,
    containers::scroll_area::ScrollBarVisibility
};
use egui_extras::{TableBuilder, Column};

use super::types::HistoryRecord;

#[derive(Default)]
pub struct HistoryTable {}

impl HistoryTable {
    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        data: Vec<HistoryRecord>
    ) {
        TableBuilder::new(ui)
            .striped(true)
            .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
            .cell_layout(Layout::left_to_right(Align::Center))
            .column(Column::remainder().resizable(false))
            .column(Column::auto().resizable(false))
            .body(|mut body| {
                for record in data {
                    let rect = body.row(30.0, |mut row| {
                        row.col(|ui| {
                            ui.label(RichText::new("timestamp")
                                .size(15.0)
                            );
                        });
                        row.col(|ui| {
                            ui.add_space(20.0);
                            ui.label(RichText::new("status")
                                .size(15.0)
                            );
                        });
                    });
                }
            });
    }
}