use percent_encoding;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub enum Page {
    Landing,
    // Guide ("Guide" mode)
    #[default]
    #[serde(alias = "introduction", alias = "Introduction")]
    Introduction,
    #[serde(alias = "roadmap", alias = "Roadmap")]
    Roadmap,
    #[serde(alias = "community", alias = "Community")]
    Community,
    #[serde(alias = "intelligence", alias = "Intelligence")]
    Intelligence,

    // Documentation ("Documentation" mode)
    Overview,
    Architecture,
    ProjectStructure,
    Customizations,
    BasicSizing,
    Colors,
    Typography,
    Layout,
    Accessibility,

    // Components -> Atoms
    Text,
    Icon,
    Divider,
    Button,
    Shapes,
    Image,
    Video,
    WebView,

    // Components -> Containers
    VStack,
    HStack,
    ZStack,
    Overlay,
    ScrollView,
    Card,

    // Components -> Navigation
    Sidebar,
    Tabbar,
    Modal,
    NavigationSplit,
    Section,
    DataTable,

    // API Schema
    ApiSchema,

    // Showcase ("Components" mode)
    // Note: These map to plural strings in sidebar: "Buttons", "Inputs"
    ShowcaseButtons,
    ShowcaseInputs,
    ShowcaseToggles,
    ShowcaseSliders,
    ShowcasePickers,

    // Hooks ("Hooks" mode)
    UseState,
    UseEffect,
    UseMemo,
    UseCallback,

    // Core Services
    PeakDB,
    PeakCloud,
    PeakHub,
    SwarmDashboard,

    // Applications
    PeakDesktop,
    PeakOSCore,

    // Settings ("Settings" mode)
    Appearance,
    Scaling,
    Shortcuts,
    About,
    Updates,
    #[serde(alias = "settingsai", alias = "SettingsAI", alias = "settings_ai")]
    SettingsAI,

    // Detail Pages (Landing Extras)
    PeakOSDetail,
    PeakUIDetail,
    PeakDBDetail,
    PeakRelayDetail,
    PeakHubDetail,

    // Fallback
    Unknown(String),
}

impl ToString for Page {
    fn to_string(&self) -> String {
        match self {
            Page::Landing => "Landing".to_string(),
            Page::Introduction => "Introduction".to_string(),
            Page::Roadmap => "Roadmap".to_string(),
            Page::Community => "Community".to_string(),
            Page::Intelligence => "Intelligence".to_string(),
            Page::Overview => "Overview".to_string(),
            Page::Architecture => "Architecture".to_string(),
            Page::ProjectStructure => "Project Structure".to_string(),
            Page::Customizations => "Customizations".to_string(),
            Page::BasicSizing => "Basic Sizing".to_string(),
            Page::Colors => "Colors".to_string(),
            Page::Typography => "Typography".to_string(),
            Page::Layout => "Layout".to_string(),
            Page::Accessibility => "Accessibility".to_string(),

            Page::Text => "Text".to_string(),
            Page::Icon => "Icon".to_string(),
            Page::Divider => "Divider".to_string(),
            Page::Button => "Button".to_string(),
            Page::Shapes => "Shapes".to_string(),
            Page::Image => "Image".to_string(),
            Page::Video => "Video".to_string(),
            Page::WebView => "WebView".to_string(),
            Page::VStack => "VStack".to_string(),
            Page::HStack => "HStack".to_string(),
            Page::ZStack => "ZStack".to_string(),
            Page::Overlay => "Overlay".to_string(),
            Page::ScrollView => "ScrollView".to_string(),
            Page::Card => "Card".to_string(),
            Page::Sidebar => "Sidebar".to_string(),
            Page::Tabbar => "Tabbar".to_string(),
            Page::Modal => "Modal".to_string(),
            Page::NavigationSplit => "NavigationSplit".to_string(),
            Page::Section => "Section".to_string(),
            Page::DataTable => "Data Table".to_string(),

            Page::ApiSchema => "API Schema".to_string(),

            Page::ShowcaseButtons => "Buttons".to_string(),
            Page::ShowcaseInputs => "Inputs".to_string(),
            Page::ShowcaseToggles => "Toggles".to_string(),
            Page::ShowcaseSliders => "Sliders".to_string(),
            Page::ShowcasePickers => "Pickers".to_string(),

            Page::UseState => "use_state".to_string(),
            Page::UseEffect => "use_effect".to_string(),
            Page::UseMemo => "use_memo".to_string(),
            Page::UseCallback => "use_callback".to_string(),

            Page::PeakDB => "PeakDB".to_string(),
            Page::PeakCloud => "PeakCloud".to_string(),
            Page::PeakHub => "Peak Hub".to_string(),
            Page::SwarmDashboard => "Swarm Dashboard".to_string(),
            Page::PeakDesktop => "PeakDesktop".to_string(),
            Page::PeakOSCore => "PeakOS Core".to_string(),

            Page::Appearance => "Appearance".to_string(),
            Page::Scaling => "Scaling".to_string(),
            Page::Shortcuts => "Shortcuts".to_string(),
            Page::About => "About".to_string(),
            Page::Updates => "Updates".to_string(),
            Page::SettingsAI => "AI".to_string(),

            Page::PeakOSDetail => "PeakOS Detail".to_string(),
            Page::PeakUIDetail => "PeakUI Detail".to_string(),
            Page::PeakDBDetail => "PeakDB Detail".to_string(),
            Page::PeakRelayDetail => "PeakRelay Detail".to_string(),
            Page::PeakHubDetail => "PeakHub Detail".to_string(),

            Page::Unknown(s) => s.clone(),
        }
    }
}

