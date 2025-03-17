use serde::{Serialize, Deserialize};
use chrono::{DateTime, offset};
use super::ConnectionStatus;

#[derive(Serialize, Deserialize, Clone)]
pub struct HistoryRecord {
    pub timestamp: DateTime<offset::Local>,
    pub status: ConnectionStatus
}

impl HistoryRecord {
    pub fn online() -> Self {
        HistoryRecord {
            timestamp: offset::Local::now(),
            status: ConnectionStatus::Online
        }
    }

    pub fn offline() -> Self {
        HistoryRecord {
            timestamp: offset::Local::now(),
            status: ConnectionStatus::Offline
        }
    }
}