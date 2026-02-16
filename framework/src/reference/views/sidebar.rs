use super::super::app::{Message, ShellMessage};
use crate::prelude::*;
use crate::reference::AppPage;

pub struct SidebarView {
    pub active_tab: AppPage,
    pub navigation_mode: String,
}

impl SidebarView {
    pub fn new(active_tab: AppPage, navigation_mode: String) -> Self {
        Self {
            active_tab,
            navigation_mode,
        }
    }
}

impl SidebarView {
    fn base_sidebar(&self, context: &Context) -> VStack<Message, IcedBackend> {
        VStack::<Message, IcedBackend>::new_generic()
            .width(Length::Fill)
            .spacing(4.0)
            .padding(Padding {
                top: context.safe_area.top.max(32.0),
                right: 16.0,
                bottom: context.safe_area.bottom.max(40.0),
                left: 16.0,
            })
    }

    fn branding_section(&self, context: &Context) -> HStack<Message, IcedBackend> {
        let is_dark = context.theme.tone == ThemeTone::Dark;
        let logo_path = if is_dark {
            "/assets/peak_logo_dark.png"
        } else {
            "/assets/peak_logo.png"
        };

        HStack::<Message, IcedBackend>::new_generic()
            .spacing(12.0)
            .align_y(iced::Alignment::Center)
            .push(
                Image::<IcedBackend>::new(logo_path)
                    .width(Length::Fixed(48.0))
                    .height(Length::Fixed(24.0)),
            )
            .push(
                Text::<IcedBackend>::new("PeakUI")
                    .headline()
                    .bold()
                    .color(context.theme.colors.text_primary),
            )
    }

    fn view_guide_sidebar(&self, context: &Context) -> VStack<Message, IcedBackend> {
        let active_tab = &self.active_tab;
        let guide = self
            .base_sidebar(context)
            .push(self.branding_section(context))
            .push(Space::<IcedBackend>::new(0.0.into(), 24.0.into()))
            .push(sidebar_section_header("GETTING STARTED"))
            .push(sidebar_item(
                "Introduction",
                "map",
                AppPage::Introduction,
                *active_tab == AppPage::Introduction,
            ))
            .push(sidebar_item(
                "Architecture",
                "boxes",
                AppPage::Architecture,
                *active_tab == AppPage::Architecture,
            ))
            .push(sidebar_item(
                "Project Structure",
                "folder-tree",
                AppPage::ProjectStructure,
                *active_tab == AppPage::ProjectStructure,
            ));

        #[cfg(feature = "intelligence")]
        let guide = guide.push(sidebar_item(
            "Intelligence",
            "brain-circuit",
            AppPage::Intelligence,
            *active_tab == AppPage::Intelligence,
        ));

        guide
            .push(Space::<IcedBackend>::new(0.0.into(), 16.0.into()))
            .push(sidebar_section_header("RESOURCES"))
            .push(
                sidebar_item(
                    "Roadmap",
                    "milestone",
                    AppPage::Roadmap,
                    *active_tab == AppPage::Roadmap,
                )
                .sudo("Accessing vision-critical roadmap data"),
            )
            .push(sidebar_item(
                "PeakSuite",
                "layout-grid",
                AppPage::PeakSuite,
                *active_tab == AppPage::PeakSuite,
            ))
            .push(Space::<IcedBackend>::new(0.0.into(), 16.0.into()))
            .push(sidebar_section_header("FOUNDATIONS"))
            .push(sidebar_item(
                "Typography",
                "type",
                AppPage::Typography,
                *active_tab == AppPage::Typography,
            ))
            .push(sidebar_item(
                "Colors",
                "palette",
                AppPage::Colors,
                *active_tab == AppPage::Colors,
            ))
            .push(sidebar_item(
                "Theming",
                "palette",
                AppPage::Customizations,
                *active_tab == AppPage::Customizations,
            ))
            .push(sidebar_item(
                "Sizing",
                "maximize-2",
                AppPage::BasicSizing,
                *active_tab == AppPage::BasicSizing,
            ))
            .push(sidebar_item(
                "Layout",
                "layout-grid",
                AppPage::Layout,
                *active_tab == AppPage::Layout,
            ))
            .push(sidebar_item(
                "Accessibility",
                "accessibility",
                AppPage::Accessibility,
                *active_tab == AppPage::Accessibility,
            ))
            .push(sidebar_item(
                "Side Effects",
                "zap",
                AppPage::SideEffects,
                *active_tab == AppPage::SideEffects,
            ))
    }