impl From<String> for Page {
    fn from(s: String) -> Self {
        let normalized = s.trim().to_lowercase();
        match normalized.as_str() {
            "landing" => Page::Landing,
            "introduction" | "intro" | "start" => Page::Introduction,
            "roadmap" => Page::Roadmap,
            "community" => Page::Community,
            "intelligence" | "ai_overview" => Page::Intelligence,
            "overview" => Page::Overview,
            "architecture" => Page::Architecture,
            "project structure" | "project-structure" | "projectstructure" => {
                Page::ProjectStructure
            }
            "customizations" => Page::Customizations,
            "basic sizing" | "basicsizing" | "sizing" => Page::BasicSizing,
            "colors" => Page::Colors,
            "typography" => Page::Typography,
            "layout" => Page::Layout,

            "text" => Page::Text,
            "icon" => Page::Icon,
            "divider" => Page::Divider,
            "button" => Page::Button,
            "shapes" => Page::Shapes,
            "image" => Page::Image,
            "video" => Page::Video,
            "webview" | "web_view" => Page::WebView,

            "vstack" => Page::VStack,
            "hstack" => Page::HStack,
            "zstack" => Page::ZStack,
            "overlay" => Page::Overlay,
            "scrollview" => Page::ScrollView,
            "card" => Page::Card,

            "sidebar" => Page::Sidebar,
            "tabbar" => Page::Tabbar,
            "modal" => Page::Modal,
            "navigationsplit" | "navigation-split" | "navigation_split" => Page::NavigationSplit,
            "section" => Page::Section,
            "datatable" | "table" | "data-table" => Page::DataTable,

            "api schema" | "apischema" | "api-schema" => Page::ApiSchema,

            "buttons" | "showcasebuttons" => Page::ShowcaseButtons,
            "inputs" | "showcaseinputs" => Page::ShowcaseInputs,
            "toggles" | "showcasetoggles" => Page::ShowcaseToggles,
            "sliders" | "showcasesliders" => Page::ShowcaseSliders,
            "pickers" | "showcasepickers" => Page::ShowcasePickers,

            "use_state" | "usestate" => Page::UseState,
            "use_effect" | "useeffect" => Page::UseEffect,
            "use_memo" | "usememo" => Page::UseMemo,
            "use_callback" | "usecallback" => Page::UseCallback,

            "peakdb" | "db" => Page::PeakDB,
            "peakcloud" | "cloud" => Page::PeakCloud,
            "peakhub" | "hub" => Page::PeakHub,
            "swarm" | "dashboard" | "swarmdashboard" => Page::SwarmDashboard,
            "peakdesktop" => Page::PeakDesktop,
            "peakos core" | "peakoscore" => Page::PeakOSCore,

            "appearance" | "theme" => Page::Appearance,
            "scaling" => Page::Scaling,
            "shortcuts" => Page::Shortcuts,
            "about" => Page::About,
            "updates" => Page::Updates,
            "ai" | "settingsai" | "settings_ai" => Page::SettingsAI,

            "peakosdetail" | "peakos detail" | "peakos-detail" => Page::PeakOSDetail,
            "peakuidetail" | "peakui detail" | "peakui-detail" => Page::PeakUIDetail,
            "peakdbdetail" | "peakdb detail" | "peakdb-detail" => Page::PeakDBDetail,
            "peakrelaydetail" | "peakrelay detail" | "peakrelay-detail" => Page::PeakRelayDetail,
            "peakhubdetail" | "peakhub detail" | "peakhub-detail" => Page::PeakHubDetail,

            _ => {
                // Try to find if any of the to_string() matches (case insensitive)
                for p in Page::all() {
                    if p.to_string().to_lowercase() == normalized {
                        return p.clone();
                    }
                }
                Page::Unknown(s)
            }
        }
    }
}

