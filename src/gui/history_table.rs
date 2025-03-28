use eframe::egui::{
    self, 
    Layout, 
    Align,
    widget_text::RichText,
    containers::scroll_area::ScrollBarVisibility
};
use egui_extras::{TableBuilder, Column};

use crate::types::HistoryRecord;

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
            .scroll_bar_visibility(ScrollBarVisibility::VisibleWhenNeeded)
            .cell_layout(Layout::left_to_right(Align::Center))
            .column(Column::remainder().resizable(false))
            .column(Column::auto().resizable(false))
            .column(Column::auto().resizable(false))
            .column(Column::auto().resizable(false))
            .column(Column::remainder().resizable(false))
            .body(|mut body| {
                for record in data {
                    body.row(30.0, |mut row| {
                        row.col(|_| {});
                        row.col(|ui| {
                            ui.label(RichText::new(
                                record.timestamp.date_naive()
                                    .to_string(),
                            ).size(15.0));
                        });
                        row.col(|ui| {
                            ui.add_space(5.0);
                            ui.label(RichText::new(
                                record.timestamp.time()
                                    .format("%H:%M")
                                    .to_string()
                            ).size(15.0));
                        });
                        row.col(|ui| {
                            ui.add_space(50.0);
                            record.status.as_icon(ui);
                        });
                        row.col(|_| {});
                    });
                }
            });
    }
}