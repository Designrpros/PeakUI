pub mod dev;
pub mod elements;
pub mod engine;
pub mod layout;
pub mod shell;

pub mod backend;
#[macro_use]
pub mod macros;
pub mod core;
pub mod reference;
pub mod semantic;
pub mod style;
pub mod views;

#[macro_export]
macro_rules! data_table {
    ($($method:ident($($arg:expr),* $(,)?)),* $(,)?) => {
        $crate::views::data_table::DataTable::new()
            $(.$method($($arg),*))*
    };
}

pub mod prelude {
    pub use crate::backend::{
        ai::AIBackend, iced_backend::IcedBackend, spatial::SpatialBackend, term::TermBackend,
        Backend, TextSpan,
    };
    pub use crate::core::{responsive, ProxyView, ShellMode, ThemeTokens, View};
    pub use crate::dev::catalog::{Catalog, CatalogItem, CatalogMessage};
    pub use crate::dev::console::Console;
    pub use crate::dev::dsl::*;
    pub use crate::elements::atoms::{
        badge::Badge, Capsule, Circle, Container, Divider, Icon, Image, Rectangle, Space, Text,
        Video, WebView,
    };
    pub use crate::elements::controls::{Button, ButtonStyle, Slider, Stepper, TextInput, Toggle};
    pub use crate::elements::forms::{Form, FormStyle};
    pub use crate::elements::segmented_picker::SegmentedPicker;
    pub use crate::engine::gestures::{Gesture, GestureDetector, TapGesture};
    pub use crate::engine::localization::Localization;
    pub use crate::engine::modifiers::ControlSize;
    pub use crate::engine::motion::{AnimationConfig, Interpolatable, MotionState, Spring};
    pub use crate::engine::navigation::{
        DetailView, NavigationLink, NavigationListView, Page, PageResult, SearchConfig, Sidebar,
        ViewExt,
    };
    pub use crate::layout::containers::{Card, GlassCard, Section};
    pub use crate::layout::nav_split_view::NavigationSplitView;
    pub use crate::layout::scroll_view::ScrollView;
    pub use crate::layout::{HStack, LayoutExt, ResponsiveGrid, VStack, ZStack};
    #[cfg(feature = "intelligence")]
    pub use crate::semantic::IntelligenceProvider;
    pub use crate::semantic::{ChatCompletionMessage, DataProvider, SemanticNode, SemanticRecord};
    pub use crate::shell::toolbar::{ToolbarGroup, ToolbarItem};
    pub use crate::style::{Context, DeviceType, Intent, ScrollDirection, Variant};
    pub use crate::views::chart::{Chart, ChartDataPoint, ChartType};
    pub use crate::views::code_block::CodeBlock;
    pub use crate::views::data_table::{DataTable, DataTablePreset};
    pub use crate::views::markdown::MarkdownView;
    pub use crate::{hstack, vstack, zstack};
    pub use peak_theme::ThemeTone;

    // Re-export core UI types so showcase doesn't need direct iced imports
    pub use iced::widget::{column, container, row, scrollable, stack, Id};
    pub use iced::{
        application, clipboard, event, futures, keyboard, mouse, run, window, Alignment,
        Background, Border, Color, Element, Event, Font, Length, Padding, Point, Renderer, Result,
        Shadow, Size, Subscription, Task, Theme, Vector,
    };
    pub mod font {
        pub use iced::font::*;
    }
}