    fn view_components_sidebar(&self, context: &Context) -> VStack<Message, IcedBackend> {
        let active_tab = &self.active_tab;
        self.base_sidebar(context)
            .push(self.branding_section(context))
            .push(Space::<IcedBackend>::new(0.0.into(), 24.0.into()))
            .push(sidebar_section_header("ATOMS"))
            .push(sidebar_item(
                "Text",
                "type",
                AppPage::Text,
                *active_tab == AppPage::Text,
            ))
            .push(sidebar_item(
                "Icon",
                "sparkles",
                AppPage::Icon,
                *active_tab == AppPage::Icon,
            ))
            .push(sidebar_item(
                "Emoji",
                "smile",
                AppPage::Emoji,
                *active_tab == AppPage::Emoji,
            ))
            .push(sidebar_item(
                "Divider",
                "minus",
                AppPage::Divider,
                *active_tab == AppPage::Divider,
            ))
            .push(sidebar_item(
                "Button",
                "square",
                AppPage::Button,
                *active_tab == AppPage::Button,
            ))
            .push(sidebar_item(
                "Shapes",
                "shapes",
                AppPage::Shapes,
                *active_tab == AppPage::Shapes,
            ))
            .push(sidebar_item(
                "Image",
                "image",
                AppPage::Image,
                *active_tab == AppPage::Image,
            ))
            .push(sidebar_item(
                "Video",
                "video",
                AppPage::Video,
                *active_tab == AppPage::Video,
            ))
            .push(sidebar_item(
                "WebView",
                "globe",
                AppPage::WebView,
                *active_tab == AppPage::WebView,
            ))
            .push(Space::<IcedBackend>::new(0.0.into(), 16.0.into()))
            .push(sidebar_section_header("CONTAINERS"))
            .push(sidebar_item(
                "Spacer",
                "move-vertical",
                AppPage::Spacer,
                *active_tab == AppPage::Spacer,
            ))
            .push(sidebar_item(
                "VStack",
                "rows-3",
                AppPage::VStack,
                *active_tab == AppPage::VStack,
            ))
            .push(sidebar_item(
                "HStack",
                "columns-2",
                AppPage::HStack,
                *active_tab == AppPage::HStack,
            ))
            .push(sidebar_item(
                "ZStack",
                "layers",
                AppPage::ZStack,
                *active_tab == AppPage::ZStack,
            ))
            .push(sidebar_item(
                "Overlay",
                "copy",
                AppPage::Overlay,
                *active_tab == AppPage::Overlay,
            ))
            .push(sidebar_item(
                "ScrollView",
                "move-3d",
                AppPage::ScrollView,
                *active_tab == AppPage::ScrollView,
            ))
            .push(sidebar_item(
                "Card",
                "credit-card",
                AppPage::Card,
                *active_tab == AppPage::Card,
            ))
            .push(sidebar_item(
                "Section",
                "package",
                AppPage::Section,
                *active_tab == AppPage::Section,
            ))
            .push(sidebar_item(
                "Data Table",
                "table",
                AppPage::DataTable,
                *active_tab == AppPage::DataTable,
            ))
            .push(sidebar_item(
                "Bar Chart",
                "chart-bar",
                AppPage::BarChart,
                *active_tab == AppPage::BarChart,
            ))
            .push(sidebar_item(
                "Line Chart",
                "chart-line",
                AppPage::LineChart,
                *active_tab == AppPage::LineChart,
            ))
            .push(sidebar_item(
                "Pie Chart",
                "chart-pie",
                AppPage::PieChart,
                *active_tab == AppPage::PieChart,
            ))
            .push(Space::<IcedBackend>::new(0.0.into(), 16.0.into()))
            .push(sidebar_section_header("FEEDBACK"))
            .push(sidebar_item(
                "Sidebar",
                "panel-left",
                AppPage::Sidebar,
                *active_tab == AppPage::Sidebar,
            ))
            .push(sidebar_item(
                "Tabbar",
                "layout-panel-top",
                AppPage::Tabbar,
                *active_tab == AppPage::Tabbar,
            ))
            .push(sidebar_item(
                "Nav Split",
                "columns-3",
                AppPage::NavigationSplit,
                *active_tab == AppPage::NavigationSplit,
            ))
            .push(Space::<IcedBackend>::new(0.0.into(), 16.0.into()))
            .push(sidebar_section_header("STATE MANAGEMENT"))
            .push(sidebar_item(
                "State",
                "zap",
                AppPage::UseState,
                *active_tab == AppPage::UseState,
            ))
            .push(sidebar_item(
                "Effects",
                "activity",
                AppPage::UseEffect,
                *active_tab == AppPage::UseEffect,
            ))
            .push(sidebar_item(
                "Memo",
                "zap",
                AppPage::UseMemo,
                *active_tab == AppPage::UseMemo,
            ))
    }

