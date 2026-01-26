use crate::registry::{AppId, AppMetadata};

pub struct AppRegistry;

impl AppRegistry {
    /// Get metadata for all built-in system applications
    pub fn get_system_apps() -> Vec<AppMetadata> {
        vec![
            AppId::Terminal,
            AppId::Browser,
            AppId::FileManager,
            AppId::Store,
            AppId::Library,
            AppId::Cortex,
            AppId::Turntable,
            AppId::Settings,
            AppId::AppGrid,
            AppId::Editor,
        ]
        .into_iter()
        .map(|id| id.metadata())
        .collect()
    }

    /// Search for applications matching a query (name, category, or description)
    pub fn search_apps(query: &str) -> Vec<AppMetadata> {
        let query = query.to_lowercase();
        Self::get_system_apps()
            .into_iter()
            .filter(|app| {
                app.name.to_lowercase().contains(&query)
                    || app.category.display_name().to_lowercase().contains(&query)
                    || app.description.to_lowercase().contains(&query)
            })
            .collect()
    }

    /// Get all apps categorized by their primary category
    pub fn get_apps_by_category(
    ) -> std::collections::HashMap<crate::registry::AppCategoryPrimary, Vec<AppMetadata>> {
        let mut map = std::collections::HashMap::new();
        for app in Self::get_system_apps() {
            map.entry(app.category).or_insert_with(Vec::new).push(app);
        }
        map
    }

    /// Resolve an AppId from a string (useful for deep-linking or command execution)
    pub fn resolve_id(id_str: &str) -> Option<AppId> {
        id_str.parse::<AppId>().ok()
    }
}
