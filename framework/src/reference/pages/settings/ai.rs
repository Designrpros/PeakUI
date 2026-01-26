use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::page::PageResult;
use crate::reference::views::ComponentDoc;
use std::sync::Arc;

pub fn view(_context: &Context, _is_mobile: bool, api_key: String) -> PageResult {
    let preview = VStack::<Message, IcedBackend>::new_generic()
        .spacing(24.0)
        .push(
            VStack::<Message, IcedBackend>::new_generic()
                .spacing(8.0)
                .push(Text::<IcedBackend>::new("OpenRouter API Key").headline())
                .push(
                    Text::<IcedBackend>::new(
                        "Enter your OpenRouter API Key to enable the AI Assistant. If left empty, a shared fallback key will be used (rate limited)."
                    )
                    .caption1()
                    .secondary()
                )
        )
        .push(
            TextInput::<Message>::new(
                api_key.clone(),
                "sk-or-...",
                |s| Message::SetApiKey(s),
            )
            .password()
        );

    PageResult::new(ComponentDoc::new(
        "AI Settings",
        "Configure your AI provider and keys.",
        r#"
// Example: Setting the API Key via Message
Message::SetApiKey("sk-or-v1-...".to_string())
"#,
        Arc::new(preview),
    ))
}
