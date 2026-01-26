use super::super::app::Message;
use super::super::page::PageResult;
use super::shared::*;
use crate::prelude::*;

pub fn view(context: &Context, is_mobile: bool) -> PageResult {
    PageResult::new(
        VStack::<Message, IcedBackend>::new_generic()
            .width(Length::Fill)
            .spacing(48.0)
            .padding(Padding {
                top: 48.0,
                right: if is_mobile { 24.0 } else { 48.0 },
                bottom: 120.0,
                left: if is_mobile { 24.0 } else { 48.0 },
            })
            .push(heading("Project Structure", context))
            .push(
                paragraph(
                    "The PeakUI reference app follows a feature-based architecture, promoting modularity and separation of concerns.",
                    context
                )
            )
            .push(
                VStack::new_generic()
                    .spacing(24.0)
                    .width(Length::Fill)
                    .push(sub_heading("Core Modules", context))
                    .push(
                        VStack::new_generic()
                            .spacing(12.0)
                            .width(Length::Fill)
                            .push(architecture_item("crates/peak-ui", "The core framework library containing all atoms, containers, and layout engines.", context))
                            .push(architecture_item("crates/peak-apps", "The consuming application binary.", context))
                    )
            )
            .push(
                VStack::new_generic()
                    .spacing(24.0)
                    .width(Length::Fill)
                    .push(sub_heading("Source Organization", context))
                    .push(
                        VStack::new_generic()
                            .spacing(12.0)
                            .width(Length::Fill)
                            .push(architecture_item("src/core.rs", "Fundamental traits (View, Context) and backend definitions.", context))
                            .push(architecture_item("src/atoms/", "Atomic UI components (Text, Button, Icon).", context))
                            .push(architecture_item("src/layout.rs", "Layout primitives (VStack, HStack, ZStack).", context))
                            .push(architecture_item("src/reference/", "The Reference App implementation (where these pages live).", context))
                    )
            )
    )
}
