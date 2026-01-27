use super::super::app::Message;
use super::super::model::Page;
use crate::prelude::*;
use crate::reference::views::ComponentDoc;

pub fn view(_context: &Context, _is_mobile: bool, api_key: String) -> PageResult<Message> {
    let preview = VStack::<Message, IcedBackend>::new_generic()
        .spacing(24.0)
        .push(
            VStack::<Message, IcedBackend>::new_generic()
                .spacing(8.0)
                .push(Text::<IcedBackend>::new("Peak Intelligence").large_title())
                .push(
                    Text::<IcedBackend>::new(
                        "The agentic engine that gives your UI eyes and hands. Peak Intelligence enables autonomous navigation, property manipulation, and real-time LLM integration."
                    )
                    .secondary(),
                ),
        )
        .push(
            VStack::<Message, IcedBackend>::new_generic()
                .spacing(12.0)
                .push(Text::<IcedBackend>::new("The Action Bridge Protocol").title2())
                .push(
                    Text::<IcedBackend>::new(
                        "The assistant interacts with the app by emitting protocol tags in its responses. The framework parses these tags and executes the corresponding messages."
                    )
                    .secondary(),
                )
                .push(
                    Card::new(
                        VStack::<Message, IcedBackend>::new_generic()
                            .spacing(8.0)
                            .push(Text::<IcedBackend>::new("[Action: Navigate(Page)]").bold())
                            .push(Text::<IcedBackend>::new("[Action: SetButtonVariant(Variant)]").bold())
                            .push(Text::<IcedBackend>::new("[Action: SetThemeTone(Dark)]").bold())
                    )
                )
        )
        .push(
            VStack::<Message, IcedBackend>::new_generic()
                .spacing(12.0)
                .push(Text::<IcedBackend>::new("AI Core Configuration").title2())
                .push(
                    Card::new(
                        VStack::<Message, IcedBackend>::new_generic()
                            .spacing(16.0)
                            .push(
                                VStack::<Message, IcedBackend>::new_generic()
                                    .spacing(8.0)
                                    .push(Text::<IcedBackend>::new("OpenRouter API Key").headline())
                                    .push(
                                        TextInput::<Message>::new(
                                            api_key.clone(),
                                            "sk-or-...",
                                            |s| Message::SetApiKey(s),
                                        )
                                        .password()
                                    )
                                    .push(
                                        Text::<IcedBackend>::new(
                                            "Enter your key to enable personalized AI features. If left empty, a shared rate-limited fallback will be used."
                                        )
                                        .caption2()
                                        .secondary()
                                    )
                            )
                            .push(
                                Button::new(Text::new("Open Full AI Settings").caption1())
                                    .on_press(Message::SetTab(Page::SettingsAI))
                                    .variant(Variant::Soft)
                            )
                    )
                )
        );

    PageResult {
        title: "Intelligence".to_string(),
        view: Box::new(
            ComponentDoc::new(
                "Peak Intelligence",
                "Documentation for the AI Action Bridge and core agentic protocols.",
                "// Triggering an AI action via protocol\nI'll show you the buttons. [Action: Navigate(Button)]",
                std::sync::Arc::new(preview)
            )
                .theory(
                        "# The Neural Architecture\n\
                        Peak Intelligence is not just a chatbot; it's a **Semantic Serialization Engine** that transforms visual hierarchy into machine-readable state.\n\n\
                        ### 1. The Semantic Tree\n\
                        Every component in PeakUI implements the `describe()` method, which outputs a `SemanticNode`. This creates a parallel tree structure optimized for LLM consumption:\n\
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
                        fn describe(&self, _context: &Context) -> SemanticNode {\n\
                            SemanticNode {\n\
                                role: \"custom_trigger\".into(),\n\
                                neural_tag: Some(\"main-action\".into()),\n\
                                ..Default::default()\n\
                            }\n\
                        }\n\
                        ```"
                )
        ),
        inspector: None,
        search_config: None,
        toolbar_items: Vec::new(),
        sidebar_toggle: Some(Message::ToggleSidebar),
    }
}
