#![allow(dead_code)]
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

pub fn config() -> &'static Path {
    CONFIG.as_path()
}

pub fn data() -> &'static Path {
    DATA.as_path()
}

static DATA: LazyLock<PathBuf> = LazyLock::new(|| {
    #[cfg(feature = "native")]
    {
        let home = std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("."));
        home.join(".peak").join("intelligence")
    }
    #[cfg(not(feature = "native"))]
    {
        PathBuf::from("/tmp/peak/intelligence")
    }
});

static CONFIG: LazyLock<PathBuf> = LazyLock::new(|| DATA.join("config"));
