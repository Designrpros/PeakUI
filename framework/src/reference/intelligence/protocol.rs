use crate::prelude::*;
use crate::reference::app::RenderMode;
use crate::reference::model::Page;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Action {
    Navigate(Page),
    SetButtonVariant(Variant),
    SetButtonIntent(Intent),
    SetThemeKind(String), // We use String to match the loose parsing for now
    SetThemeTone(String),
    SetLabMode(RenderMode),
    Shell(String),    // New: Shell command execution (PROTECTED)
    Memorize(String), // New: Save information to PeakDB
    Unknown(String),
}

impl Action {
    pub fn is_protected(&self) -> bool {
        match self {
            Action::Shell(_) => true,
            Action::Navigate(Page::Roadmap) | Action::Navigate(Page::SettingsAI) => true,
            _ => false,
        }
    }

    pub fn protection_reason(&self) -> Option<String> {
        match self {
            Action::Shell(cmd) => Some(format!("Execute shell command: `{}`", cmd)),
            Action::Navigate(Page::Roadmap) => {
                Some("Accessing vision-critical roadmap data".to_string())
            }
            Action::Navigate(Page::SettingsAI) => {
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
        let content = text.to_string();
        let lower_content = content.to_lowercase();

        let action_marker = "[action: ";
        let mut search_pos = 0;

        while let Some(start_rel) = lower_content[search_pos..].find(action_marker) {
            let start = search_pos + start_rel;
            if let Some(open_paren_rel) = lower_content[start..].find('(') {
                let open_paren = start + open_paren_rel;
                if let Some(close_paren_rel) = lower_content[open_paren..].find(")]") {
                    let close_paren = open_paren + close_paren_rel;

                    let action_name = lower_content[start + action_marker.len()..open_paren].trim();
                    let params = &content[open_paren + 1..close_paren].trim();

                    match action_name {
                        "navigate" => {
                            let page: Page = params.to_string().into();
                            actions.push(Action::Navigate(page));
                        }
                        "setbuttonvariant" => {
                            let variant = match params.to_lowercase().as_str() {
                                "solid" => Variant::Solid,
                                "soft" => Variant::Soft,
                                "outline" => Variant::Outline,
                                "ghost" => Variant::Ghost,
                                _ => Variant::Solid,
                            };
                            actions.push(Action::SetButtonVariant(variant));
                        }
                        "setbuttonintent" => {
                            let intent = match params.to_lowercase().as_str() {
                                "primary" => Intent::Primary,
                                "secondary" => Intent::Secondary,
                                "success" => Intent::Success,
                                "warning" => Intent::Warning,
                                "danger" => Intent::Danger,
                                "info" => Intent::Info,
                                "neutral" => Intent::Neutral,
                                _ => Intent::Primary,
                            };
                            actions.push(Action::SetButtonIntent(intent));
                        }
                        "setthemekind" => {
                            actions.push(Action::SetThemeKind(params.to_string()));
                        }
                        "setthemetone" => {
                            actions.push(Action::SetThemeTone(params.to_string()));
                        }
                        "setlabmode" => {
                            let mode = match params.to_lowercase().as_str() {
                                "canvas" => RenderMode::Canvas,
                                "terminal" => RenderMode::Terminal,
                                "neural" => RenderMode::Neural,
                                "spatial" => RenderMode::Spatial,
                                _ => RenderMode::Canvas,
                            };
                            actions.push(Action::SetLabMode(mode));
                        }
                        "shell" => {
                            actions.push(Action::Shell(params.to_string()));
                        }
                        "memorize" => {
                            actions.push(Action::Memorize(params.to_string()));
                        }
                        _ => {
                            actions.push(Action::Unknown(action_name.to_string()));
                        }
                    }

                    search_pos = close_paren + 2;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        actions
    }
}
