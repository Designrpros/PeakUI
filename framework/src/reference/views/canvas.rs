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
    pub accessibility_lab: super::super::app::AccessibilityLabState,
    pub icon_lab: super::super::app::IconLabState,
    pub render_mode: super::super::app::RenderMode,
    pub api_key: String,
    pub ai_provider: super::super::app::AIProviderChoice,
    pub search_query: String,
    pub icon_limit: usize,
    pub db_records: Vec<crate::core::SemanticRecord>,
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
        accessibility_lab: super::super::app::AccessibilityLabState,
        icon_lab: super::super::app::IconLabState,
        render_mode: super::super::app::RenderMode,
        api_key: String,
        ai_provider: super::super::app::AIProviderChoice,
        search_query: String,
        icon_limit: usize,
        db_records: Vec<crate::core::SemanticRecord>,
    ) -> Self {
        Self {
            active_tab,
            navigation_mode,
            button_lab,
            typography_lab,
            layout_lab,
            sizing_lab,
            accessibility_lab,
            icon_lab,
            render_mode,
            api_key,
            ai_provider,
            search_query,
            icon_limit,
            db_records,
        }
    }

    pub fn render_page(&self, context: &Context) -> PageResult {
        let is_mobile = context.size.width < 900.0;

        let page = match &self.active_tab {
            // Guide
            ReferencePage::Introduction => pages::guide::introduction::view(context, is_mobile),
            ReferencePage::Architecture => pages::docs::architecture::view(context, is_mobile),
            ReferencePage::ProjectStructure => {
                pages::docs::project_structure::view(context, is_mobile)
            }
            ReferencePage::Accessibility => {
                pages::docs::accessibility::view(context, &self.accessibility_lab, self.render_mode)
            }

            ReferencePage::Roadmap => pages::guide::roadmap::view(context, is_mobile),
            ReferencePage::Intelligence => {
                pages::guide::intelligence::view(context, is_mobile, self.api_key.clone())
            }

            // Ecosystem
            ReferencePage::PeakDB => pages::core::peak_db::view(context, is_mobile),
            ReferencePage::PeakCloud => pages::core::peak_cloud::view(context, is_mobile),
            ReferencePage::PeakHub => pages::core::peak_hub::view(context, is_mobile),
            ReferencePage::SwarmDashboard => {
                let view = super::swarm_dashboard::SwarmDashboardView::new(context.peak_id.clone());
                crate::navigation::PageResult::new(view)
            }
            ReferencePage::PeakDesktop => pages::core::peak_desktop::view(context, is_mobile),
            ReferencePage::PeakOSCore => pages::core::peak_os_core::view(context, is_mobile),

            // Legacy
            ReferencePage::ApiSchema => pages::core::peak_os_core::view(context, is_mobile), // Redirect to new page
            ReferencePage::PeakSuite => pages::guide::peak_suite::view(context, is_mobile),

            // Concepts (Overview is legacy/fallback)
            ReferencePage::Overview => pages::guide::introduction::view(context, is_mobile),
            ReferencePage::Customizations => {
                pages::docs::customizations::view(context, self.render_mode)
            }
            ReferencePage::BasicSizing => {
                pages::docs::sizing::view(context, &self.sizing_lab, self.render_mode)
            }
            ReferencePage::Colors => pages::docs::colors::view(context, self.render_mode),
            ReferencePage::Typography => {
                pages::docs::typography::view(context, &self.typography_lab, self.render_mode)
            }
            ReferencePage::Layout => {
                pages::docs::layout::view(context, &self.layout_lab, self.render_mode)
            }

            // Atoms (Phase 3/4)
            ReferencePage::Text => {
                pages::components::text::view(context, &self.typography_lab, self.render_mode)
            }
            ReferencePage::Icon => pages::components::icon::view(
                context,
                &self.icon_lab,
                self.render_mode,
                self.search_query.clone(),
                self.icon_limit,
            ),
            ReferencePage::Button => {
                pages::components::button::view(context, &self.button_lab, self.render_mode)
            }
            ReferencePage::Shapes => pages::components::shapes::view(context, self.render_mode),
            ReferencePage::Image => pages::components::image::view(context, self.render_mode),
            ReferencePage::Video => pages::components::video::view(context, self.render_mode),
            ReferencePage::WebView => pages::components::web_view::view(context, self.render_mode),
            ReferencePage::Divider => pages::components::divider::view(context, self.render_mode),

            // Containers (Phase 4)
            ReferencePage::VStack => pages::components::vstack::view(context, self.render_mode),
            ReferencePage::HStack => pages::components::hstack::view(context, self.render_mode),
            ReferencePage::ZStack => pages::components::zstack::view(context, self.render_mode),
            ReferencePage::Overlay => pages::components::overlay::view(context, self.render_mode),
            ReferencePage::ScrollView => {
                pages::components::scroll_view::view(context, self.render_mode)
            }
            ReferencePage::Card => pages::components::card::view(context, self.render_mode),

            // Navigation (Phase 4)
            ReferencePage::Sidebar => {
                pages::components::sidebar_doc::view(context, self.render_mode)
            }
            ReferencePage::Tabbar => pages::components::tabbar_doc::view(context, self.render_mode),
            ReferencePage::Modal => pages::components::modal_doc::view(context, self.render_mode),
            ReferencePage::NavigationSplit => {
                pages::components::navigation_split::view(context, self.render_mode)
            }
            ReferencePage::Section => pages::components::section::view(context, self.render_mode),
            ReferencePage::DataTable => {
                pages::components::data_table::view(context, self.render_mode)
            }

            // Showcase Gallery (Deprecated / Redirects)
            ReferencePage::ShowcaseButtons => {
                pages::components::button::view(context, &self.button_lab, self.render_mode)
            }
            ReferencePage::ShowcaseInputs
            | ReferencePage::ShowcaseToggles
            | ReferencePage::ShowcaseSliders
            | ReferencePage::ShowcasePickers => {
                pages::guide::introduction::view(context, is_mobile)
            }

            // Hooks Gallery
            ReferencePage::UseState => pages::hooks::overview::view(context, is_mobile),
            ReferencePage::UseEffect => pages::docs::side_effects::view(context, is_mobile),
            ReferencePage::UseMemo | ReferencePage::UseCallback => {
                pages::docs::performance::view(context, is_mobile)
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

            // Details (from Landing)
            ReferencePage::PeakOSDetail => pages::landing::peak_os::view(context, is_mobile),
            ReferencePage::PeakUIDetail => pages::landing::peak_ui::view(context, is_mobile),
            ReferencePage::PeakDBDetail => {
                pages::landing::peak_db::view(context, is_mobile, self.db_records.clone())
            }
            ReferencePage::PeakRelayDetail => pages::landing::peak_relay::view(context, is_mobile),
            ReferencePage::PeakHubDetail => pages::landing::peak_hub::view(context, is_mobile),

            // Fallback
            ReferencePage::Landing | ReferencePage::Unknown(_) => {
                pages::guide::introduction::view(context, is_mobile)
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
        page.view.view(context)
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        let page = self.render_page(context);
        page.view.describe(context)
    }
}
