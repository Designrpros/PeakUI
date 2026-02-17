use super::super::app::{App, IntelligenceState, InteractionState, LabState, ShellState};
use std::sync::Arc;

/// ViewState is the single source of truth for all UI state in the PeakUI reference implementation.
///
/// It acts as the "Digital Nervous System" of the application, enabling:
/// 1. **State Optimization**: All lab and app states are consolidated here.
/// 2. **Performance**: Large collections (chat messages, DB records) are wrapped in `Arc` for pointer-speed cloning.
/// 3. **AI Introspection**: By deriving `Serialize`, the entire state can be dumped to JSON for AI agents to understand the app's internal "thought process."
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ViewState {
    pub shell: ShellState,
    pub intelligence: IntelligenceState,
    pub labs: LabState,
    pub interaction: InteractionState,

    // Services / Global Data
    pub icon_limit: usize,
    #[serde(skip, default)]
    pub db_records: Arc<Vec<crate::core::SemanticRecord>>,
}

impl ViewState {
    pub fn new(app: &App) -> Self {
        Self {
            shell: app.shell.clone(),
            intelligence: app.intelligence.clone(),
            labs: app.labs.clone(),
            interaction: app.interaction.clone(),
            icon_limit: app.icon_limit,
            #[cfg(feature = "neural")]
            db_records: app.db.get_all(),
            #[cfg(not(feature = "neural"))]
            db_records: Arc::new(Vec::new()),
        }
    }
}
