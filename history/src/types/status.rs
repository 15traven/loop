use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum ConnectionStatus {
    Online,
    Offline
}

impl ToString for ConnectionStatus {
    fn to_string(&self) -> String {
        match self {
            ConnectionStatus::Online => "online".to_string(),
            ConnectionStatus::Offline => "offline".to_string(),
        }
    }
}