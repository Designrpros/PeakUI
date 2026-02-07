use crate::prelude::*;
use crate::reference::app::{AIProviderChoice, Message};
use crate::navigation::PageResult;

pub fn view(
    _context: &Context,
    _is_mobile: bool,
    api_key: String,
    ai_provider: AIProviderChoice,
    enable_exposure: bool,
) -> PageResult<Message> {
    PageResult::new(crate::core::ProxyView::new(move |context| {
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

        // Manual Grid Layout for robustness
        let mut provider_view = VStack::<Message, IcedBackend>::new_generic()
            .spacing(16.0)
            .width(Length::Fill);

        let is_wide = context.size.width > 700.0;
        let columns = if is_wide { 3 } else { 1 };
        
        for chunk in providers.chunks(columns) {
            let mut row = HStack::<Message, IcedBackend>::new_generic()
                .spacing(16.0)
                .width(Length::Fill);
            
            for (choice, name, icon, desc) in chunk {
                let is_selected = *choice == ai_provider;
                
                // Build robust card content
                let content = VStack::<Message, IcedBackend>::new_generic()
                    .spacing(12.0)
                    .padding(20.0)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .push(
                        HStack::<Message, IcedBackend>::new_generic()
                            .spacing(12.0)
                            .align_y(Alignment::Center)
                            .push(Icon::<IcedBackend>::new(icon.to_string()).size(24.0))
                            .push(
                                Text::<IcedBackend>::new(name.to_string())
                                    .bold()
                            )
                    )
                    .push(
                        Text::<IcedBackend>::new(desc.to_string())
                            .caption2()
                            .secondary()
                            .width(Length::Fill)
                            .align_center()
                    );

                row = row.push(
                    Button::<Message, IcedBackend>::new(content)
                        .variant(if is_selected { Variant::Soft } else { Variant::Ghost })
                        .on_press(Message::SetAIProvider(*choice))
                        .width(Length::Fill)
                        .height(Length::Fixed(140.0))
                );
            }
            
            // Add row to view
            provider_view = provider_view.push(row);
        }

        let main_view = VStack::<Message, IcedBackend>::new_generic()
            .width(Length::Fill)
            .spacing(32.0)
            .padding(Padding {
                top: context.safe_area.top,
                right: 20.0,
                bottom: context.safe_area.bottom,
                left: 20.0,
            })
            // Hero Header (Standardized with other settings)
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
                crate::containers::Section::new(
                    "Model Provider",
                    VStack::new_generic()
                        .spacing(16.0)
                        .push(
                            Text::<IcedBackend>::new("Select how you want to run your AI models. Local providers run entirely on your machine, while OpenRouter connects to high-performance cloud fleets.")
                                .body()
                                .secondary()
                        )
                        .push(provider_view)
                )
                .width(Length::Fill)
            )
            .push(Divider::<IcedBackend>::new())

            // Configuration Section
            .push(
                crate::containers::Section::new(
                    "Configuration",
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
                        .push(Divider::<IcedBackend>::new().padding(8.0))
                        .push(
                            HStack::<Message, IcedBackend>::new_generic()
                                .spacing(12.0)
                                .align_y(Alignment::Center)
                                .push(
                                    VStack::<Message, IcedBackend>::new_generic()
                                        .spacing(4.0)
                                        .width(Length::Fill)
                                        .push(Text::<IcedBackend>::new("Neural Exposure").body().bold())
                                        .push(
                                            Text::<IcedBackend>::new("Expose a local network API (Port 8081) for external AI control.")
                                                .caption2()
                                                .secondary()
                                        )
                                )
                                .push(
                                    {
                                        #[cfg(target_arch = "wasm32")]
                                        {
                                            HStack::<Message, IcedBackend>::new_generic()
                                                .spacing(10.0)
                                                .align_y(iced::Alignment::Center)
                                                .push(
                                                    Toggle::<Message, IcedBackend>::new(
                                                        "",
                                                        enable_exposure,
                                                        |b| Message::SetExposure(b) 
                                                    )
                                                )
                                                .push(
                                                    Text::<IcedBackend>::new("Not available in Browser").caption2().secondary()
                                                )
                                        }
                                        #[cfg(not(target_arch = "wasm32"))]
                                        {
                                            Toggle::<Message, IcedBackend>::new(
                                                "",
                                                enable_exposure,
                                                |b| Message::SetExposure(b)
                                            )
                                        }
                                    }
                                )
                        )
                )
                .width(Length::Fill)
            )

            // Implementation Detail
            .push(Divider::<IcedBackend>::new())
            .push(
                crate::containers::Section::new(
                    "Agnostic Routing Example",
                    crate::views::CodeBlock::rust(
                        r#"
                            // The application remains agnostic of the underlying provider.
                            // Switching providers hot-reloads the Bridge automatically.

                            Message::SetAIProvider(AIProviderChoice::LlamaCpp) => {
                                self.intelligence = Arc::new(PeakIntelligenceBridge::new(
                                    ModelProvider::LlamaCpp,
                                    "models/llama-3-8b.gguf",
                                    None
                                ));
                            }
                        "#
                    )
                    .on_copy(Message::CopyCode)
                )
                .width(Length::Fill)
            );

        main_view.view(context)
    }))
}
