use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct McpComponent {
    pub name: String,
    pub description: String,
    pub props: HashMap<String, String>,
    pub example: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct McpSchema {
    pub name: String,
    pub version: String,
    pub components: Vec<McpComponent>,
}

pub fn get_framework_schema() -> McpSchema {
    McpSchema {
        name: "PeakUI".to_string(),
        version: "2.0".to_string(),
        components: vec![
            McpComponent {
                name: "VStack".to_string(),
                description: "A vertical layout container.".to_string(),
                props: [
                    ("spacing".to_string(), "f32".to_string()),
                    ("padding".to_string(), "Padding".to_string()),
                ].into_iter().collect(),
                example: "VStack::new().spacing(10.0).push(Text::new(\"Hello\"))".to_string(),
            },
            McpComponent {
                name: "HStack".to_string(),
                description: "A horizontal layout container.".to_string(),
                props: [
                    ("spacing".to_string(), "f32".to_string()),
                    ("padding".to_string(), "Padding".to_string()),
                ].into_iter().collect(),
                example: "HStack::new().spacing(10.0).push(Text::new(\"Left\")).push(Text::new(\"Right\"))".to_string(),
            },
            McpComponent {
                name: "Text".to_string(),
                description: "A text element with semantic styles.".to_string(),
                props: [
                    ("size".to_string(), "f32".to_string()),
                    ("bold".to_string(), "bool".to_string()),
                    ("color".to_string(), "Color".to_string()),
                ].into_iter().collect(),
                example: "Text::new(\"PeakOS\").title1().bold()".to_string(),
            },
            McpComponent {
                name: "Button".to_string(),
                description: "A clickable button component.".to_string(),
                props: [
                    ("variant".to_string(), "Variant".to_string()),
                    ("on_press".to_string(), "Message".to_string()),
                ].into_iter().collect(),
                example: "Button::new(Text::new(\"Click Me\")).on_press(Message::ButtonTapped)".to_string(),
            },
        ],
    }
}