impl Page {
    pub fn to_path(&self) -> String {
        match self {
            // Landing
            Page::Landing => "/landing".to_string(),
            // Guide
            Page::Introduction => "/guide/introduction".to_string(),
            Page::Roadmap => "/guide/roadmap".to_string(),
            Page::Community => "/guide/community".to_string(),
            Page::Intelligence => "/intelligence".to_string(),

            // Docs
            Page::Overview => "/docs/overview".to_string(),
            Page::Architecture => "/docs/architecture".to_string(),
            Page::ProjectStructure => "/docs/project-structure".to_string(),
            Page::Customizations => "/docs/customizations".to_string(),
            Page::BasicSizing => "/docs/basic-sizing".to_string(),
            Page::Colors => "/docs/colors".to_string(),
            Page::Typography => "/docs/typography".to_string(),
            Page::Layout => "/docs/layout".to_string(),
            Page::Accessibility => "/docs/accessibility".to_string(),

            // Components (Atoms)
            Page::Text => "/components/text".to_string(),
            Page::Icon => "/components/icon".to_string(),
            Page::Divider => "/components/divider".to_string(),
            Page::Button => "/components/button".to_string(),
            Page::Shapes => "/components/shapes".to_string(),
            Page::Image => "/components/image".to_string(),
            Page::Video => "/components/video".to_string(),
            Page::WebView => "/components/webview".to_string(),

            // Components (Containers)
            Page::VStack => "/components/vstack".to_string(),
            Page::HStack => "/components/hstack".to_string(),
            Page::ZStack => "/components/zstack".to_string(),
            Page::Overlay => "/components/overlay".to_string(),
            Page::ScrollView => "/components/scrollview".to_string(),
            Page::Card => "/components/card".to_string(),

            // Components (Navigation)
            Page::Sidebar => "/components/sidebar".to_string(),
            Page::Tabbar => "/components/tabbar".to_string(),
            Page::Modal => "/components/modal".to_string(),
            Page::NavigationSplit => "/components/navigation-split".to_string(),
            Page::Section => "/components/section".to_string(),
            Page::DataTable => "/components/data-table".to_string(),

            Page::ApiSchema => "/api-schema".to_string(),

            // Showcase
            Page::ShowcaseButtons => "/showcase/buttons".to_string(),
            Page::ShowcaseInputs => "/showcase/inputs".to_string(),
            Page::ShowcaseToggles => "/showcase/toggles".to_string(),
            Page::ShowcaseSliders => "/showcase/sliders".to_string(),
            Page::ShowcasePickers => "/showcase/pickers".to_string(),

            // Hooks
            Page::UseState => "/hooks/use-state".to_string(),
            Page::UseEffect => "/hooks/use-effect".to_string(),
            Page::UseMemo => "/hooks/use-memo".to_string(),
            Page::UseCallback => "/hooks/use-callback".to_string(),

            // Core
            Page::PeakDB => "/core/peak-db".to_string(),
            Page::PeakCloud => "/core/peak-cloud".to_string(),
            Page::PeakHub => "/core/peak-hub".to_string(),
            Page::SwarmDashboard => "/swarm-dashboard".to_string(),
            Page::PeakDesktop => "/core/peak-desktop".to_string(),
            Page::PeakOSCore => "/core/peak-os-core".to_string(),

            // Settings
            Page::Appearance => "/settings/appearance".to_string(),
            Page::Scaling => "/settings/scaling".to_string(),
            Page::Shortcuts => "/settings/shortcuts".to_string(),
            Page::About => "/settings/about".to_string(),
            Page::Updates => "/settings/updates".to_string(),
            Page::SettingsAI => "/settings/ai".to_string(),

            // Details
            Page::PeakOSDetail => "/landing/peakos".to_string(),
            Page::PeakUIDetail => "/landing/peakui".to_string(),
            Page::PeakDBDetail => "/landing/peakdb".to_string(),
            Page::PeakRelayDetail => "/landing/peakrelay".to_string(),
            Page::PeakHubDetail => "/landing/peakhub".to_string(),

            // Fallback
            Page::Unknown(_) => "/".to_string(),
        }
    }

