use super::super::app::Message;
use super::super::model::Page as ReferencePage;
use super::super::pages;
use crate::prelude::*;

use super::state::ViewState;

pub struct CanvasView {
    pub state: ViewState,
}

use super::super::page::PageResult;

impl CanvasView {
    pub fn new(state: ViewState) -> Self {
        Self { state }
    }

    pub fn render_page(&self, context: &Context) -> PageResult {
        let is_mobile = context.size.width < 900.0;

        let page = match &self.state.active_tab {
            // Guide
            ReferencePage::Introduction => pages::guide::introduction::view(context, is_mobile),
            ReferencePage::Architecture => pages::docs::architecture::view(context, is_mobile),
            ReferencePage::ProjectStructure => {
                pages::docs::project_structure::view(context, is_mobile)
            }
            ReferencePage::Accessibility => pages::docs::accessibility::view(
                context,
                &self.state.accessibility_lab,
                self.state.render_mode,
            ),

            ReferencePage::Roadmap => pages::guide::roadmap::view(context, is_mobile),
            ReferencePage::Intelligence => {
                pages::guide::intelligence::view(context, is_mobile, self.state.api_key.clone())
            }

            // Ecosystem
            ReferencePage::PeakDB => pages::core::peak_db::view(context, is_mobile),
            ReferencePage::PeakCloud => pages::core::peak_cloud::view(context, is_mobile),
            ReferencePage::PeakHub => pages::core::peak_hub::view(context, is_mobile),
            ReferencePage::SwarmDashboard => {
                let view =
                    super::swarm_dashboard::SwarmDashboardView::new(context.peak_id.to_string());
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
                pages::docs::customizations::view(context, self.state.render_mode)
            }
            ReferencePage::BasicSizing => {
                pages::docs::sizing::view(context, &self.state.sizing_lab, self.state.render_mode)
            }
            ReferencePage::Colors => pages::docs::colors::view(context, self.state.render_mode),
            ReferencePage::Typography => pages::docs::typography::view(
                context,
                &self.state.typography_lab,
                self.state.render_mode,
            ),
            ReferencePage::Layout => {
                pages::docs::layout::view(context, &self.state.layout_lab, self.state.render_mode)
            }

            // Atoms (Phase 3/4)
            ReferencePage::Text => pages::components::text::view(
                context,
                &self.state.typography_lab,
                self.state.render_mode,
            ),
            ReferencePage::Icon => pages::components::icon::view(
                context,
                &self.state.icon_lab,
                self.state.render_mode,
                self.state.search_query.clone(),
                self.state.icon_limit,
            ),
            ReferencePage::Button => pages::components::button::view(
                context,
                &self.state.button_lab,
                self.state.render_mode,
            ),
            ReferencePage::Shapes => {
                pages::components::shapes::view(context, self.state.render_mode)
            }
            ReferencePage::Image => pages::components::image::view(context, self.state.render_mode),
            ReferencePage::Video => pages::components::video::view(context, self.state.render_mode),
            ReferencePage::WebView => {
                pages::components::web_view::view(context, self.state.render_mode)
            }
            ReferencePage::Divider => {
                pages::components::divider::view(context, self.state.render_mode)
            }

            // Containers (Phase 4)
            ReferencePage::VStack => {
                pages::components::vstack::view(context, self.state.render_mode)
            }
            ReferencePage::HStack => {
                pages::components::hstack::view(context, self.state.render_mode)
            }
            ReferencePage::ZStack => {
                pages::components::zstack::view(context, self.state.render_mode)
            }
            ReferencePage::Overlay => {
                pages::components::overlay::view(context, self.state.render_mode)
            }
            ReferencePage::ScrollView => {
                pages::components::scroll_view::view(context, self.state.render_mode)
            }
            ReferencePage::Card => pages::components::card::view(context, self.state.render_mode),

            // Navigation (Phase 4)
            ReferencePage::Sidebar => {
                pages::components::sidebar_doc::view(context, self.state.render_mode)
            }
            ReferencePage::Tabbar => {
                pages::components::tabbar_doc::view(context, self.state.render_mode)
            }
            ReferencePage::Modal => {
                pages::components::modal_doc::view(context, self.state.render_mode)
            }
            ReferencePage::NavigationSplit => {
                pages::components::navigation_split::view(context, self.state.render_mode)
            }
            ReferencePage::Section => {
                pages::components::section::view(context, self.state.render_mode)
            }
            ReferencePage::DataTable => {
                pages::components::data_table::view(context, self.state.render_mode)
            }
            ReferencePage::BarChart => {
                pages::components::bar_chart::view(context, self.state.render_mode)
            }
            ReferencePage::LineChart => {
                pages::components::line_chart::view(context, self.state.render_mode)
            }
            ReferencePage::PieChart => {
                pages::components::pie_chart::view(context, self.state.render_mode)
            }

            // Showcase Gallery (Deprecated / Redirects)
            ReferencePage::ShowcaseButtons => pages::components::button::view(
                context,
                &self.state.button_lab,
                self.state.render_mode,
            ),
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
                self.state.api_key.clone(),
                self.state.ai_provider,
                self.state.enable_exposure,
            ),

            // Details (from Landing)
            ReferencePage::PeakOSDetail => pages::landing::peak_os::view(context, is_mobile),
            ReferencePage::PeakUIDetail => pages::landing::peak_ui::view(context, is_mobile),
            ReferencePage::PeakDBDetail => {
                pages::landing::peak_db::view(context, is_mobile, self.state.db_records.to_vec())
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
