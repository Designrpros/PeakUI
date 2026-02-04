use crate::navigation::PageResult;
use crate::prelude::*;
use crate::reference::app::Message;
use crate::reference::pages::shared::*;

pub fn view(context: &Context, is_mobile: bool) -> PageResult<Message> {
    PageResult::new(
        VStack::<Message, IcedBackend>::new_generic()
            .width(Length::Fill)
            .spacing(48.0)
            .padding(Padding {
                top: context.safe_area.top,
                right: if is_mobile { 24.0 } else { 48.0 },
                bottom: context.safe_area.bottom,
                left: if is_mobile { 24.0 } else { 48.0 },
            })
            .push(heading("Accessibility & Bridge", context))
            .push(paragraph(
                "PeakUI unifies AI-readability and human accessibility through a single semantic tree and the AccessibilityBridge.",
                context,
            ))
            .push(
                VStack::new_generic()
                    .spacing(24.0)
                    .width(Length::Fill)
                    .push(sub_heading("The Semantic Tree", context))
                    .push(paragraph(
                        "Every widget in PeakUI implements a describe() method that emits a SemanticNode. This node contains both information for AI agents and essential metadata for assistive technologies.",
                        context,
                    ))
                    .push(bullet_list(
                        vec![
                            "Roles: Standardized roles like Button, Slider, and TextField.",
                            "States: Dynamic flags for is_focused, is_disabled, and selected.",
                            "Labels & Hints: Human-readable descriptions that serve both screen readers and LLMs.",
                        ],
                        context,
                    )),
            )
            .push(
                VStack::new_generic()
                    .spacing(24.0)
                    .width(Length::Fill)
                    .push(sub_heading("AccessibilityBridge", context))
                    .push(paragraph(
                        "The AccessibilityBridge acts as a dispatcher, connecting the semantic tree to platform-native accessibility APIs (like AccessKit on desktop).",
                        context,
                    ))
                    .push(bullet_list(
                        vec![
                            "Event-Driven: Processes updates in real-time as the UI changes.",
                            "Type-Safe: Uses the AccessibilityRole enum for robust mapping.",
                            "Unified Logic: Developers describe the component once, and the bridge handles the rest.",
                        ],
                        context,
                    )),
            )
            .push(
                VStack::new_generic()
                    .spacing(24.0)
                    .width(Length::Fill)
                    .push(sub_heading("Why it matters", context))
                    .push(paragraph(
                        "By treating accessibility as a core data structure, PeakUI ensures that applications are 'Green' (energy efficient for AI) and 'Inclusive' (perfect for human assistance tools) by default.",
                        context,
                    )),
            )
            .push(Space::<IcedBackend>::new(
                Length::Fill,
                Length::Fixed(120.0),
            )),
    )
}