    pub fn from_path(path: &str) -> Self {
        let decoded = percent_encoding::percent_decode_str(path.trim()).decode_utf8_lossy();
        let mut path_str = decoded.into_owned();

        // Remove '#' if it accidentally slipped into the start through some other logic
        if path_str.starts_with('#') {
            path_str = path_str[1..].to_string();
        }

        // Remove trailing slash if it's not the only character
        if path_str.len() > 1 && path_str.ends_with('/') {
            path_str.pop();
        }

        let result = match path_str.as_str() {
            "/landing" => Page::Landing,
            "/" | "" => Page::Introduction,

            "/guide/introduction" => Page::Introduction,
            "/guide/roadmap" => Page::Roadmap,
            "/guide/community" => Page::Community,
            "/intelligence" => Page::Intelligence,

            "/docs/overview" => Page::Overview,
            "/docs/architecture" => Page::Architecture,
            "/docs/project-structure" => Page::ProjectStructure,
            "/docs/customizations" => Page::Customizations,
            "/docs/basic-sizing" => Page::BasicSizing,
            "/docs/colors" => Page::Colors,
            "/docs/typography" => Page::Typography,
            "/docs/layout" => Page::Layout,
            "/docs/accessibility" => Page::Accessibility,

            "/components/text" => Page::Text,
            "/components/icon" => Page::Icon,
            "/components/divider" => Page::Divider,
            "/components/button" => Page::Button,
            "/components/shapes" => Page::Shapes,
            "/components/image" => Page::Image,
            "/components/video" => Page::Video,
            "/components/webview" => Page::WebView,

            "/components/vstack" => Page::VStack,
            "/components/hstack" => Page::HStack,
            "/components/zstack" => Page::ZStack,
            "/components/overlay" => Page::Overlay,
            "/components/scrollview" => Page::ScrollView,
            "/components/card" => Page::Card,

            "/components/sidebar" => Page::Sidebar,
            "/components/tabbar" => Page::Tabbar,
            "/components/modal" => Page::Modal,
            "/components/navigation-split" => Page::NavigationSplit,
            "/components/section" => Page::Section,
            "/components/data-table" => Page::DataTable,

            "/api-schema" => Page::ApiSchema,

            "/showcase/buttons" => Page::ShowcaseButtons,
            "/showcase/inputs" => Page::ShowcaseInputs,
            "/showcase/toggles" => Page::ShowcaseToggles,
            "/showcase/sliders" => Page::ShowcaseSliders,
            "/showcase/pickers" => Page::ShowcasePickers,

            "/hooks/use-state" => Page::UseState,
            "/hooks/use-effect" => Page::UseEffect,
            "/hooks/use-memo" => Page::UseMemo,
            "/hooks/use-callback" => Page::UseCallback,

            "/core/peak-db" => Page::PeakDB,
            "/core/peak-cloud" => Page::PeakCloud,
            "/core/peak-hub" => Page::PeakHub,
            "/swarm-dashboard" => Page::SwarmDashboard,
            "/core/peak-desktop" => Page::PeakDesktop,
            "/core/peak-os-core" => Page::PeakOSCore,

            "/settings/appearance" => Page::Appearance,
            "/settings/scaling" => Page::Scaling,
            "/settings/shortcuts" => Page::Shortcuts,
            "/settings/about" => Page::About,
            "/settings/updates" => Page::Updates,
            "/settings/ai" => Page::SettingsAI,

            "/landing/peakos" => Page::PeakOSDetail,
            "/landing/peakui" => Page::PeakUIDetail,
            "/landing/peakdb" => Page::PeakDBDetail,
            "/landing/peakrelay" => Page::PeakRelayDetail,
            "/landing/peakhub" => Page::PeakHubDetail,

            _ => Page::Introduction,
        };

        log::info!(
            "Page::from_path: input='{}', resolved={:?}",
            path_str,
            result
        );
        result
    }

