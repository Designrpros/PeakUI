use super::super::app::Message;
use crate::prelude::*;

pub fn view(context: &Context, _is_mobile: bool, _api_key: String) -> PageResult<Message> {
    let _t = context.theme;

    let content = format!(
        "# Peak Intelligence\n\n\
        The assistant interacts with the app by emitting protocol tags in its responses. The framework parses these tags and executes the corresponding messages.\n\n\
        ## The Action Bridge Protocol\n\n\
        The assistant interacts with the app by emitting protocol tags in its responses. The framework parses these tags and executes the corresponding messages.\n\n\
        - `[Action: Navigate(Page)]`\n\
        - `[Action: SetButtonVariant(Variant)]`\n\
        - `[Action: SetThemeTone(Dark)]`\n\n\
        # The Neural Architecture\n\n\
        Peak Intelligence is not just a chatbot; it's a **Semantic Serialization Engine** that transforms visual hierarchy into machine-readable state.\n\n\
        ### 1. The Semantic Tree\n\
        Every component in PeakUI implements the `describe()` method, which outputs a `SemanticNode`. This creates a parallel tree structure optimized for LLM consumption:\n\n\
        - **Role**: Defines the component purpose (e.g., `button`, `sidebar_item`).\n\
        - **Content**: The textual value if applicable.\n\
        - **Neural Tag**: A unique identifier allows the AI to 'target' specific elements.\n\n\
        ### 2. The Action Bridge Protocol\n\
        The framework uses a **Closed-Loop Feedback System**. The assistant emits tags in its natural language responses which are intercepted and executed as internal framework messages:\n\n\
        | Protocol | Parameter | Effect |\n\
        | :--- | :--- | :--- |\n\
        | `Navigate` | `PageName` | Switches the active view |\n\
        | `SetTheme` | `ThemeName` | Hot-swaps the design system |\n\
        | `SetButtonVariant` | `Variant` | Updates the 'Lab' button style |\n\
        | `SetTypographySize` | `Number` | Proportional modular scaling |\n\n\
        ### 3. Contextual Awareness\n\
        By combining the **Semantic Tree** (Eyes) with the **Action Bridge** (Hands), the AI can reason about the UI state and perform multi-step operations like 'Set the landing page to a Dark Terminal theme and show me the Button lab'.\n\n\
        ```rust\n\
        // How a component defines its 'Eyes'\n\
        fn describe(&self, _context: &Context) -> SemanticNode {{ \n\
            SemanticNode {{ accessibility: None, \n\
                role: \"custom_trigger\".into(),\n\
                neural_tag: Some(\"main-action\".into()),\n\
                ..Default::default()\n\
            }}\n\
        }}\n\
        ```"
    );

    PageResult {
        title: "Intelligence".to_string(),
        view: Box::new(ScrollView::new(
            VStack::new_generic()
                .padding(Padding {
                    top: 48.0,
                    right: if context.is_slim() { 24.0 } else { 48.0 },
                    bottom: 48.0,
                    left: if context.is_slim() { 24.0 } else { 48.0 },
                })
                .push(MarkdownView::new(content)),
        )),
        inspector: None,
        search_config: None,
        toolbar_items: Vec::new(),
        sidebar_toggle: Some(Message::ToggleSidebar),
    }
}
