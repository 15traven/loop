use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::ConnectionStatus;

#[derive(Serialize, Deserialize)]
pub struct HistoryRecord {
    timestamp: DateTime<Utc>,
    status: ConnectionStatus
}

impl HistoryRecord {
    pub fn online() -> Self {
        HistoryRecord {
            timestamp: Utc::now(),
            status: ConnectionStatus::Online
        }
    }

    pub fn offline() -> Self {
        HistoryRecord {
            timestamp: Utc::now(),
            status: ConnectionStatus::Offline
        }
    }
}