    pub fn navigation_mode(&self) -> String {
        match self {
            Page::Landing
            | Page::Introduction
            | Page::Overview
            | Page::Architecture
            | Page::ProjectStructure
            | Page::Roadmap
            | Page::Community
            | Page::Typography
            | Page::Customizations
            | Page::BasicSizing
            | Page::Colors
            | Page::Layout
            | Page::Accessibility
            | Page::Intelligence
            | Page::PeakOSDetail
            | Page::PeakUIDetail
            | Page::PeakDBDetail
            | Page::PeakRelayDetail
            | Page::PeakHubDetail => "Start".to_string(),
            Page::SettingsAI => "Settings".to_string(),

            Page::Text
            | Page::Icon
            | Page::Divider
            | Page::Button
            | Page::Shapes
            | Page::Image
            | Page::Video
            | Page::WebView
            | Page::VStack
            | Page::HStack
            | Page::ZStack
            | Page::Overlay
            | Page::ScrollView
            | Page::Card
            | Page::Sidebar
            | Page::Tabbar
            | Page::Modal
            | Page::NavigationSplit
            | Page::Section
            | Page::DataTable
            | Page::ShowcaseButtons
            | Page::ShowcaseInputs
            | Page::ShowcaseToggles
            | Page::ShowcaseSliders
            | Page::ShowcasePickers
            | Page::UseState
            | Page::UseEffect
            | Page::UseMemo
            | Page::UseCallback => "Catalog".to_string(),

            Page::ApiSchema
            | Page::PeakDB
            | Page::PeakCloud
            | Page::PeakHub
            | Page::SwarmDashboard
            | Page::PeakDesktop
            | Page::PeakOSCore => "Data".to_string(),

            Page::Appearance | Page::Scaling | Page::Shortcuts | Page::About | Page::Updates => {
                "Settings".to_string()
            }

            Page::Unknown(_) => "Start".to_string(),
        }
    }
    pub fn all() -> &'static [Page] {
        &[
            Page::Landing,
            Page::Introduction,
            Page::Roadmap,
            Page::Community,
            Page::Architecture,
            Page::ProjectStructure,
            Page::Intelligence,
            Page::Typography,
            Page::Customizations,
            Page::BasicSizing,
            Page::Colors,
            Page::Layout,
            Page::Accessibility,
            Page::Text,
            Page::Icon,
            Page::Divider,
            Page::Button,
            Page::Shapes,
            Page::Image,
            Page::Video,
            Page::WebView,
            Page::VStack,
            Page::HStack,
            Page::ZStack,
            Page::Overlay,
            Page::ScrollView,
            Page::Card,
            Page::Sidebar,
            Page::Tabbar,
            Page::Modal,
            Page::NavigationSplit,
            Page::Section,
            Page::DataTable,
            Page::PeakDB,
            Page::PeakCloud,
            Page::PeakHub,
            Page::Appearance,
            Page::SettingsAI,
            Page::About,
            Page::Updates,
            Page::PeakOSDetail,
            Page::PeakUIDetail,
            Page::PeakDBDetail,
            Page::PeakRelayDetail,
            Page::PeakHubDetail,
        ]
    }
}
