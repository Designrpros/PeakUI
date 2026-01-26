use percent_encoding;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
pub enum Page {
    // Guide ("Guide" mode)
    #[default]
    Introduction,
    Roadmap,
    Community,

    // Documentation ("Documentation" mode)
    Overview,
    Architecture,
    ProjectStructure,
    Customizations,
    BasicSizing,
    Typography,
    Layout,

    // Components -> Atoms
    Text,
    Icon,
    Divider,
    Button,
    Shapes,

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

    // Applications
    PeakDesktop,
    PeakOSCore,

    // Settings ("Settings" mode)
    Appearance,
    Scaling,
    Shortcuts,
    About,
    Updates,

    // Fallback
    Unknown(String),
}

impl ToString for Page {
    fn to_string(&self) -> String {
        match self {
            Page::Introduction => "Introduction".to_string(),
            Page::Roadmap => "Roadmap".to_string(),
            Page::Community => "Community".to_string(),
            Page::Overview => "Overview".to_string(),
            Page::Architecture => "Architecture".to_string(),
            Page::ProjectStructure => "Project Structure".to_string(),
            Page::Customizations => "Customizations".to_string(),
            Page::BasicSizing => "Basic Sizing".to_string(),
            Page::Typography => "Typography".to_string(),
            Page::Layout => "Layout".to_string(),

            Page::Text => "Text".to_string(),
            Page::Icon => "Icon".to_string(),
            Page::Divider => "Divider".to_string(),
            Page::Button => "Button".to_string(),
            Page::Shapes => "Shapes".to_string(),
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
            Page::PeakDesktop => "PeakDesktop".to_string(),
            Page::PeakOSCore => "PeakOS Core".to_string(),

            Page::Appearance => "Appearance".to_string(),
            Page::Scaling => "Scaling".to_string(),
            Page::Shortcuts => "Shortcuts".to_string(),
            Page::About => "About".to_string(),
            Page::Updates => "Updates".to_string(),

            Page::Unknown(s) => s.clone(),
        }
    }
}

impl From<String> for Page {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Introduction" => Page::Introduction,
            "Roadmap" => Page::Roadmap,
            "Community" => Page::Community,
            "Overview" => Page::Overview,
            "Architecture" => Page::Architecture,
            "Project Structure" => Page::ProjectStructure,
            "Customizations" => Page::Customizations,
            "Basic Sizing" => Page::BasicSizing,
            "Typography" => Page::Typography,
            "Layout" => Page::Layout,

            "Text" => Page::Text,
            "Icon" => Page::Icon,
            "Divider" => Page::Divider,
            "Button" => Page::Button,
            "Shapes" => Page::Shapes,

            "VStack" => Page::VStack,
            "HStack" => Page::HStack,
            "ZStack" => Page::ZStack,
            "Overlay" => Page::Overlay,
            "ScrollView" => Page::ScrollView,
            "Card" => Page::Card,

            "Sidebar" => Page::Sidebar,
            "Tabbar" => Page::Tabbar,
            "Modal" => Page::Modal,
            "NavigationSplit" => Page::NavigationSplit,
            "Section" => Page::Section,

            "API Schema" => Page::ApiSchema,

            "Buttons" => Page::ShowcaseButtons,
            "Inputs" => Page::ShowcaseInputs,
            "Toggles" => Page::ShowcaseToggles,
            "Sliders" => Page::ShowcaseSliders,
            "Pickers" => Page::ShowcasePickers,

            "use_state" => Page::UseState,
            "use_effect" => Page::UseEffect,
            "use_memo" => Page::UseMemo,
            "use_callback" => Page::UseCallback,

            "PeakDB" => Page::PeakDB,
            "PeakCloud" => Page::PeakCloud,
            "PeakDesktop" => Page::PeakDesktop,
            "PeakOS Core" => Page::PeakOSCore,

            "Appearance" => Page::Appearance,
            "Scaling" => Page::Scaling,
            "Shortcuts" => Page::Shortcuts,
            "About" => Page::About,
            "Updates" => Page::Updates,

            _ => Page::Unknown(s),
        }
    }
}

impl Page {
    pub fn to_path(&self) -> String {
        match self {
            // Guide
            Page::Introduction => "/".to_string(),
            Page::Roadmap => "/guide/roadmap".to_string(),
            Page::Community => "/guide/community".to_string(),

            // Docs
            Page::Overview => "/docs/overview".to_string(),
            Page::Architecture => "/docs/architecture".to_string(),
            Page::ProjectStructure => "/docs/project-structure".to_string(),
            Page::Customizations => "/docs/customizations".to_string(),
            Page::BasicSizing => "/docs/basic-sizing".to_string(),
            Page::Typography => "/docs/typography".to_string(),
            Page::Layout => "/docs/layout".to_string(),

            // Components (Atoms)
            Page::Text => "/components/text".to_string(),
            Page::Icon => "/components/icon".to_string(),
            Page::Divider => "/components/divider".to_string(),
            Page::Button => "/components/button".to_string(),
            Page::Shapes => "/components/shapes".to_string(),

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
            Page::PeakDesktop => "/core/peak-desktop".to_string(),
            Page::PeakOSCore => "/core/peak-os-core".to_string(),

            // Settings
            Page::Appearance => "/settings/appearance".to_string(),
            Page::Scaling => "/settings/scaling".to_string(),
            Page::Shortcuts => "/settings/shortcuts".to_string(),
            Page::About => "/settings/about".to_string(),
            Page::Updates => "/settings/updates".to_string(),

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
            "/" | "" => Page::Introduction,

            "/guide/roadmap" => Page::Roadmap,
            "/guide/community" => Page::Community,

            "/docs/overview" => Page::Overview,
            "/docs/architecture" => Page::Architecture,
            "/docs/project-structure" => Page::ProjectStructure,
            "/docs/customizations" => Page::Customizations,
            "/docs/basic-sizing" => Page::BasicSizing,
            "/docs/typography" => Page::Typography,
            "/docs/layout" => Page::Layout,

            "/components/text" => Page::Text,
            "/components/icon" => Page::Icon,
            "/components/divider" => Page::Divider,
            "/components/button" => Page::Button,
            "/components/shapes" => Page::Shapes,

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
            "/core/peak-desktop" => Page::PeakDesktop,
            "/core/peak-os-core" => Page::PeakOSCore,

            "/settings/appearance" => Page::Appearance,
            "/settings/scaling" => Page::Scaling,
            "/settings/shortcuts" => Page::Shortcuts,
            "/settings/about" => Page::About,
            "/settings/updates" => Page::Updates,

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
            Page::Introduction
            | Page::Overview
            | Page::Architecture
            | Page::ProjectStructure
            | Page::Roadmap
            | Page::Community
            | Page::Typography
            | Page::Customizations
            | Page::BasicSizing
            | Page::Layout => "Start".to_string(),

            Page::Text
            | Page::Icon
            | Page::Divider
            | Page::Button
            | Page::Shapes
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
            | Page::PeakDesktop
            | Page::PeakOSCore => "Data".to_string(),

            Page::Appearance | Page::Scaling | Page::Shortcuts | Page::About | Page::Updates => {
                "Settings".to_string()
            }

            Page::Unknown(_) => "Start".to_string(),
        }
    }
}
