use crate::prelude::*;
use crate::reference::app::{AIProviderChoice, Message};

pub fn view<B: Backend>(
    _context: &Context,
    _is_mobile: bool,
    api_key: String,
    ai_provider: AIProviderChoice,
    enable_exposure: bool,
    state_json: Option<String>,
) -> PageResult<Message, B> {
    PageResult::new(crate::core::ProxyView::new(move |context| {
        // Safe Area Adaptation
        let safe = context.safe_area;
        let p_top = safe.top;
        let p_bottom = safe.bottom;

        // Config for providers
        let providers = [
            (
                AIProviderChoice::Ollama,
                "Ollama",
                "brain",
                "Privacy-first local inference using your GPU.",
            ),
            (
                AIProviderChoice::LlamaCpp,
                "Llama.cpp",
                "cpu",
                "Lightweight local execution for fast response.",
            ),
            (
                AIProviderChoice::OpenRouter,
                "OpenRouter",
                "cloud",
                "Access massive cloud models via API.",
            ),
        ];

        // Manual Grid Layout for robustness
        let mut provider_view = VStack::<Message, B>::new_generic()
            .spacing(16.0)
            .width(Length::Fill);

        let is_wide = context.size.width > 700.0;
        let columns = if is_wide { 3 } else { 1 };

        for chunk in providers.chunks(columns) {
            let mut row = HStack::<Message, B>::new_generic()
                .spacing(16.0)
                .width(Length::Fill);

            for (choice, name, icon, desc) in chunk {
                let is_selected = *choice == ai_provider;

                let mut name_text = Text::<B>::new(name.to_string()).bold().title3();
                let mut desc_text = Text::<B>::new(desc.to_string()).caption2();
                let mut icon_view = Icon::<B>::new(icon.to_string()).size(32.0);

                if is_selected {
                    name_text = name_text.color(iced::Color::WHITE);
                    desc_text = desc_text.color(iced::Color::WHITE);
                    icon_view = icon_view.color(iced::Color::WHITE);
                }

                let card_content = VStack::<Message, B>::new_generic()
                    .push(icon_view)
                    .push(name_text)
                    .push(desc_text)
                    .spacing(8.0);

                row = row.push(
                    Button::<Message, B>::new(card_content)
                        .variant(if is_selected {
                            Variant::Solid
                        } else {
                            Variant::Soft
                        })
                        .on_press(Message::SetAIProvider(*choice))
                        .width(Length::Fill)
                        .height(Length::Fixed(160.0)),
                );
            }
            provider_view = provider_view.push(row);
        }

        let main_view = VStack::<Message, B>::new_generic()
            .width(Length::Fill)
            .spacing(32.0)
            .padding(iced::Padding {
                top: p_top + 40.0,
                bottom: p_bottom + 40.0,
                left: 40.0,
                right: 40.0,
            })
            .push(Text::<B>::new("Intelligence").large_title().bold())
            .push(provider_view)
            .push(
                VStack::<Message, B>::new_generic()
                    .spacing(12.0)
                    .push(Text::<B>::new("API Configuration").title3().bold())
                    .push(
                        TextInput::<Message, B>::new(
                            api_key.clone(),
                            "Enter API Key...",
                            Message::SetApiKey,
                        )
                        .password()
                        .on_submit(Message::None),
                    ),
            )
            .push(
                VStack::<Message, B>::new_generic()
                    .push(Text::<B>::new("Local Exposure").title3().bold())
                    .push(
                        Text::<B>::new("Allow local network instances to discover this brain.")
                            .caption1(),
                    )
                    .push(Toggle::<Message, B>::new(
                        "Enable Exposure".to_string(),
                        enable_exposure,
                        Message::SetExposure,
                    )),
            )
            .push(
                VStack::<Message, B>::new_generic()
                    .push(Text::<B>::new("Neural State").title3().bold())
                    .push(if let Some(json) = state_json.clone() {
                        crate::views::CodeBlock::new(json)
                            .language("json")
                            .on_copy(Message::CopyCode)
                    } else {
                        crate::views::CodeBlock::new("// Serialization not available".to_string())
                            .language("json")
                            .on_copy(Message::CopyCode)
                    }),
            );

        main_view.view(context)
    }))
}
