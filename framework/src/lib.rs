pub mod accessibility;
pub mod alert;
pub mod localization;
pub mod navigation;
pub mod reference;
pub mod segmented_picker;
pub mod toolbar;
pub mod window_chrome;

pub mod dsl;
#[macro_export]
macro_rules! vstack {
    ($($child:expr),* $(,)?) => {
        $crate::layout::VStack::new()
            $(.push($child))*
    };
}

#[macro_export]
macro_rules! hstack {
    ($($child:expr),* $(,)?) => {
        $crate::layout::HStack::new()
            $(.push($child))*
    };
}

#[macro_export]
macro_rules! zstack {
    ($($child:expr),* $(,)?) => {
        $crate::layout::ZStack::new()
            $(.push($child))*
    };
}

#[macro_export]
macro_rules! data_table {
    ($($method:ident($($arg:expr),* $(,)?)),* $(,)?) => {
        $crate::views::data_table::DataTable::new()
            $(.$method($($arg),*))*
    };
}

pub mod assets;
pub mod atoms;
pub mod backend;
pub mod benchmark;
pub mod catalog;
pub mod console;
pub mod containers;
pub mod controls;
pub mod core;
pub mod forms;
pub mod gestures;
pub mod layout;
pub mod modifiers;
pub mod motion;
pub mod nav_split_view;
pub mod scroll_view;
pub mod semantic;
pub mod style;
pub mod views;

pub mod prelude {
    pub use crate::atoms::{
        badge::Badge, Capsule, Circle, Container, Divider, Icon, Image, Rectangle, Space, Text,
        Video, WebView,
    };
    pub use crate::backend::{
        ai::AIBackend, iced_backend::IcedBackend, spatial::SpatialBackend, term::TermBackend,
        Backend, TextSpan,
    };
    pub use crate::catalog::{Catalog, CatalogItem, CatalogMessage};
    pub use crate::console::Console;
    pub use crate::containers::{Card, GlassCard, Section};
    pub use crate::controls::{Button, ButtonStyle, Slider, Stepper, TextInput, Toggle};
    pub use crate::core::{responsive, ProxyView, ShellMode, ThemeTokens, View};
    pub use crate::dsl::*;
    pub use crate::forms::{Form, FormStyle};
    pub use crate::gestures::{Gesture, GestureDetector, TapGesture};
    pub use crate::layout::{HStack, LayoutExt, ResponsiveGrid, VStack, ZStack};
    pub use crate::localization::Localization;
    pub use crate::modifiers::ControlSize;
    pub use crate::motion::{AnimationConfig, Interpolatable, MotionState, Spring};
    pub use crate::nav_split_view::NavigationSplitView;
    pub use crate::navigation::{
        DetailView, NavigationLink, NavigationListView, Page, PageResult, SearchConfig, Sidebar,
        ViewExt,
    };
    pub use crate::scroll_view::ScrollView;
    pub use crate::segmented_picker::SegmentedPicker;
    pub use crate::semantic::{
        ChatCompletionMessage, DataProvider, IntelligenceProvider, SemanticNode, SemanticRecord,
    };
    pub use crate::style::{Context, DeviceType, Intent, ScrollDirection, Variant};
    pub use crate::toolbar::{ToolbarGroup, ToolbarItem};
    pub use crate::views::chart::{Chart, ChartDataPoint, ChartType};
    pub use crate::views::code_block::CodeBlock;
    pub use crate::views::data_table::{DataTable, DataTablePreset};
    pub use crate::views::markdown::MarkdownView;
    pub use crate::{hstack, vstack, zstack};
    pub use peak_theme::ThemeTone;

    // Re-export core UI types so showcase doesn't need direct iced imports
    pub use iced::widget::{column, container, row, scrollable, stack, Id};
    pub use iced::{
        application, run, window, Alignment, Background, Border, Color, Element, Font, Length,
        Padding, Renderer, Result, Shadow, Size, Task, Theme, Vector,
    };
    pub mod font {
        pub use iced::font::*;
    }
}
