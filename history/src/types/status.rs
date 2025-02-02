use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum ConnectionStatus {
    Online,
    Offline
}