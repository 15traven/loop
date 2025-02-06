use std::fs;

pub mod window;
pub mod table;
pub mod types;

use native_dialog::{
    MessageDialog, 
    MessageType
};
use types::HistoryRecord;

pub fn load() -> std::io::Result<Vec<HistoryRecord>> {
    let json = fs::read_to_string("history.json")?;
    let records: Vec<HistoryRecord> = serde_json::from_str(&json)?;
    
    Ok(records)
}

pub fn save(record: HistoryRecord) -> std::io::Result<()> {
    let mut records = load().unwrap_or_else(|_| Vec::new());
    records.push(record);

    let json = serde_json::to_string_pretty(&records)?;
    fs::write("history.json", json)?;

    Ok(())
}

pub fn clear() -> std::io::Result<()> {
    let confirm = MessageDialog::new()
        .set_type(MessageType::Warning)
        .set_text("Do you want to clear history?")
        .show_confirm()
        .unwrap();

    if confirm {
        fs::write("history.json", "")?;
    }

    Ok(())
}