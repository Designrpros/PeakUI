use super::super::app::Message;
use super::super::model::Page;
use crate::prelude::*;
use std::collections::HashSet;

pub struct SidebarView {
    pub active_tab: Page,
    pub navigation_mode: String,
    pub expanded_sections: HashSet<String>,
}

impl SidebarView {
    pub fn new(
        active_tab: Page,
        navigation_mode: String,
        expanded_sections: HashSet<String>,
    ) -> Self {
        Self {
            active_tab,
            navigation_mode,
            expanded_sections,
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
        self.base_sidebar(context)
            .push(self.branding_section(context))
            .push(Space::<IcedBackend>::new(0.0.into(), 24.0.into()))
            .push(sidebar_section_header("GETTING STARTED"))
            .push(sidebar_item(
                "Introduction",
                "map",
                Page::Introduction,
                *active_tab == Page::Introduction,
            ))
            .push(sidebar_item(
                "Architecture",
                "boxes",
                Page::Architecture,
                *active_tab == Page::Architecture,
            ))
            .push(sidebar_item(
                "Project Structure",
                "folder-tree",
                Page::ProjectStructure,
                *active_tab == Page::ProjectStructure,
            ))
            .push(sidebar_item(
                "Intelligence",
                "brain-circuit",
                Page::Intelligence,
                *active_tab == Page::Intelligence,
            ))
            .push(Space::<IcedBackend>::new(0.0.into(), 16.0.into()))
            .push(sidebar_section_header("RESOURCES"))
            .push(
                sidebar_item(
                    "Roadmap",
                    "milestone",
                    Page::Roadmap,
                    *active_tab == Page::Roadmap,
                )
                .sudo("Accessing vision-critical roadmap data"),
            )
            .push(sidebar_item(
                "PeakSuite",
                "layout-grid",
                Page::PeakSuite,
                *active_tab == Page::PeakSuite,
            ))
            .push(Space::<IcedBackend>::new(0.0.into(), 16.0.into()))
            .push(sidebar_section_header("FOUNDATIONS"))
            .push(sidebar_item(
                "Typography",
                "type",
                Page::Typography,
                *active_tab == Page::Typography,
            ))
            .push(sidebar_item(
                "Colors",
                "palette",
                Page::Colors,
                *active_tab == Page::Colors,
            ))
            .push(sidebar_item(
                "Theming",
                "palette",
                Page::Customizations,
                *active_tab == Page::Customizations,
            ))
            .push(sidebar_item(
                "Sizing",
                "maximize-2",
                Page::BasicSizing,
                *active_tab == Page::BasicSizing,
            ))
            .push(sidebar_item(
                "Layout",
                "layout-grid",
                Page::Layout,
                *active_tab == Page::Layout,
            ))
            .push(sidebar_item(
                "Accessibility",
                "accessibility",
                Page::Accessibility,
                *active_tab == Page::Accessibility,
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
                Page::Text,
                *active_tab == Page::Text,
            ))
            .push(sidebar_item(
                "Icon",
                "sparkles",
                Page::Icon,
                *active_tab == Page::Icon,
            ))
            .push(sidebar_item(
                "Divider",
                "minus",
                Page::Divider,
                *active_tab == Page::Divider,
            ))
            .push(sidebar_item(
                "Button",
                "square",
                Page::Button,
                *active_tab == Page::Button,
            ))
            .push(sidebar_item(
                "Shapes",
                "shapes",
                Page::Shapes,
                *active_tab == Page::Shapes,
            ))
            .push(sidebar_item(
                "Image",
                "image",
                Page::Image,
                *active_tab == Page::Image,
            ))
            .push(sidebar_item(
                "Video",
                "video",
                Page::Video,
                *active_tab == Page::Video,
            ))
            .push(sidebar_item(
                "WebView",
                "globe",
                Page::WebView,
                *active_tab == Page::WebView,
            ))
            .push(Space::<IcedBackend>::new(0.0.into(), 16.0.into()))
            .push(sidebar_section_header("CONTAINERS"))
            .push(sidebar_item(
                "VStack",
                "rows-3",
                Page::VStack,
                *active_tab == Page::VStack,
            ))
            .push(sidebar_item(
                "HStack",
                "columns-2",
                Page::HStack,
                *active_tab == Page::HStack,
            ))
            .push(sidebar_item(
                "ZStack",
                "layers",
                Page::ZStack,
                *active_tab == Page::ZStack,
            ))
            .push(sidebar_item(
                "Overlay",
                "copy",
                Page::Overlay,
                *active_tab == Page::Overlay,
            ))
            .push(sidebar_item(
                "ScrollView",
                "move-3d",
                Page::ScrollView,
                *active_tab == Page::ScrollView,
            ))
            .push(sidebar_item(
                "Card",
                "credit-card",
                Page::Card,
                *active_tab == Page::Card,
            ))
            .push(sidebar_item(
                "Section",
                "package",
                Page::Section,
                *active_tab == Page::Section,
            ))
            .push(sidebar_item(
                "Data Table",
                "table",
                Page::DataTable,
                *active_tab == Page::DataTable,
            ))
            .push(sidebar_item(
                "Bar Chart",
                "chart-bar",
                Page::BarChart,
                *active_tab == Page::BarChart,
            ))
            .push(sidebar_item(
                "Line Chart",
                "chart-line",
                Page::LineChart,
                *active_tab == Page::LineChart,
            ))
            .push(sidebar_item(
                "Pie Chart",
                "chart-pie",
                Page::PieChart,
                *active_tab == Page::PieChart,
            ))
            .push(Space::<IcedBackend>::new(0.0.into(), 16.0.into()))
            .push(sidebar_section_header("FEEDBACK"))
            .push(sidebar_item(
                "Sidebar",
                "panel-left",
                Page::Sidebar,
                *active_tab == Page::Sidebar,
            ))
            .push(sidebar_item(
                "Tabbar",
                "layout-panel-top",
                Page::Tabbar,
                *active_tab == Page::Tabbar,
            ))
            .push(sidebar_item(
                "Nav Split",
                "columns-3",
                Page::NavigationSplit,
                *active_tab == Page::NavigationSplit,
            ))
            .push(Space::<IcedBackend>::new(0.0.into(), 16.0.into()))
            .push(sidebar_section_header("STATE MANAGEMENT"))
            .push(sidebar_item(
                "State",
                "zap",
                Page::UseState,
                *active_tab == Page::UseState,
            ))
            .push(sidebar_item(
                "Effects",
                "activity",
                Page::UseEffect,
                *active_tab == Page::UseEffect,
            ))
            .push(sidebar_item(
                "Memo",
                "zap",
                Page::UseMemo,
                *active_tab == Page::UseMemo,
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
                Page::PeakDB,
                *active_tab == Page::PeakDB,
            ))
            .push(sidebar_item(
                "Swarm Dashboard",
                "layout-dashboard",
                Page::SwarmDashboard,
                *active_tab == Page::SwarmDashboard,
            ))
            .push(Space::<IcedBackend>::new(0.0.into(), 16.0.into()))
            .push(sidebar_section_header("PEAKCLOUD"))
            .push(sidebar_item(
                "PeakCloud",
                "cloud-cog",
                Page::PeakCloud,
                *active_tab == Page::PeakCloud,
            ))
            .push(Space::<IcedBackend>::new(0.0.into(), 16.0.into()))
            .push(sidebar_section_header("PEAK HUB"))
            .push(sidebar_item(
                "PeakOS Core",
                "cpu",
                Page::PeakOSCore,
                *active_tab == Page::PeakOSCore,
            ))
            .push(sidebar_item(
                "Peak Hub",
                "activity",
                Page::PeakHub,
                *active_tab == Page::PeakHub,
            ))
    }

    fn view_settings_sidebar(&self, context: &Context) -> VStack<Message, IcedBackend> {
        let active_tab = &self.active_tab;
        self.base_sidebar(context)
            .push(self.branding_section(context))
            .push(Space::<IcedBackend>::new(0.0.into(), 24.0.into()))
            .push(sidebar_section_header("USER PREFERENCES"))
            .push(sidebar_item(
                "Appearance",
                "sun-medium",
                Page::Appearance,
                *active_tab == Page::Appearance,
            ))
            .push(sidebar_item(
                "Scaling",
                "maximize-2",
                Page::Scaling,
                *active_tab == Page::Scaling,
            ))
            .push(sidebar_item(
                "Shortcuts",
                "command",
                Page::Shortcuts,
                *active_tab == Page::Shortcuts,
            ))
            .push(sidebar_item(
                "AI Assistant",
                "brain-circuit",
                Page::SettingsAI,
                *active_tab == Page::SettingsAI,
            ))
            .push(Space::<IcedBackend>::new(0.0.into(), 16.0.into()))
            .push(sidebar_section_header("SYSTEM"))
            .push(sidebar_item(
                "About",
                "info",
                Page::About,
                *active_tab == Page::About,
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
    page: Page,
    active: bool,
) -> impl View<Message, IcedBackend> {
    SidebarItem::new(label, icon, page, active)
}

struct SidebarItem {
    label: String,
    icon: String,
    page: Page,
    active: bool,
}

impl SidebarItem {
    fn new(label: impl Into<String>, icon: impl Into<String>, page: Page, active: bool) -> Self {
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
        .on_press(Message::SetTab(page_clone))
        .view(context)
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode::new("sidebar_item")
            .with_label(format!("{} (page={:?})", self.label, self.page))
            .with_content(if self.active { "ACTIVE" } else { "" })
    }
}
