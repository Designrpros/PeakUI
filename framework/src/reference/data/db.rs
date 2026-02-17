use crate::core::DataProvider;
use crate::semantic::SemanticRecord;
use iced::Task;
use peak_db::{PeakDB, PeakRecord};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct PeakDBBridge {
    // In-memory cache for fast access
    storage: Arc<Mutex<Arc<Vec<SemanticRecord>>>>,
    // Real SQLite connection (Global PeakDB)
    db: Arc<tokio::sync::RwLock<Option<PeakDB>>>,
}

impl Default for PeakDBBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl PeakDBBridge {
    pub fn new() -> Self {
        let storage = Arc::new(Mutex::new(Arc::new(Vec::new())));
        let db = Arc::new(tokio::sync::RwLock::new(None));

        let slf = Self { storage, db };

        // Load initial state (from JSON fallback if exists)
        slf.load();

        // Spawn background initialization for Global SQLite
        let storage_clone = slf.storage.clone();
        let db_clone = slf.db.clone();

        #[cfg(not(target_arch = "wasm32"))]
        {
            tokio::spawn(async move {
                eprintln!("[PeakDB] Starting initialization...");
                let _ = std::fs::create_dir_all(".peak");
                // Use global peak.db
                match PeakDB::connect("sqlite:.peak/global_memory.db").await {
                    Ok(pdb) => {
                        eprintln!("[PeakDB] Connected to SQLite.");
                        // Check if we need to migrate from JSON
                        let to_migrate = {
                            if let Ok(storage) = storage_clone.lock() {
                                if !storage.is_empty() {
                                    eprintln!(
                                        "[PeakDB] Found {} records to migrate.",
                                        (*storage).len()
                                    );
                                    Some((*storage).clone())
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        };

                        if let Some(records) = to_migrate {
                            // Migrate each record to Global SQLite
                            for record in records.iter() {
                                let _ = pdb
                                    .save_record(PeakRecord {
                                        id: record.id.clone(),
                                        collection: record.collection.clone(),
                                        content: record.content.clone(),
                                        vector: record.vector.clone(),
                                        metadata: record.metadata.clone(),
                                        timestamp: record.timestamp,
                                    })
                                    .await;
                            }

                            // Optional: Rename memory.json to memory.json.bak
                            let _ = std::fs::rename(".peak/memory.json", ".peak/memory.json.bak");
                            eprintln!("[PeakDB] Migration complete.");
                        }

                        // Sync: Load everything from Global SQLite
                        if let Ok(all_records) = pdb.find_keyword("").await {
                            // "" gets everything
                            if let Ok(mut storage) = storage_clone.lock() {
                                let mut loaded = Vec::new();
                                for r in all_records {
                                    loaded.push(SemanticRecord {
                                        id: r.id,
                                        collection: r.collection,
                                        content: r.content,
                                        vector: r.vector,
                                        metadata: r.metadata,
                                        timestamp: r.timestamp,
                                    });
                                }
                                let count = loaded.len();
                                *storage = Arc::new(loaded);
                                eprintln!("[PeakDB] Synced {} records from SQLite.", count);
                            }
                        }

                        let mut db_write = db_clone.write().await;
                        *db_write = Some(pdb);
                    }
                    Err(e) => {
                        eprintln!("[PeakDB] Connection failed: {}", e);
                    }
                }
            });
        }

        slf
    }

    pub fn load(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Ok(content) = std::fs::read_to_string(".peak/memory.json") {
                if let Ok(records) = serde_json::from_str::<Vec<SemanticRecord>>(&content) {
                    if let Ok(mut storage) = self.storage.lock() {
                        *storage = Arc::new(records);
                    }
                }
            }
        }
    }

    pub fn save_to_disk(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = std::fs::create_dir_all(".peak");
            if let Ok(storage) = self.storage.lock() {
                if let Ok(content) = serde_json::to_string_pretty(&**storage) {
                    let _ = std::fs::write(".peak/memory.json", content);
                }
            }
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
        let db_handle = self.db.clone();

        Task::perform(
            async move {
                // Update in-memory cache first
                if let Ok(mut mutex) = storage_handle.lock() {
                    let mut new_vec = (**mutex).clone();
                    if let Some(pos) = new_vec.iter().position(|r| r.id == record.id) {
                        new_vec[pos] = record.clone();
                    } else {
                        new_vec.push(record.clone());
                    }
                    *mutex = Arc::new(new_vec);
                }

                // Persist to Global SQLite
                let db_read = db_handle.read().await;
                if let Some(pdb) = &*db_read {
                    let _ = pdb
                        .save_record(PeakRecord {
                            id: record.id,
                            collection: record.collection,
                            content: record.content,
                            vector: record.vector,
                            metadata: record.metadata,
                            timestamp: record.timestamp,
                        })
                        .await;
                }
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
                let query_lower = query.to_lowercase();

                let mut results: Vec<SemanticRecord> = db
                    .iter()
                    .filter(|r| r.content.to_lowercase().contains(&query_lower))
                    .cloned()
                    .collect();

                if results.len() < 5 {
                    let mut recent: Vec<_> = db.iter().cloned().collect();
                    recent.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                    for r in recent.into_iter().take(10) {
                        if !results.iter().any(|existing| existing.id == r.id) {
                            results.push(r);
                        }
                    }
                }
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
        let db_handle = self.db.clone();
        let storage_handle = self.storage.clone();

        Box::pin(async move {
            // Try Global SQLite first
            let db_read = db_handle.read().await;
            if let Some(pdb) = &*db_read {
                if let Ok(records) = pdb.find_keyword(&query).await {
                    if !records.is_empty() {
                        return Ok(records
                            .into_iter()
                            .map(|r| SemanticRecord {
                                id: r.id,
                                collection: r.collection,
                                content: r.content,
                                vector: r.vector,
                                metadata: r.metadata,
                                timestamp: r.timestamp,
                            })
                            .collect());
                    }
                }
            }

            // Fallback to in-mem cache
            let db = storage_handle.lock().map_err(|e| e.to_string())?;
            let query_lower = query.to_lowercase();
            let query_keywords: Vec<_> = query_lower
                .split(|c: char| !c.is_alphanumeric())
                .filter(|w| w.len() > 3)
                .collect();

            let mut results: Vec<SemanticRecord> = db
                .iter()
                .filter(|r| {
                    let content_lower = r.content.to_lowercase();
                    content_lower.contains(&query_lower)
                        || query_keywords.iter().any(|w| content_lower.contains(w))
                })
                .cloned()
                .collect();

            if results.len() < 5 {
                let mut recent: Vec<_> = db.iter().cloned().collect();
                recent.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                for r in recent.into_iter().take(10) {
                    if !results.iter().any(|existing| existing.id == r.id) {
                        results.push(r);
                    }
                }
            }
            Ok(results)
        })
    }

    fn async_save(
        &self,
        record: SemanticRecord,
    ) -> iced::futures::future::BoxFuture<'static, std::result::Result<(), String>> {
        let storage_handle = self.storage.clone();
        let db_handle = self.db.clone();

        Box::pin(async move {
            // Update in-memory cache
            if let Ok(mut mutex) = storage_handle.lock() {
                let mut new_vec = (**mutex).clone();
                if let Some(pos) = new_vec.iter().position(|r| r.id == record.id) {
                    new_vec[pos] = record.clone();
                } else {
                    new_vec.push(record.clone());
                }
                *mutex = Arc::new(new_vec);
            }

            // Persist to Global SQLite
            let db_read = db_handle.read().await;
            if let Some(pdb) = &*db_read {
                let _ = pdb
                    .save_record(PeakRecord {
                        id: record.id,
                        collection: record.collection,
                        content: record.content,
                        vector: record.vector,
                        metadata: record.metadata,
                        timestamp: record.timestamp,
                    })
                    .await;
            }
            Ok(())
        })
    }

    fn async_find_semantic(
        &self,
        vector: Vec<f32>,
        limit: usize,
    ) -> iced::futures::future::BoxFuture<'static, std::result::Result<Vec<SemanticRecord>, String>>
    {
        let db_handle = self.db.clone();
        let storage_handle = self.storage.clone();

        Box::pin(async move {
            // Try Global SQLite first for high-quality semantic search
            let db_read = db_handle.read().await;
            if let Some(pdb) = &*db_read {
                if let Ok(records) = pdb.find_semantic(&vector, limit).await {
                    return Ok(records
                        .into_iter()
                        .map(|r| SemanticRecord {
                            id: r.id,
                            collection: r.collection,
                            content: r.content,
                            vector: r.vector,
                            metadata: r.metadata,
                            timestamp: r.timestamp,
                        })
                        .collect());
                }
            }

            // Fallback to in-mem cache if DB is not ready
            let db = storage_handle.lock().map_err(|e| e.to_string())?;

            let mut scored_results: Vec<(f32, SemanticRecord)> = db
                .iter()
                .filter_map(|r| {
                    r.vector.as_ref().map(|v| {
                        let score = cosine_similarity(&vector, v);
                        (score, r.clone())
                    })
                })
                .collect();

            scored_results
                .sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

            let results = scored_results
                .into_iter()
                .take(limit)
                .map(|(_, r)| r)
                .collect();

            Ok(results)
        })
    }
}

fn cosine_similarity(v1: &[f32], v2: &[f32]) -> f32 {
    if v1.len() != v2.len() || v1.is_empty() {
        return 0.0;
    }
    let dot: f32 = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum();
    let n1: f32 = v1.iter().map(|a| a * a).sum::<f32>().sqrt();
    let n2: f32 = v2.iter().map(|a| a * a).sum::<f32>().sqrt();
    if n1 == 0.0 || n2 == 0.0 {
        return 0.0;
    }
    dot / (n1 * n2)
}
