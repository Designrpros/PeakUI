use super::super::app::Message;
use super::super::model::Page;
use super::super::page::PageResult;
use crate::prelude::*;

pub fn view(_context: &Context, is_mobile: bool) -> PageResult {
    PageResult::new(ProxyView::<Message, IcedBackend>::new(move |ctx| {
        let t = ctx.theme;
        let is_narrow = is_mobile || ctx.size.width < 1000.0;

        // --- 1. Hero Section ---
        let hero = VStack::<Message, IcedBackend>::new_generic()
            .spacing(32.0)
            .align_x(if is_narrow {
                iced::Alignment::Center
            } else {
                iced::Alignment::Start
            })
            .width(Length::Fill)
            .push(
                VStack::new_generic()
                    .spacing(12.0)
                    .align_x(if is_narrow {
                        iced::Alignment::Center
                    } else {
                        iced::Alignment::Start
                    })
                    .push(
                        Text::<IcedBackend>::new("Quick Start")
                            .size(if is_narrow { 32.0 } else { 48.0 })
                            .bold()
                            .color(t.colors.text_primary),
                    )
                    .push(
                        Text::<IcedBackend>::new("Welcome to the PeakUI documentation! This page will give you an introduction to 80% of the PeakUI concepts that you will use on a daily basis.")
                            .size(20.0)
                            .color(t.colors.text_secondary),
                    ),
            )
            .push(
                HStack::new_generic()
                    .spacing(20.0)
                    .align_y(iced::Alignment::Center)
                    .push(
                        Button::label("Learn more")
                            .variant(Variant::Outline)
                            .on_press(Message::SetTab(Page::Architecture))
                            .size(ControlSize::Large)
                            .width(Length::Fixed(180.0)),
                    )
                    .push(
                        Button::label("Browse Catalog")
                            .variant(Variant::Soft)
                            .on_press(Message::SetTab(Page::ShowcaseButtons))
                            .size(ControlSize::Large)
                            .width(Length::Fixed(180.0)),
                    ),
            );

        // --- Helper: Content Section ---
        let doc_section =
            |title: &'static str, content: Vec<Box<dyn View<Message, IcedBackend>>>| {
                let mut column = VStack::new_generic()
                    .spacing(24.0)
                    .align_x(iced::Alignment::Start)
                    .width(Length::Fill);

                column = column.push(
                    Text::<IcedBackend>::new(title)
                        .title2()
                        .bold()
                        .color(t.colors.text_primary),
                );

                for item in content {
                    column = column.push(item);
                }

                column
            };

        // --- Helper: Code Block ---
        let code_block = |code: &'static str| {
            Box::new(ProxyView::<Message, IcedBackend>::new(move |ctx| {
                let t = ctx.theme;
                iced::widget::container(
                    Text::<IcedBackend>::new(code)
                        .size(14.0)
                        .color(t.colors.text_primary)
                        .view(ctx),
                )
                .padding(24)
                .width(Length::Fill)
                .style(move |_| iced::widget::container::Style {
                    background: Some(t.colors.surface.scale_alpha(0.5).into()),
                    border: iced::Border {
                        radius: 12.0.into(),
                        color: t.colors.border.scale_alpha(0.2),
                        width: 1.0,
                    },
                    ..Default::default()
                })
                .into()
            })) as Box<dyn View<Message, IcedBackend>>
        };

        // --- Sections ---

        let you_will_learn = doc_section(
            "You will learn",
            vec![
                Box::new(Text::new("• How to create and nest components")),
                Box::new(Text::new(
                    "• How to add markup and styles using Method Chaining",
                )),
                Box::new(Text::new(
                    "• How to display data and handle conditional rendering",
                )),
                Box::new(Text::new(
                    "• How to respond to events and update the screen (MVU)",
                )),
                Box::new(Text::new(
                    "• How to share data between components via state lifting",
                )),
            ],
        );

        let creating_components = doc_section("Creating and nesting components", vec![
            Box::new(Text::new("PeakUI apps are made out of views. A view is a piece of the UI that has its own logic and appearance. Concepts flow logically from small atoms to entire pages.")),
            Box::new(Text::new("PeakUI views are Rust functions that return a `View` trait object:")),
            Box::new(code_block("fn my_button() -> impl View<Message> {\n  Button::label(\"I'm a button\")\n}")),
            Box::new(Text::new("Now that you've declared my_button, you can nest it into another view:")),
            Box::new(code_block("fn my_app() -> impl View<Message> {\n  VStack::new()\n    .push(Text::new(\"Welcome to my app\"))\n    .push(my_button())\n}")),
        ]);

        let adding_styles = doc_section("Adding styles", vec![
            Box::new(Text::new("In PeakUI, you specify styles using semantic variants and intent via modifiers. This ensures consistency across your entire application.")),
            Box::new(code_block("Button::label(\"Action\")\n  .variant(Variant::Solid)\n  .intent(Intent::Primary)")),
            Box::new(Text::new("You can also use custom theme tokens to style your components:")),
            Box::new(code_block("Text::new(\"Hello\")\n  .color(t.colors.primary)\n  .bold()")),
        ]);

        let conditional_rendering = doc_section("Conditional rendering", vec![
            Box::new(Text::new("In PeakUI, there is no special syntax for writing conditions. Instead, you'll use regular Rust code, such as `if` statements or `match` expressions.")),
            Box::new(code_block("let content = if is_logged_in {\n    Box::new(AdminPanel::new())\n} else {\n    Box::new(LoginForm::new())\n};\n\nVStack::new().push(content)")),
        ]);

        let rendering_lists = doc_section("Rendering lists", vec![
            Box::new(Text::new("You will rely on Rust's powerful iterators to render lists of components.")),
            Box::new(code_block("let list_items = products.iter().map(|product| {\n    Text::new(&product.title)\n});\n\nVStack::new().extend(list_items)")),
        ]);

        let localization = doc_section("Localization", vec![
            Box::new(Text::new("For global applications, PeakUI leverages the Mozilla Fluent system. This allows for complex translations, including plurals and gender-neutral terms.")),
            Box::new(code_block("// .ftl: welcome-user = Welcome, { $user }!\nText::new(context.t(\"welcome-user\"))")),
        ]);

        let responding_to_events = doc_section("Responding to events", vec![
            Box::new(Text::new("You can respond to events by mapping them to your application's `Message` enum:")),
            Box::new(code_block("fn my_button() -> impl View<Message> {\n  Button::label(\"Click me\")\n    .on_press(Message::ButtonClicked)\n}")),
            Box::new(Text::new("The `update` function in your app will then handle these messages to change the state.")),
        ]);

        let updating_the_screen = doc_section("Updating the screen", vec![
            Box::new(Text::new("To update the screen, you change your application's Model in the `update` function. PeakUI handles the re-rendering for you efficiently.")),
            Box::new(code_block("fn update(&mut self, message: Message) -> Task<Message> {\n    match message {\n        Message::Increment => {\n            self.count += 1;\n            Task::none()\n        }\n    }\n}")),
        ]);

        // --- Final Assembly ---
        VStack::new_generic()
            .width(Length::Fill)
            .spacing(64.0)
            .padding(Padding {
                top: 48.0,
                right: if is_narrow { 24.0 } else { 48.0 },
                bottom: 120.0,
                left: if is_narrow { 24.0 } else { 48.0 },
            })
            .align_x(iced::Alignment::Start)
            .push(hero)
            .push(you_will_learn)
            .push(creating_components)
            .push(adding_styles)
            .push(conditional_rendering)
            .push(rendering_lists)
            .push(localization)
            .push(responding_to_events)
            .push(updating_the_screen)
            .view(ctx)
    }))
}
