use super::super::app::Message;
use super::super::model::Page as ReferencePage;
use super::super::pages;
use crate::prelude::*;

pub struct CanvasView {
    pub active_tab: ReferencePage,
    pub navigation_mode: String,
    pub button_lab: super::super::app::ButtonLabState,
    pub typography_lab: super::super::app::TypographyLabState,
    pub layout_lab: super::super::app::LayoutLabState,
    pub sizing_lab: super::super::app::SizingLabState,
    pub render_mode: super::super::app::RenderMode,
    pub api_key: String,
    pub ai_provider: super::super::app::AIProviderChoice,
    pub search_query: String,
    pub icon_limit: usize,
}

use super::super::page::PageResult;

impl CanvasView {
    pub fn new(
        active_tab: ReferencePage,
        navigation_mode: String,
        button_lab: super::super::app::ButtonLabState,
        typography_lab: super::super::app::TypographyLabState,
        layout_lab: super::super::app::LayoutLabState,
        sizing_lab: super::super::app::SizingLabState,
        render_mode: super::super::app::RenderMode,
        api_key: String,
        ai_provider: super::super::app::AIProviderChoice,
        search_query: String,
        icon_limit: usize,
    ) -> Self {
        Self {
            active_tab,
            navigation_mode,
            button_lab,
            typography_lab,
            layout_lab,
            sizing_lab,
            render_mode,
            api_key,
            ai_provider,
            search_query,
            icon_limit,
        }
    }

    pub fn render_page(&self, context: &Context) -> PageResult {
        let is_mobile = context.size.width < 900.0;

        let page = match &self.active_tab {
            // Guide
            ReferencePage::Introduction => pages::introduction::view(context, is_mobile),
            ReferencePage::Architecture => pages::architecture::view(context, is_mobile),
            ReferencePage::ProjectStructure => pages::project_structure::view(context, is_mobile),

            ReferencePage::Roadmap => pages::roadmap::view(context, is_mobile),
            ReferencePage::Intelligence => {
                pages::intelligence::view(context, is_mobile, self.api_key.clone())
            }

            // Ecosystem
            ReferencePage::PeakDB => pages::peak_db::view(context, is_mobile),
            ReferencePage::PeakCloud => pages::peak_cloud::view(context, is_mobile),
            ReferencePage::PeakHub => pages::peak_hub::view(context, is_mobile),
            ReferencePage::SwarmDashboard => {
                let view = super::swarm_dashboard::SwarmDashboardView::new(context.peak_id.clone());
                crate::navigation::PageResult::new(view)
            }
            ReferencePage::PeakDesktop => pages::peak_desktop::view(context, is_mobile),
            ReferencePage::PeakOSCore => pages::peak_os_core::view(context, is_mobile),

            // Legacy
            ReferencePage::ApiSchema => pages::peak_os_core::view(context, is_mobile), // Redirect to new page
            ReferencePage::Community => pages::community::view(context, is_mobile),

            // Concepts (Overview is legacy/fallback)
            ReferencePage::Overview => pages::introduction::view(context, is_mobile),
            ReferencePage::Customizations => {
                pages::customizations::view(context, is_mobile, self.render_mode)
            }
            ReferencePage::BasicSizing => {
                pages::sizing::view(context, is_mobile, &self.sizing_lab, self.render_mode)
            }
            ReferencePage::Colors => pages::colors::view(context, self.render_mode),
            ReferencePage::Typography => {
                pages::typography::view(context, &self.typography_lab, self.render_mode)
            }
            ReferencePage::Layout => {
                pages::layout::view(context, is_mobile, &self.layout_lab, self.render_mode)
            }

            // Atoms (Phase 3/4)
            ReferencePage::Text => pages::text::view(context),
            ReferencePage::Icon => {
                pages::icon::view(context, self.search_query.clone(), self.icon_limit)
            }
            ReferencePage::Button => {
                pages::button::view(context, &self.button_lab, self.render_mode)
            }
            ReferencePage::Shapes => pages::shapes::view(context, is_mobile),
            ReferencePage::Image => pages::image::view(context),
            ReferencePage::Video => pages::video::view(context),
            ReferencePage::WebView => pages::web_view::view(context),
            ReferencePage::Divider => pages::divider::view(context),

            // Containers (Phase 4)
            ReferencePage::VStack => pages::vstack::view(context),
            ReferencePage::HStack => pages::hstack::view(context),
            ReferencePage::ZStack => pages::zstack::view(context),
            ReferencePage::Overlay => pages::overlay::view(context),
            ReferencePage::ScrollView => pages::scroll_view::view(context),
            ReferencePage::Card => pages::card::view(context),

            // Navigation (Phase 4)
            ReferencePage::Sidebar => pages::sidebar_doc::view(context),
            ReferencePage::Tabbar => pages::tabbar_doc::view(context),
            ReferencePage::Modal => pages::modal_doc::view(context),
            ReferencePage::NavigationSplit => pages::navigation_split::view(context),
            ReferencePage::Section => pages::section::view(context),

            // Showcase Gallery (Deprecated / Redirects)
            ReferencePage::ShowcaseButtons => {
                pages::button::view(context, &self.button_lab, self.render_mode)
            }
            ReferencePage::ShowcaseInputs
            | ReferencePage::ShowcaseToggles
            | ReferencePage::ShowcaseSliders
            | ReferencePage::ShowcasePickers => pages::introduction::view(context, is_mobile),

            // Hooks Gallery
            ReferencePage::UseState => pages::hooks::view(context, is_mobile),
            ReferencePage::UseEffect => pages::side_effects::view(context, is_mobile),
            ReferencePage::UseMemo | ReferencePage::UseCallback => {
                pages::performance::view(context, is_mobile)
            }

            // Settings Gallery
            ReferencePage::Appearance => pages::settings::appearance::view(context, is_mobile),
            ReferencePage::Scaling => pages::settings::scaling::view(context, is_mobile),
            ReferencePage::Shortcuts => pages::settings::shortcuts::view(context, is_mobile),
            ReferencePage::About | ReferencePage::Updates => {
                pages::settings::about::view(context, is_mobile)
            }
            ReferencePage::SettingsAI => pages::settings::ai::view(
                context,
                is_mobile,
                self.api_key.clone(),
                self.ai_provider,
            ),

            // Fallback
            ReferencePage::Landing | ReferencePage::Unknown(_) => {
                pages::introduction::view(context, is_mobile)
            }
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
