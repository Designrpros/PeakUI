use crate::navigation::PageResult;
use crate::prelude::*;
use crate::reference::app::Message;

pub fn view(_context: &Context, is_mobile: bool) -> PageResult<Message> {
    PageResult::new(ProxyView::<Message, IcedBackend>::new(move |ctx| {
        let t = ctx.theme;
        let is_narrow = is_mobile || ctx.size.width < 1000.0;

        // --- 1. Hero Section ---
        let hero = VStack::<Message, IcedBackend>::new_generic()
            .spacing(12.0)
            .align_x(iced::Alignment::Start)
            .width(Length::Fill)
            .push(
                Text::<IcedBackend>::new("PeakSuite")
                    .size(if is_narrow { 42.0 } else { 56.0 })
                    .bold()
                    .align_start()
                    .width(Length::Fill)
                    .color(t.colors.text_primary),
            )
            .push(
                Text::<IcedBackend>::new("The Unified Neural Intelligence Stack")
                    .size(20.0)
                    .align_start()
                    .width(Length::Fill)
                    .color(t.colors.text_secondary),
            );

        // --- 2. Sections ---
        let sections = VStack::<Message, IcedBackend>::new_generic()
            .width(Length::Fill)
            .spacing(64.0)
            .push(doc_section(
                "PeakOS",
                "The foundation of the suite. A decentralized, agent-native operating system designed to bridge the gap between human intent and machine execution.",
                ctx,
            ))
            .push(doc_section(
                "PeakDB",
                "The sovereign neural memory. A high-performance vector database that stores and indexes semantic data, enabling long-term agent reasoning and context awareness.",
                ctx,
            ))
            .push(doc_section(
                "Peak Cloud",
                "The shared intelligence layer. Facilitating peer-to-peer synchronization and distributed processing across the Peak ecosystem.",
                ctx,
            ))
            .push(doc_section(
                "Peak Hub",
                "The command and control center. A centralized interface for managing your agents, monitoring system performance, and orchestrating complex workflows.",
                ctx,
            ))
            .push(doc_section(
                "Peak UI",
                "The human-agent interface engine. A multi-kernel design system that allows developers to build interfaces that both humans and AI can perceive and manipulate.",
                ctx,
            ));

        // --- Final Assembly ---
        VStack::new_generic()
            .width(Length::Fill)
            .spacing(64.0)
            .padding(Padding {
                top: ctx.safe_area.top,
                right: if is_narrow { 24.0 } else { 48.0 },
                bottom: ctx.safe_area.bottom,
                left: if is_narrow { 24.0 } else { 48.0 },
            })
            .align_x(iced::Alignment::Start)
            .push(hero)
            .push(sections)
            .push(Space::<IcedBackend>::new(Length::Fill, Length::Fixed(120.0)))
            .view(ctx)
    }))
}

fn doc_section(
    title: &str,
    description: &str,
    ctx: &Context,
) -> VStack<Message, IcedBackend> {
    let t = ctx.theme;
    VStack::new_generic()
        .width(Length::Fill)
        .spacing(16.0)
        .push(
            Text::<IcedBackend>::new(title)
                .size(24.0)
                .bold()
                .color(t.colors.text_primary),
        )
        .push(
            Text::<IcedBackend>::new(description)
                .size(16.0)
                .color(t.colors.text_secondary)
                .width(Length::Fill),
        )
}
