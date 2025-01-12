use std::sync::Arc;
use redb::{Database, TableDefinition};

const TABLE: TableDefinition<&str, bool> = TableDefinition::new("preferences");

#[derive(Clone)]
pub struct Preferences {
    db: Arc<Database>
}

impl Preferences {
    pub fn new() -> Self {
        let db = Database::create("preferences.redb")
            .expect("Failed to create database");

        let txn = db.begin_write().expect("Failed to begin write transaction");
        {
            let _table = txn.open_table(TABLE).expect("Failed to open table");
        }
        let _ = txn.commit();

        Self { 
            db: Arc::new(db)     
        }
    }

    pub fn set_intial_values(&self) {
        if !self.exists("notifications") {
            self.save_preference("notifications", true);
        }

        if !self.exists("autolaunch") {
            self.save_preference("autolaunch", true);
        }
    }

    pub fn save_preference(&self, key: &str, value: bool) {
        let txn = self.db.begin_write().expect("Failed to begin write transaction");
        {
            let mut table = txn.open_table(TABLE).expect("Failed to open table");
            let _ = table.insert(key, value);
        }
        let _ = txn.commit();
    }

    pub fn load_preference(&self, key: &str) -> bool {
        let txn = self.db.begin_read().expect("Failed to begin read transaction");
        let table = txn.open_table(TABLE).expect("Failed to open table");

        table.get(key).expect("Failed to read preference").unwrap().value()
    }

    pub fn toggle_preference(&self, key: &str) {
        let current_value = self.load_preference(key);

        match current_value {
            true => {
                self.save_preference(key, false);
            }
            false => {
                self.save_preference(key, true);
            }
        }
    }

    fn exists(&self, key: &str) -> bool {
        let txn = self.db.begin_read().expect("Failed to begin read transaction");
        let table = txn.open_table(TABLE).expect("Failed to open table");
        table.get(key).expect("Failed to read preference").is_some()
    }
}