use super::super::app::Message;
use super::super::model::Page;
use super::super::pages;
use crate::prelude::*;

pub struct CanvasView {
    pub active_tab: Page,
    pub navigation_mode: String,
    pub button_lab: super::super::app::ButtonLabState,
    pub typography_lab: super::super::app::TypographyLabState,
    pub layout_lab: super::super::app::LayoutLabState,
    pub sizing_lab: super::super::app::SizingLabState,
    pub render_mode: super::super::app::RenderMode,
}

use super::super::page::PageResult;

impl CanvasView {
    pub fn new(
        active_tab: Page,
        navigation_mode: String,
        button_lab: super::super::app::ButtonLabState,
        typography_lab: super::super::app::TypographyLabState,
        layout_lab: super::super::app::LayoutLabState,
        sizing_lab: super::super::app::SizingLabState,
        render_mode: super::super::app::RenderMode,
    ) -> Self {
        Self {
            active_tab,
            navigation_mode,
            button_lab,
            typography_lab,
            layout_lab,
            sizing_lab,
            render_mode,
        }
    }

    pub fn render_page(&self, context: &Context) -> PageResult {
        let is_mobile = context.size.width < 900.0;

        let page = match &self.active_tab {
            // Guide
            Page::Introduction => pages::introduction::view(context, is_mobile),
            Page::Architecture => pages::architecture::view(context, is_mobile),
            Page::ProjectStructure => pages::project_structure::view(context, is_mobile),

            Page::Roadmap => pages::roadmap::view(context, is_mobile),

            // Ecosystem
            Page::PeakDB => pages::peak_db::view(context, is_mobile),
            Page::PeakCloud => pages::peak_cloud::view(context, is_mobile),
            Page::PeakDesktop => pages::peak_desktop::view(context, is_mobile),
            Page::PeakOSCore => pages::peak_os_core::view(context, is_mobile),

            // Legacy
            Page::ApiSchema => pages::peak_os_core::view(context, is_mobile), // Redirect to new page
            Page::Community => pages::community::view(context, is_mobile),

            // Concepts (Overview is legacy/fallback)
            Page::Overview => pages::introduction::view(context, is_mobile),
            Page::Customizations => {
                pages::customizations::view(context, is_mobile, self.render_mode)
            }
            Page::BasicSizing => {
                pages::sizing::view(context, is_mobile, &self.sizing_lab, self.render_mode)
            }
            Page::Typography => {
                pages::typography::view(context, &self.typography_lab, self.render_mode)
            }
            Page::Layout => {
                pages::layout::view(context, is_mobile, &self.layout_lab, self.render_mode)
            }

            // Atoms (Phase 3/4)
            Page::Text => pages::text::view(context),
            Page::Icon => pages::icon::view(context),
            Page::Button => pages::button::view(context, &self.button_lab, self.render_mode),
            Page::Shapes => pages::shapes::view(context, is_mobile),
            Page::Divider => pages::divider::view(context),

            // Containers (Phase 4)
            Page::VStack => pages::vstack::view(context),
            Page::HStack => pages::hstack::view(context),
            Page::ZStack => pages::zstack::view(context),
            Page::Overlay => pages::overlay::view(context),
            Page::ScrollView => pages::scroll_view::view(context),
            Page::Card => pages::card::view(context),

            // Navigation (Phase 4)
            Page::Sidebar => pages::sidebar_doc::view(context),
            Page::Tabbar => pages::tabbar_doc::view(context),
            Page::Modal => pages::modal_doc::view(context),
            Page::NavigationSplit => pages::navigation_split::view(context),
            Page::Section => pages::section::view(context),

            // Showcase Gallery (Deprecated / Redirects)
            Page::ShowcaseButtons => {
                pages::button::view(context, &self.button_lab, self.render_mode)
            }
            Page::ShowcaseInputs
            | Page::ShowcaseToggles
            | Page::ShowcaseSliders
            | Page::ShowcasePickers => pages::introduction::view(context, is_mobile),

            // Hooks Gallery
            Page::UseState => pages::hooks::view(context, is_mobile),
            Page::UseEffect => pages::side_effects::view(context, is_mobile),
            Page::UseMemo | Page::UseCallback => pages::performance::view(context, is_mobile),

            // Settings Gallery
            Page::Appearance => pages::settings::appearance::view(context, is_mobile),
            Page::Scaling => pages::settings::scaling::view(context, is_mobile),
            Page::Shortcuts => pages::settings::shortcuts::view(context, is_mobile),
            Page::About | Page::Updates => pages::settings::about::view(context, is_mobile),

            // Strict enforcement: Fallback to introduction if something goes wrong,
            // but the compiler will now enforce that all variants above are handled.
            Page::Unknown(_) => pages::introduction::view(context, is_mobile),
        };

        if is_mobile {
            page.sidebar_toggle(Message::ToggleSidebar)
        } else {
            page
        }
    }
}

impl View<Message, IcedBackend> for CanvasView {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let page = self.render_page(context);

        container(ScrollView::from_boxed(page.view).view(context))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        let page = self.render_page(context);
        page.view.describe(context)
    }
}
