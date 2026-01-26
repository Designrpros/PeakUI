use iced::widget::image;
#[cfg(feature = "native")]
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
#[cfg(feature = "native")]
use tokio::io::AsyncWriteExt;

// Global Cache State
lazy_static::lazy_static! {
    static ref MEMORY_CACHE: Mutex<HashMap<String, image::Handle>> = Mutex::new(HashMap::new());
}

pub struct ImageLoader;

impl ImageLoader {
    // 1. Get the local cache path for a URL
    fn get_cache_path(url: &str) -> PathBuf {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let hash = hex::encode(Sha256::digest(url.as_bytes()));
            // Use directories crate to find a proper cache location
            let dirs = directories::ProjectDirs::from("com", "peak", "os").unwrap();
            let cache_dir = dirs.cache_dir();
            // Ensure dir exists
            let _ = std::fs::create_dir_all(cache_dir);
            cache_dir.join(format!("{}.jpg", hash))
        }

        #[cfg(not(feature = "native"))]
        {
            let _ = url;
            PathBuf::from("/tmp/cache")
        }
    }

    // 2. Load image (Check Memory -> Check Disk -> Download)
    pub async fn load(url: String) -> Option<image::Handle> {
        if url.is_empty() {
            return None;
        }

        // A. Check Memory (Fastest)
        if let Ok(cache) = MEMORY_CACHE.lock() {
            if let Some(handle) = cache.get(&url) {
                return Some(handle.clone());
            }
        }

        let _path = Self::get_cache_path(&url);

        // B. Check Disk (Fast) - Only on native
        #[cfg(not(target_arch = "wasm32"))]
        {
            if _path.exists() {
                let handle = image::Handle::from_path(_path);
                if let Ok(mut cache) = MEMORY_CACHE.lock() {
                    cache.insert(url.clone(), handle.clone());
                }
                return Some(handle);
            }
        }

        // C. Download from Network (Slow)
        if !url.starts_with("http") {
            // Not a remote URL, skip download
            return None;
        }

        println!("⬇️ Downloading cover: {}", url);
        if let Ok(response) = peak_intelligence::http::HttpClient::get(&url).await {
            let bytes_vec = response.bytes().to_vec();

            #[cfg(not(target_arch = "wasm32"))]
            {
                // Save to Disk
                if let Ok(mut file) = tokio::fs::File::create(&_path).await {
                    let _ = file.write_all(&bytes_vec).await;
                }
            }

            // Return Handle
            let handle = image::Handle::from_bytes(bytes_vec); // Corrected from from_memory
            if let Ok(mut cache) = MEMORY_CACHE.lock() {
                cache.insert(url, handle.clone());
            }
            return Some(handle);
        }

        None
    }
}