    fn view_ecosystem_sidebar(&self, context: &Context) -> VStack<Message, IcedBackend> {
        let active_tab = &self.active_tab;
        self.base_sidebar(context)
            .push(self.branding_section(context))
            .push(Space::<IcedBackend>::new(0.0.into(), 24.0.into()))
            .push(sidebar_section_header("PEAKDB"))
            .push(sidebar_item(
                "PeakDB",
                "database-backup",
                AppPage::PeakDB,
                *active_tab == AppPage::PeakDB,
            ))
            .push(Space::<IcedBackend>::new(0.0.into(), 16.0.into()))
            .push(sidebar_section_header("PEAKCLOUD"))
            .push(sidebar_item(
                "PeakCloud",
                "cloud-cog",
                AppPage::PeakCloud,
                *active_tab == AppPage::PeakCloud,
            ))
            .push(Space::<IcedBackend>::new(0.0.into(), 16.0.into()))
            .push(sidebar_section_header("PEAK HUB"))
            .push(sidebar_item(
                "Peak Hub",
                "activity",
                AppPage::PeakHub,
                *active_tab == AppPage::PeakHub,
            ))
    }

    fn view_settings_sidebar(&self, context: &Context) -> VStack<Message, IcedBackend> {
        let active_tab = &self.active_tab;
        let settings = self
            .base_sidebar(context)
            .push(self.branding_section(context))
            .push(Space::<IcedBackend>::new(0.0.into(), 24.0.into()))
            .push(sidebar_section_header("USER PREFERENCES"))
            .push(sidebar_item(
                "Appearance",
                "sun-medium",
                AppPage::Appearance,
                *active_tab == AppPage::Appearance,
            ))
            .push(sidebar_item(
                "Scaling",
                "maximize-2",
                AppPage::Scaling,
                *active_tab == AppPage::Scaling,
            ))
            .push(sidebar_item(
                "Shortcuts",
                "command",
                AppPage::Shortcuts,
                *active_tab == AppPage::Shortcuts,
            ));

        #[cfg(feature = "intelligence")]
        let settings = settings.push(sidebar_item(
            "AI Assistant",
            "brain-circuit",
            AppPage::SettingsAI,
            *active_tab == AppPage::SettingsAI,
        ));

        settings
            .push(Space::<IcedBackend>::new(0.0.into(), 16.0.into()))
            .push(sidebar_section_header("SYSTEM"))
            .push(sidebar_item(
                "About",
                "info",
                AppPage::About,
                *active_tab == AppPage::About,
            ))
    }
}

impl View<Message, IcedBackend> for SidebarView {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let theme = context.theme;

