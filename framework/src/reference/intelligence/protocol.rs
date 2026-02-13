use crate::prelude::*;
use crate::reference::app::RenderMode;
use crate::reference::AppPage;
pub use peak_theme::{PeakTheme, ThemeTone};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub enum Action {
    #[serde(alias = "navigate", alias = "Navigate")]
    Navigate(AppPage),
    #[serde(alias = "setbuttonvariant", alias = "SetButtonVariant")]
    SetButtonVariant(Variant),
    #[serde(alias = "setbuttonintent", alias = "SetButtonIntent")]
    SetButtonIntent(Intent),
    #[serde(alias = "setthemekind", alias = "SetThemeKind")]
    SetThemeKind(PeakTheme),
    #[serde(alias = "setthemetone", alias = "SetThemeTone")]
    SetThemeTone(ThemeTone),
    #[serde(alias = "setlabmode", alias = "SetLabMode")]
    SetLabMode(RenderMode),
    #[serde(alias = "shell", alias = "Shell")]
    Shell(String), // New: Shell command execution (PROTECTED)
    #[serde(alias = "memorize", alias = "Memorize")]
    Memorize(String), // New: Save information to PeakDB
    #[serde(alias = "teleport", alias = "Teleport")]
    Teleport {
        target: String,
        x: f32,
        y: f32,
        z: f32,
    },
    #[serde(alias = "scale", alias = "Scale")]
    Scale { target: String, factor: f32 },
    #[serde(alias = "rotate", alias = "Rotate")]
    Rotate {
        target: String,
        x: f32,
        y: f32,
        z: f32,
    },
    #[serde(alias = "web_search", alias = "WebSearch")]
    WebSearch(String),
    #[serde(alias = "read_file", alias = "ReadFile")]
    ReadFile(String),
    #[serde(alias = "write_file", alias = "WriteFile")]
    WriteFile { path: String, content: String },
    #[serde(alias = "unknown", alias = "Unknown")]
    Unknown(String),
}

impl Action {
    pub fn is_protected(&self) -> bool {
        match self {
            Action::Shell(_) => true,
            Action::Navigate(AppPage::Roadmap) | Action::Navigate(AppPage::SettingsAI) => true,
            _ => false,
        }
    }

    pub fn protection_reason(&self) -> Option<String> {
        match self {
            Action::Shell(cmd) => Some(format!("Execute shell command: `{}`", cmd)),
            Action::Navigate(AppPage::Roadmap) => {
                Some("Accessing vision-critical roadmap data".to_string())
            }
            Action::Navigate(AppPage::SettingsAI) => {
                Some("Accessing AI configuration and API keys".to_string())
            }
            _ => None,
        }
    }
}

pub struct ActionParser;

impl ActionParser {
    pub fn parse_text(text: &str) -> Vec<Action> {
        let mut actions = Vec::new();

        // Search for JSON blocks in the text
        let mut search_pos = 0;
        while let Some(start_rel) = text[search_pos..].find("[action: ") {
            let start = search_pos + start_rel;
            let start_json = start + "[action: ".len();

            // AI might use ] or )] as terminator. We look for both.
            let mut end = None;
            if let Some(end_rel) = text[start_json..].find(")]") {
                end = Some(start_json + end_rel);
            } else if let Some(end_rel) = text[start_json..].find(']') {
                // FALLBACK: If only ] is found, use it
                end = Some(start_json + end_rel);
            }

            if let Some(end_pos) = end {
                let json_slice = &text[start_json..end_pos].trim();

                if let Ok(action) = serde_json::from_str::<Action>(json_slice) {
                    actions.push(action);
                } else {
                    actions.push(Action::Unknown(json_slice.to_string()));
                }

                search_pos = end_pos + 1;
            } else {
                break;
            }
        }

        actions
    }

    /// Removes all [action: ...] blocks from the text for clean UI display
    pub fn strip_actions(text: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let mut search_pos = 0;

        while let Some(start_rel) = text[search_pos..].find("[action: ") {
            let start = search_pos + start_rel;
            result.push_str(&text[search_pos..start]);

            let start_json = start + "[action: ".len();
            let mut end = None;
            if let Some(end_rel) = text[start_json..].find(")]") {
                end = Some(start_json + end_rel + 2);
            } else if let Some(end_rel) = text[start_json..].find(']') {
                end = Some(start_json + end_rel + 1);
            }

            if let Some(end_pos) = end {
                search_pos = end_pos;
            } else {
                search_pos = start_json; // Move past the tag to avoid infinite loop
                break;
            }
        }

        result.push_str(&text[search_pos..]);
        result.trim().to_string()
    }

