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
            .push(heading("Architecture", context))
            .push(
                paragraph(
                    "The PeakUI architecture is designed for scalability, separating concerns between state, logic, and presentation.",
                    context
                )
            )
            .push(
                VStack::new_generic()
                    .spacing(24.0)
                    .width(Length::Fill)
                    .push(sub_heading("Orchestration Layer", context))
                    .push(
                        paragraph(
                            "The ContentView acts as the central hub. It observes the PageResult metadata to dynamically compose the shell:",
                            context
                        )
                    )
                    .push(bullet_list(vec![
                        "Dynamic Overlays: Automatic search and inspector injections.",
                        "Responsive Adaptability: Unifies mobile and desktop navigation patterns.",
                        "Contextual Awareness: Propagates safe areas and theme tokens down the tree."
                    ], context))
            )
            .push(
                VStack::new_generic()
                    .spacing(24.0)
                    .width(Length::Fill)
                    .push(sub_heading("Navigation Modes", context))
                    .push(
                        paragraph("The app supports distinct navigation contexts tailored to the user's intent:", context)
                    )
                    .push(bullet_list(vec![
                        "Guide: Linear, narrative-driven documentation.",
                        "Catalog: Visual exploration of atomic components.",
                        "Workspace: Complex, multi-pane application layouts."
                    ], context))
            )
            .push(
                VStack::new_generic()
                    .spacing(24.0)
                    .width(Length::Fill)
                    .push(sub_heading("Visual Systems", context))
                    .push(bullet_list(vec![
                        "Dynamic Notch: Context-aware status and tool area.",
                        "Floating Dock: Persistent, Z-indexed navigation anchor.",
                        "Glassmorphism: Real-time background blur and saturation adaptation."
                    ], context))
            )
            .push(Space::<IcedBackend>::new(Length::Fill, Length::Fixed(120.0)))
    )
}
