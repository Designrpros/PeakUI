use super::super::app::{Message, ShellMessage};
use super::super::pages;
use super::state::ViewState;
use crate::prelude::*;
use crate::reference::AppPage as ReferenceAppPage;

pub struct CanvasView {
    pub state: ViewState,
}

use crate::reference::AppPageResult;

impl CanvasView {
    pub fn new(state: ViewState) -> Self {
        Self { state }
    }

    pub fn render_page(&self, context: &Context) -> AppPageResult {
        let is_mobile = context.size.width < 900.0;

        let page = match &self.state.shell.active_tab {
            // Guide
            ReferenceAppPage::Introduction => pages::guide::introduction::view(context, is_mobile),
            ReferenceAppPage::Architecture => pages::docs::architecture::view(context, is_mobile),
            ReferenceAppPage::ProjectStructure => {
                pages::docs::project_structure::view(context, is_mobile)
            }
            ReferenceAppPage::Accessibility => pages::docs::accessibility::view(
                context,
                &self.state.labs.accessibility,
                self.state.labs.render_mode,
            ),
            ReferenceAppPage::SideEffects => pages::docs::side_effects::view(context, is_mobile),

            ReferenceAppPage::Roadmap => pages::guide::roadmap::view(context, is_mobile),
            #[cfg(feature = "intelligence")]
            ReferenceAppPage::Intelligence => pages::guide::intelligence::view(
                context,
                is_mobile,
                self.state.intelligence.api_key.clone(),
            ),

            // Ecosystem
            ReferenceAppPage::PeakDesktop => pages::core::peak_desktop::view(context, is_mobile),

            // Legacy
            #[cfg(feature = "intelligence")]
            ReferenceAppPage::ApiSchema => pages::docs::api_schema::view(context, is_mobile),
            ReferenceAppPage::PeakSuite => pages::guide::peak_suite::view(context, is_mobile),
            ReferenceAppPage::PeakDB => pages::core::peak_db::view(context, is_mobile),
            ReferenceAppPage::PeakCloud => pages::core::peak_cloud::view(context, is_mobile),
            ReferenceAppPage::PeakHub => pages::core::peak_hub::view(context, is_mobile),

            // Concepts (Overview is legacy/fallback)
            ReferenceAppPage::Overview => pages::guide::introduction::view(context, is_mobile),
            ReferenceAppPage::Customizations => {
                pages::docs::customizations::view(context, self.state.labs.render_mode)
            }
            ReferenceAppPage::BasicSizing => pages::docs::sizing::view(
                context,
                &self.state.labs.sizing,
                self.state.labs.render_mode,
            ),
            ReferenceAppPage::Colors => {
                pages::docs::colors::view(context, self.state.labs.render_mode)
            }
            ReferenceAppPage::Typography => pages::docs::typography::view(
                context,
                &self.state.labs.typography,
                self.state.labs.render_mode,
            ),
            ReferenceAppPage::Layout => pages::docs::layout::view(
                context,
                &self.state.labs.layout,
                self.state.labs.render_mode,
            ),

            // Atoms (Phase 3/4)
            ReferenceAppPage::Text => pages::components::text::view(
                context,
                &self.state.labs.typography,
                self.state.labs.render_mode,
            ),
            ReferenceAppPage::Icon => pages::components::icon::view(
                context,
                &self.state.labs.icon,
                self.state.labs.render_mode,
                self.state.shell.search_query.clone(),
                self.state.icon_limit,
            ),
            ReferenceAppPage::Emoji => pages::components::emoji::view(
                context,
                &self.state.labs.emoji,
                self.state.labs.render_mode,
                self.state.shell.search_query.clone(),
            ),
            ReferenceAppPage::Button => pages::components::button::view(
                context,
                &self.state.labs.button,
                self.state.labs.render_mode,
            ),
            ReferenceAppPage::Shapes => {
                pages::components::shapes::view(context, self.state.labs.render_mode)
            }
            ReferenceAppPage::Image => {
                pages::components::image::view(context, self.state.labs.render_mode)
            }
            ReferenceAppPage::Video => {
                pages::components::video::view(context, self.state.labs.render_mode)
            }
            ReferenceAppPage::WebView => {
                pages::components::web_view::view(context, self.state.labs.render_mode)
            }
            ReferenceAppPage::Divider => {
                pages::components::divider::view(context, self.state.labs.render_mode)
            }

            // Containers (Phase 4)
            ReferenceAppPage::Spacer => pages::components::spacer::view(
                context,
                &self.state.labs.spacer,
                self.state.labs.render_mode,
            ),
            ReferenceAppPage::VStack => {
                pages::components::vstack::view(context, self.state.labs.render_mode)
            }
            ReferenceAppPage::HStack => {
                pages::components::hstack::view(context, self.state.labs.render_mode)
            }
            ReferenceAppPage::ZStack => {
                pages::components::zstack::view(context, self.state.labs.render_mode)
            }
            ReferenceAppPage::Overlay => {
                pages::components::overlay::view(context, self.state.labs.render_mode)
            }
            ReferenceAppPage::ScrollView => {
                pages::components::scroll_view::view(context, self.state.labs.render_mode)
            }
            ReferenceAppPage::Card => {
                pages::components::card::view(context, self.state.labs.render_mode)
            }

            // Navigation (Phase 4)
            ReferenceAppPage::Sidebar => {
                pages::components::sidebar_doc::view(context, self.state.labs.render_mode)
            }
            ReferenceAppPage::Tabbar => {
                pages::components::tabbar_doc::view(context, self.state.labs.render_mode)
            }
            ReferenceAppPage::Modal => {
                pages::components::modal_doc::view(context, self.state.labs.render_mode)
            }
            ReferenceAppPage::NavigationSplit => {
                pages::components::navigation_split::view(context, self.state.labs.render_mode)
            }
            ReferenceAppPage::Section => {
                pages::components::section::view(context, self.state.labs.render_mode)
            }
            ReferenceAppPage::DataTable => {
                pages::components::data_table::view(context, self.state.labs.render_mode)
            }
            ReferenceAppPage::BarChart => {
                pages::components::bar_chart::view(context, self.state.labs.render_mode)
            }
            ReferenceAppPage::LineChart => {
                pages::components::line_chart::view(context, self.state.labs.render_mode)
            }
            ReferenceAppPage::PieChart => {
                pages::components::pie_chart::view(context, self.state.labs.render_mode)
            }

            // Showcase Gallery (Deprecated / Redirects)
            ReferenceAppPage::ShowcaseButtons => pages::components::button::view(
                context,
                &self.state.labs.button,
                self.state.labs.render_mode,
            ),
            ReferenceAppPage::ShowcaseInputs => {
                pages::components::inputs::view(context, self.state.labs.render_mode)
            }
            ReferenceAppPage::ShowcaseToggles
            | ReferenceAppPage::ShowcaseSliders
            | ReferenceAppPage::ShowcasePickers => {
                pages::guide::introduction::view(context, is_mobile)
            }

            // Hooks Gallery
            ReferenceAppPage::UseState => pages::hooks::overview::view(context, is_mobile),
            ReferenceAppPage::UseEffect => pages::docs::side_effects::view(context, is_mobile),
            ReferenceAppPage::UseMemo | ReferenceAppPage::UseCallback => {
                pages::docs::performance::view(context, is_mobile)
            }

            // Settings Gallery
            ReferenceAppPage::Appearance => pages::settings::appearance::view(context, is_mobile),
            ReferenceAppPage::Scaling => pages::settings::scaling::view(context, is_mobile),
            ReferenceAppPage::Shortcuts => pages::settings::shortcuts::view(context, is_mobile),
            ReferenceAppPage::About | ReferenceAppPage::Updates => {
                pages::settings::about::view(context, is_mobile)
            }
            #[cfg(feature = "intelligence")]
            ReferenceAppPage::SettingsAI => {
                let state_json = None;
                pages::settings::ai::view(
                    context,
                    is_mobile,
                    self.state.intelligence.api_key.clone(),
                    self.state.intelligence.ai_provider,
                    self.state.interaction.enable_exposure,
                    state_json,
                )
            }

            // Details (from Landing)
            ReferenceAppPage::PeakOSDetail => pages::landing::peak_os::view(context, is_mobile),
            ReferenceAppPage::PeakUIDetail => pages::landing::peak_ui::view(context, is_mobile),
            ReferenceAppPage::PeakDBDetail => {
                pages::landing::peak_db::view(context, is_mobile, self.state.db_records.to_vec())
            }
            ReferenceAppPage::PeakRelayDetail => {
                pages::landing::peak_relay::view(context, is_mobile)
            }
            ReferenceAppPage::PeakHubDetail => pages::landing::peak_hub::view(context, is_mobile),

            // Fallback
            ReferenceAppPage::Landing | ReferenceAppPage::Unknown(_) => {
                pages::guide::introduction::view(context, is_mobile)
            }
        };

        if is_mobile {
            page.sidebar_toggle(Message::Shell(ShellMessage::SetNavigationMode(
                "Sidebar".to_string(),
            )))
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