        let content = match self.navigation_mode.to_lowercase().trim() {
            "start" | "guide" | "introduction" | "documentation" => {
                self.view_guide_sidebar(context)
            }
            "catalog" | "components" | "labs" => self.view_components_sidebar(context),
            "data" | "ecosystem" | "services" => self.view_ecosystem_sidebar(context),
            "settings" | "preferences" => self.view_settings_sidebar(context),
            _ => {
                log::warn!(
                    "SidebarView: Unknown mode '{}', falling back to guide",
                    self.navigation_mode
                );
                self.view_guide_sidebar(context)
            }
        };

        container(ScrollView::new(content.width(Length::Fill)).view(context))
            .width(Length::Fill)
            .height(Length::Fill)
            .style({
                let bg_color = theme.colors.surface.scale_alpha(0.5);
                let border_color = theme.colors.border.scale_alpha(0.05);
                move |_| container::Style {
                    background: Some(bg_color.into()),
                    border: Border {
                        color: border_color,
                        width: 1.0,
                        radius: 0.0.into(),
                    },
                    ..Default::default()
                }
            })
            .into()
    }

    fn describe(&self, context: &Context) -> crate::core::SemanticNode {
        let content = match self.navigation_mode.as_str() {
            "Start" => self.view_guide_sidebar(context),
            "Catalog" => self.view_components_sidebar(context),
            "Data" | "Ecosystem" => self.view_ecosystem_sidebar(context),
            "Settings" => self.view_settings_sidebar(context),
            _ => self.base_sidebar(context),
        };

        crate::core::SemanticNode::new("sidebar")
            .with_label(self.navigation_mode.clone())
            .push_child(content.describe(context))
    }
}

fn sidebar_section_header(label: &str) -> impl View<Message, IcedBackend> {
    let label = label.to_string();
    ProxyView::new(move |ctx| {
        container(
            Text::<IcedBackend>::new(label.clone())
                .caption2()
                .bold()
                .secondary()
                .view(ctx),
        )
        .padding(Padding::from([8, 12]))
        .width(Length::Fill)
        .into()
    })
}

fn sidebar_item(
    label: impl Into<String>,
    icon: impl Into<String>,
    page: AppPage,
    active: bool,
) -> impl View<Message, IcedBackend> {
    SidebarItem::new(label, icon, page, active)
}

struct SidebarItem {
    label: String,
    icon: String,
    page: AppPage,
    active: bool,
}

impl SidebarItem {
    fn new(label: impl Into<String>, icon: impl Into<String>, page: AppPage, active: bool) -> Self {
        Self {
            label: label.into(),
            icon: icon.into(),
            page,
            active,
        }
    }
}

impl View<Message, IcedBackend> for SidebarItem {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let theme = context.theme;
        let active = self.active;
        let page_clone = self.page.clone();
        let _label_clone = self.label.clone();

        log::trace!(
            "📱 SidebarItem::view - Rendering: {} (active: {})",
            self.label,
            active
        );

        Button::new(
            HStack::new_generic()
                .width(Length::Fill)
                .spacing(12.0)
                .padding(Padding::from([6, 12]))
                .align_y(Alignment::Center)
                .push(
                    Icon::<IcedBackend>::new(self.icon.clone())
                        .size(14.0)
                        .color(if active {
                            theme.colors.primary
                        } else {
                            theme.colors.text_secondary
                        }),
                )
                .push(if active {
                    Text::<IcedBackend>::new(self.label.clone())
                        .caption1()
                        .bold()
                        .width(Length::Fill)
                        .align_start()
                } else {
                    Text::<IcedBackend>::new(self.label.clone())
                        .caption1()
                        .width(Length::Fill)
                        .align_start()
                }),
        )
        .variant(if active {
            Variant::Soft
        } else {
            Variant::Ghost
        })
        .width(Length::Fill)
        .on_press(Message::Shell(ShellMessage::SetTab(page_clone)))
        .view(context)
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode::new("sidebar_item")
            .with_label(format!("{} (page={:?})", self.label, self.page))
            .with_content(if self.active { "ACTIVE" } else { "" })
    }
}
