use include_dir::{include_dir, Dir};

static ICONS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../framework/assets/lucide/icons");

/// Retrieves an icon's SVG content by its Lucide name (e.g., "settings", "user").
pub fn get_icon(name: &str) -> Option<&'static str> {
    let filename = format!("{}.svg", name);
    
    // Safety: include_dir files are embedded in the binary and live for 'static
    ICONS_DIR.get_file(&filename)
        .and_then(|file| std::str::from_utf8(file.contents()).ok())
}

/// Returns a list of all available icon names.
pub fn available_icons() -> Vec<&'static str> {
    ICONS_DIR.files()
        .filter_map(|f| f.path().file_stem())
        .filter_map(|s| s.to_str())
        .collect()
}
