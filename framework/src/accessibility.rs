use crate::semantic::SemanticNode;

pub enum AccessibilityEvent {
    NodeCreated(SemanticNode),
    NodeUpdated(SemanticNode),
    NodeRemoved(String),
}

impl Clone for AccessibilityEvent {
    fn clone(&self) -> Self {
        match self {
            Self::NodeCreated(n) => Self::NodeCreated(n.clone()),
            Self::NodeUpdated(n) => Self::NodeUpdated(n.clone()),
            Self::NodeRemoved(id) => Self::NodeRemoved(id.clone()),
        }
    }
}

/// A bridge between PeakUI's semantic tree and platform accessibility APIs.
pub struct AccessibilityBridge {
    pub is_enabled: bool,
    handlers: Vec<Box<dyn Fn(AccessibilityEvent) + Send + Sync>>,
}

impl AccessibilityBridge {
    pub fn new() -> Self {
        Self {
            is_enabled: true,
            handlers: Vec::new(),
        }
    }

    pub fn add_handler<F>(&mut self, handler: F)
    where
        F: Fn(AccessibilityEvent) + Send + Sync + 'static,
    {
        self.handlers.push(Box::new(handler));
    }

    /// Converts a semantic tree into a platform accessible tree.
    pub fn update(&self, root: &SemanticNode) {
        if !self.is_enabled {
            return;
        }

        // Broadcast the root update
        self.broadcast(AccessibilityEvent::NodeUpdated(root.clone()));

        // Recursive bridge to platform accessibility
        self.bridge_node(root);
    }

    fn bridge_node(&self, node: &SemanticNode) {
        if let Some(_a11y) = &node.accessibility {
            // Commented out trace log to fix performance hangs during high-frequency updates
            // log::trace!("A11y Bridge: {:?} label: '{}'", a11y.role, a11y.label);
        }

        for child in &node.children {
            self.bridge_node(child);
        }
    }

    fn broadcast(&self, event: AccessibilityEvent) {
        for handler in &self.handlers {
            handler(event.clone());
        }
    }
}

pub trait Accessible {
    // using generic string for now as placeholder
    fn set_accessibility_role(&mut self, role: String);
    fn set_accessibility_label(&mut self, label: String);
}

/// Standard roles for accessibility nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub enum AccessibilityRole {
    #[default]
    Unknown,
    Button,
    CheckBox,
    Header,
    Link,
    SearchBox,
    Slider,
    SpinButton,
    Switch,
    TextField,
    Label,
    List,
    ListItem,
    Menu,
    MenuItem,
    ProgressBar,
    RadioButton,
    Tab,
    TabList,
    TabPanel,
    Toolbar,
    Tooltip,
    Window,
    Dialog,
    Image,
    Graphic,
    Video,
    Status,
    Icon,
    Text,
    WebView,
    Group,
}

impl std::fmt::Display for AccessibilityRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct AccessibilityNode {
    #[serde(rename = "r")]
    pub role: AccessibilityRole,
    #[serde(rename = "l")]
    pub label: std::borrow::Cow<'static, str>,
    #[serde(rename = "h", skip_serializing_if = "Option::is_none")]
    pub hint: Option<std::borrow::Cow<'static, str>>,
    #[serde(rename = "v", skip_serializing_if = "Option::is_none")]
    pub value: Option<std::borrow::Cow<'static, str>>,
    #[serde(rename = "s", skip_serializing_if = "Vec::is_empty")]
    pub states: Vec<std::borrow::Cow<'static, str>>,
    #[serde(rename = "hd", skip_serializing_if = "is_false")]
    pub is_hidden: bool,
    #[serde(rename = "dis", skip_serializing_if = "is_false")]
    pub is_disabled: bool,
}

fn is_false(b: &bool) -> bool {
    !*b
}
