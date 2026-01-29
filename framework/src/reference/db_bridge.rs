use crate::core::{DataProvider, SemanticRecord};
use iced::Task;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct PeakDBBridge {
    // For now, using an in-memory storage, but will eventually connect to PeakDB
    storage: Arc<Mutex<Vec<SemanticRecord>>>,
}

impl PeakDBBridge {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl DataProvider for PeakDBBridge {
    fn save(&self, record: SemanticRecord) -> Task<std::result::Result<(), String>> {
        let storage = self.storage.clone();
        Task::perform(
            async move {
                let mut db = storage.lock().await;
                // If ID exists, replace; otherwise, push
                if let Some(pos) = db.iter().position(|r| r.id == record.id) {
                    db[pos] = record;
                } else {
                    db.push(record);
                }
                Ok(())
            },
            |res| res,
        )
    }

    fn find(&self, query: String) -> Task<std::result::Result<Vec<SemanticRecord>, String>> {
        let storage = self.storage.clone();
        Task::perform(
            async move {
                let db = storage.lock().await;
                // Simple keyword search in content for now
                let results = db
                    .iter()
                    .filter(|r| r.content.contains(&query) || r.collection.contains(&query))
                    .cloned()
                    .collect();
                Ok(results)
            },
            |res| res,
        )
    }

    fn delete(&self, id: String) -> Task<std::result::Result<(), String>> {
        let storage = self.storage.clone();
        Task::perform(
            async move {
                let mut db = storage.lock().await;
                db.retain(|r| r.id != id);
                Ok(())
            },
            |res| res,
        )
    }
}
