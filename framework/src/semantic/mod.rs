use crate::engine::accessibility::AccessibilityNode;
use iced::Task;
use std::borrow::Cow;

/// A semantic representation of a UI component for AI agents and Accessibility.
///
/// `SemanticNode` is a simplified, structured graph of the UI that AI models can
/// consume directly. It eliminates the need for computer vision by
/// exposing roles, labels, and state in a dense JSON format.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct SemanticNode {
    /// The role of the component (e.g., "button", "text_field").
    #[serde(rename = "r")]
    pub role: Cow<'static, str>,
    /// An optional stable identifier for the component.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Cow<'static, str>>,
    /// A human-readable label or name for the component.
    #[serde(rename = "l", skip_serializing_if = "Option::is_none")]
    pub label: Option<Cow<'static, str>>,
    /// The primary text content or value of the component.
    #[serde(rename = "c", skip_serializing_if = "Option::is_none")]
    pub content: Option<Cow<'static, str>>,
    /// Hierarchical children of this node.
    #[serde(rename = "ch", skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<SemanticNode>,
    /// A unique tag for AI-triggered actions.
    #[serde(rename = "t", skip_serializing_if = "Option::is_none")]
    pub neural_tag: Option<Cow<'static, str>>,
    /// Developer-provided documentation for this component.
    #[serde(rename = "d", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<Cow<'static, str>>,
    /// Metadata specifically for platform accessibility APIs.
    #[serde(rename = "a", skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<AccessibilityNode>,
    /// Whether this component is disabled.
    #[serde(rename = "dis", skip_serializing_if = "is_false")]
    pub is_disabled: bool,
    /// Whether this component is hidden.
    #[serde(rename = "hid", skip_serializing_if = "is_false")]
    pub is_hidden: bool,
    /// Whether this component requires elevated "Neural Sudo" permissions to interact with.
    #[serde(rename = "p", skip_serializing_if = "is_false")]
    pub is_protected: bool,
    /// The reason why this component is protected.
    #[serde(rename = "pr", skip_serializing_if = "Option::is_none")]
    pub protection_reason: Option<Cow<'static, str>>,
    /// The Z-depth of the component in spatial/volumetric environments.
    #[serde(rename = "z", skip_serializing_if = "Option::is_none")]
    pub depth: Option<f32>,
    /// The 3D scale of the component.
    #[serde(rename = "s", skip_serializing_if = "Option::is_none")]
    pub scale: Option<[f32; 3]>,
    /// The color of the component in Hex format (e.g., "#FF0000").
    #[serde(rename = "col", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

fn is_false(b: &bool) -> bool {
    !*b
}

impl SemanticNode {
    pub fn new(role: impl Into<Cow<'static, str>>) -> Self {
        Self {
            role: role.into(),
            ..Default::default()
        }
    }

    pub fn with_label(mut self, label: impl Into<Cow<'static, str>>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_content(mut self, content: impl Into<Cow<'static, str>>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn push_child(mut self, child: SemanticNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn extend_children(mut self, children: impl IntoIterator<Item = SemanticNode>) -> Self {
        self.children.extend(children);
        self
    }

    pub fn with_accessibility(mut self, accessibility: AccessibilityNode) -> Self {
        self.accessibility = Some(accessibility);
        self
    }

    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Recursively find a node that matches the predicate
    pub fn find_deep<F>(&self, predicate: &F) -> Option<&SemanticNode>
    where
        F: Fn(&SemanticNode) -> bool,
    {
        if predicate(self) {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.find_deep(predicate) {
                return Some(found);
            }
        }
        None
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemanticRecord {
    pub id: String,
    pub collection: String,
    pub content: String,
    pub vector: Option<Vec<f32>>,
    pub metadata: serde_json::Value,
    pub timestamp: u64,
}

pub trait DataProvider: Send + Sync {
    fn save(&self, record: SemanticRecord) -> Task<std::result::Result<(), String>>;
    fn find(&self, query: String) -> Task<std::result::Result<Vec<SemanticRecord>, String>>;
    fn delete(&self, id: String) -> Task<std::result::Result<(), String>>;

    /// Async version for internal use (e.g. by IntelligenceProvider)
    fn async_find(
        &self,
        query: String,
    ) -> iced::futures::future::BoxFuture<'static, std::result::Result<Vec<SemanticRecord>, String>>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatCompletionMessage {
    pub role: String,
    pub content: String,
}

pub trait IntelligenceProvider: Send + Sync {
    fn chat(
        &self,
        messages: Vec<ChatCompletionMessage>,
    ) -> Task<std::result::Result<String, String>>;

    fn chat_stream(
        &self,
        _messages: Vec<ChatCompletionMessage>,
    ) -> iced::futures::stream::BoxStream<'static, std::result::Result<String, String>> {
        use iced::futures::StreamExt;
        iced::futures::stream::empty().boxed()
    }

    fn reason(
        &self,
        prompt: String,
        context: Vec<SemanticNode>,
    ) -> Task<std::result::Result<String, String>> {
        let context_json = serde_json::to_string(&context).unwrap_or_default();
        let messages = vec![
            ChatCompletionMessage {
                role: "system".to_string(),
                content: format!("UI Context: {}", context_json),
            },
            ChatCompletionMessage {
                role: "user".to_string(),
                content: prompt,
            },
        ];
        self.chat(messages)
    }

    fn execute_tool(
        &self,
        name: String,
        args: serde_json::Value,
    ) -> Task<std::result::Result<serde_json::Value, String>>;

    fn get_system_context(&self) -> String {
        "Peak Intelligence Provider".to_string()
    }
}