    /// Splits text into parts of either plain text, parsed Actions, or Tool Results
    pub fn split_text_and_actions(text: &str) -> Vec<ContentPart> {
        let mut parts = Vec::new();
        let mut search_pos = 0;

        while let Some(start_rel) = text[search_pos..]
            .find("[action: ")
            .or_else(|| text[search_pos..].find("[result:"))
        {
            let start = search_pos + start_rel;

            // Push the text BEFORE the part if not empty
            if start > search_pos {
                parts.push(ContentPart::Text(text[search_pos..start].to_string()));
            }

            let rest = &text[start..];
            if rest.starts_with("[action: ") {
                let start_json = start + "[action: ".len();
                let mut end = None;
                if let Some(end_rel) = text[start_json..].find(")]") {
                    end = Some(start_json + end_rel);
                } else if let Some(end_rel) = text[start_json..].find(']') {
                    end = Some(start_json + end_rel);
                }

                if let Some(end_pos) = end {
                    let json_slice = &text[start_json..end_pos].trim();
                    if let Ok(action) = serde_json::from_str::<Action>(json_slice) {
                        parts.push(ContentPart::Action(action));
                    } else {
                        parts.push(ContentPart::Action(Action::Unknown(json_slice.to_string())));
                    }
                    search_pos = end_pos
                        + if text[end_pos..].starts_with(")]") {
                            2
                        } else {
                            1
                        };
                } else {
                    break;
                }
            } else if rest.starts_with("[result:") {
                let tag_end_rel = text[start..].find(']').unwrap_or(0);
                let tag_end = start + tag_end_rel;
                let tool_name = text[start + "[result:".len()..tag_end].to_string();
                let start_json = tag_end + 1;

                // Find next marker or end of string
                let end_pos = text[start_json..]
                    .find("[action: ")
                    .or_else(|| text[start_json..].find("[result:"))
                    .map(|rel| start_json + rel)
                    .unwrap_or(text.len());

                let json_str = text[start_json..end_pos].trim();
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(json_str) {
                    parts.push(ContentPart::ToolResult(tool_name, value));
                } else {
                    parts.push(ContentPart::Text(text[start..end_pos].to_string()));
                }
                search_pos = end_pos;
            } else {
                search_pos += 1; // Safeguard
            }
        }

        // Push remaining text
        if search_pos < text.len() {
            parts.push(ContentPart::Text(text[search_pos..].to_string()));
        }

        parts
    }
}

#[derive(Debug, Clone)]
pub enum ContentPart {
    Text(String),
    Action(Action),
    ToolResult(String, serde_json::Value),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json_actions() {
        let text = "Hello! [action: {\"Navigate\": \"Introduction\"})] and [action: {\"SetLabMode\": \"Spatial\"})]";
        let actions = ActionParser::parse_text(text);
        assert_eq!(actions.len(), 2);
        match &actions[0] {
            Action::Navigate(AppPage::Introduction) => (),
            _ => panic!("Expected Navigate(Introduction), got {:?}", actions[0]),
        }
        match &actions[1] {
            Action::SetLabMode(RenderMode::Spatial) => (),
            _ => panic!("Expected SetLabMode(Spatial), got {:?}", actions[1]),
        }
    }

    #[test]
    fn test_parse_malformed_json() {
        let text = "Malformed: [action: {\"Navigate\": \"InvalidAppPage\"})]";
        let actions = ActionParser::parse_text(text);
        assert_eq!(actions.len(), 1);
        match &actions[0] {
            Action::Unknown(s) => assert!(s.contains("InvalidAppPage")),
            _ => panic!("Expected Unknown, got {:?}", actions[0]),
        }
    }
    #[test]
    fn test_parse_theme_actions() {
        let text =
            "[action: {\"SetThemeKind\": \"Peak\"})] [action: {\"SetThemeTone\": \"Dark\"})]";
        let actions = ActionParser::parse_text(text);
        assert_eq!(actions.len(), 2);
        match &actions[0] {
            Action::SetThemeKind(PeakTheme::Peak) => (),
            _ => panic!("Expected SetThemeKind(Peak), got {:?}", actions[0]),
        }
        match &actions[1] {
            Action::SetThemeTone(ThemeTone::Dark) => (),
            _ => panic!("Expected SetThemeTone(Dark), got {:?}", actions[1]),
        }
    }

    #[test]
    fn test_parse_theme_actions_robust() {
        let text =
            "[action: {\"setthemekind\": \"peak\"})] [action: {\"setthemetone\": \"dark\"})]";
        let actions = ActionParser::parse_text(text);
        assert_eq!(actions.len(), 2);
        match &actions[0] {
            Action::SetThemeKind(PeakTheme::Peak) => (),
            _ => panic!("Expected SetThemeKind(Peak), got {:?}", actions[0]),
        }
        match &actions[1] {
            Action::SetThemeTone(ThemeTone::Dark) => (),
            _ => panic!("Expected SetThemeTone(Dark), got {:?}", actions[1]),
        }
    }
}
