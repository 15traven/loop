use eframe::egui::{self, Image, Vec2};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum ConnectionStatus {
    Online,
    Offline
}

impl ConnectionStatus {
    pub fn as_icon(&self, ui: &mut egui::Ui) {
        match self {
            ConnectionStatus::Online => {
                ui.add(Image::new(
            egui::include_image!(r"..\..\..\assets\connected_icon.png"
                )).fit_to_exact_size(Vec2::new(18.0, 18.0)));
            },
            ConnectionStatus::Offline => {
                ui.add(Image::new(
            egui::include_image!(r"..\..\..\assets\disconnected_icon.png"
                )).fit_to_exact_size(Vec2::new(18.0, 18.0)));
            },
        }
    }
}