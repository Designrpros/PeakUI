use crate::prelude::*;
use crate::reference::app::{AIProviderChoice, Message};
use crate::reference::page::PageResult;

pub fn view(
    _context: &Context,
    _is_mobile: bool,
    api_key: String,
    ai_provider: AIProviderChoice,
) -> PageResult {
    let mut provider_selection = HStack::<Message, IcedBackend>::new_generic()
        .spacing(16.0)
        .width(Length::Fill);

    // Config for providers
    let providers = [
        (
            AIProviderChoice::Ollama, 
            "Ollama", 
            "brain", 
            "Privacy-first local inference using your GPU."
        ),
        (
            AIProviderChoice::LlamaCpp, 
            "Llama.cpp", 
            "cpu", 
            "Lightweight local execution for fast response."
        ),
        (
            AIProviderChoice::OpenRouter, 
            "OpenRouter", 
            "cloud", 
            "Access massive cloud models via API."
        ),
    ];

    for (choice, name, icon, desc) in providers {
        let is_selected = choice == ai_provider;
        
        // Build the card content
        let mut card_header = HStack::<Message, IcedBackend>::new_generic()
            .spacing(12.0)
            .align_y(Alignment::Center)
            .push(Icon::<IcedBackend>::new(icon).size(20.0))
            .push(Text::<IcedBackend>::new(name).title3().bold().width(Length::Fill));
        
        if is_selected {
            card_header = card_header.push(
                Icon::<IcedBackend>::new("check_circle")
                    .size(16.0)
                    .primary()
            );
        }

        let card_content = VStack::<Message, IcedBackend>::new_generic()
            .spacing(12.0)
            .padding(16.0)
            .push(card_header)
            .push(
                Text::<IcedBackend>::new(desc)
                    .caption1()
                    .secondary()
                    .width(Length::Fill)
            );

        // Wrap in a GlassCard for premium look
        let card = crate::containers::GlassCard::<Message, IcedBackend>::new(card_content);

        // Add to the selection row, wrapped in a button to handle clicks
        provider_selection = provider_selection.push(
            Button::<Message, IcedBackend>::new(card)
                .variant(if is_selected { Variant::Soft } else { Variant::Ghost })
                .on_press(Message::SetAIProvider(choice))
                .width(Length::FillPortion(1))
        );
    }

    let main_view = VStack::<Message, IcedBackend>::new_generic()
        .width(Length::Fill)
        .spacing(32.0)
            .padding(Padding {
                top: _context.safe_area.top,
                right: 20.0,
                bottom: _context.safe_area.bottom,
                left: 20.0,
            })
        // Header
        .push(
            VStack::<Message, IcedBackend>::new_generic()
                .spacing(8.0)
                .push(Text::<IcedBackend>::new("Intelligence").large_title().bold())
                .push(
                    Text::<IcedBackend>::new("Configure your AI routing, local inference engines, and cloud API keys.")
                        .title3()
                        .secondary(),
                )
        )
        .push(Divider::<IcedBackend>::new())
        
        // Provider Selection Section
        .push(
            VStack::<Message, IcedBackend>::new_generic()
                .spacing(20.0)
                .push(Text::<IcedBackend>::new("Model Provider").title2().bold())
                .push(
                    Text::<IcedBackend>::new("Select how you want to run your AI models. Local providers run entirely on your machine, while OpenRouter connects to high-performance cloud fleets.")
                        .body()
                        .secondary()
                )
                .push(provider_selection)
        )
        .push(Divider::<IcedBackend>::new())

        // Configuration Section
        .push(
            VStack::<Message, IcedBackend>::new_generic()
                .spacing(20.0)
                .push(Text::<IcedBackend>::new("Configuration").title2().bold())
                .push(
                    VStack::<Message, IcedBackend>::new_generic()
                        .spacing(12.0)
                        .push(
                            VStack::<Message, IcedBackend>::new_generic()
                                .spacing(8.0)
                                .push(Text::<IcedBackend>::new("OpenRouter API Key").body().bold())
                                .push(
                                    TextInput::<Message>::new(
                                        api_key.clone(),
                                        "sk-or-v1-...",
                                        |s| Message::SetApiKey(s),
                                    )
                                    .password()
                                )
                                .push(
                                    Text::<IcedBackend>::new("Only required if OpenRouter is selected as the provider.")
                                        .caption2()
                                        .secondary()
                                )
                        )
                )
        )

        // Implementation Detail
        .push(Divider::<IcedBackend>::new())
        .push(
            VStack::<Message, IcedBackend>::new_generic()
                .spacing(16.0)
                .push(Text::<IcedBackend>::new("Agnostic Routing Example").title2().bold())
                .push(
                    crate::views::CodeBlock::rust(
                        r#"// The application remains agnostic of the underlying provider.
// Switching providers hot-reloads the Bridge automatically.

Message::SetAIProvider(AIProviderChoice::LlamaCpp) => {
    self.intelligence = Arc::new(PeakIntelligenceBridge::new(
        ModelProvider::LlamaCpp,
        "models/llama-3-8b.gguf",
        None
    ));
}"#
                    )
                    .on_copy(Message::CopyCode)
                )
        );

    PageResult::new(crate::scroll_view::ScrollView::new(main_view))
}
