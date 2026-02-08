use crate::core::{DataProvider, SemanticRecord};
use iced::Task;
use std::sync::{Arc, Mutex};

pub struct PeakDBBridge {
    // For now, using an in-memory storage, but will eventually connect to PeakDB
    storage: Arc<Mutex<Arc<Vec<SemanticRecord>>>>,
}

impl PeakDBBridge {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(Mutex::new(Arc::new(Vec::new()))),
        }
    }

    pub fn get_all(&self) -> Arc<Vec<SemanticRecord>> {
        self.storage
            .lock()
            .map(|db| db.clone())
            .unwrap_or_else(|_| Arc::new(Vec::new()))
    }
}

impl DataProvider for PeakDBBridge {
    fn save(&self, record: SemanticRecord) -> Task<std::result::Result<(), String>> {
        let storage_handle = self.storage.clone();
        Task::perform(
            async move {
                let mut mutex = storage_handle.lock().map_err(|e| e.to_string())?;
                let mut new_vec = (**mutex).clone();

                if let Some(pos) = new_vec.iter().position(|r| r.id == record.id) {
                    new_vec[pos] = record;
                } else {
                    new_vec.push(record);
                }
                *mutex = Arc::new(new_vec);
                Ok(())
            },
            |res| res,
        )
    }

    fn find(&self, query: String) -> Task<std::result::Result<Vec<SemanticRecord>, String>> {
        let storage_handle = self.storage.clone();
        Task::perform(
            async move {
                let db = storage_handle.lock().map_err(|e| e.to_string())?;
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
        let storage_handle = self.storage.clone();
        Task::perform(
            async move {
                let mut mutex = storage_handle.lock().map_err(|e| e.to_string())?;
                let mut new_vec = (**mutex).clone();
                new_vec.retain(|r| r.id != id);
                *mutex = Arc::new(new_vec);
                Ok(())
            },
            |res| res,
        )
    }

    fn async_find(
        &self,
        query: String,
    ) -> iced::futures::future::BoxFuture<'static, std::result::Result<Vec<SemanticRecord>, String>>
    {
        let storage_handle = self.storage.clone();
        Box::pin(async move {
            let db = storage_handle.lock().map_err(|e| e.to_string())?;
            let results = db
                .iter()
                .filter(|r| {
                    r.content.to_lowercase().contains(&query.to_lowercase())
                        || r.collection.to_lowercase().contains(&query.to_lowercase())
                })
                .cloned()
                .collect();
            Ok(results)
        })
    }
}
