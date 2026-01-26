use crate::atoms::{Icon, Text};
use crate::core::{Backend, Context, IcedBackend, TermBackend, View};
use crate::prelude::*;
use iced::{Element, Renderer, Task, Theme};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Design,
    System,
    Layouts,
    Collections,
    Navigation,
    Content,
    Controls,
    View,
}

impl Category {
    fn title(&self) -> &'static str {
        match self {
            Self::Design => "Design",
            Self::System => "System",
            Self::Layouts => "Layouts",
            Self::Collections => "Collections",
            Self::Navigation => "Navigation",
            Self::Content => "Content",
            Self::Controls => "Controls",
            Self::View => "View",
        }
    }
}

#[derive(Clone)]
pub struct CatalogItem<B: Backend = IcedBackend> {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub category: Category,
    pub render: fn(&Context) -> Box<dyn View<CatalogMessage, B>>,
}

#[derive(Debug, Clone)]
pub enum CatalogMessage {
    ItemSelected(&'static str),
    GoBack,
    ToggleInspector,
    // Control States
    ToggleChanged(bool),
    SliderChanged(f32),
    StepperChanged(i32),
    PickerChanged(usize),
    TextChanged(String),
    ToggleAlert(bool),
    ThemeChanged(peak_theme::PeakTheme),
    ToneChanged(peak_theme::ThemeTone),
    None,
}

#[derive(Clone)]
pub struct Catalog<B: Backend = IcedBackend> {
    pub selected_id: Option<&'static str>,
    pub items: Vec<CatalogItem<B>>,
    // Theme State
    pub theme: peak_theme::PeakTheme,
    pub tone: peak_theme::ThemeTone,
}

impl<B: Backend> Catalog<B> {
    pub fn new() -> Self {
        Self {
            selected_id: Some("typography"),
            items: Self::build_items(),
            theme: peak_theme::PeakTheme::Cupertino,
            tone: peak_theme::ThemeTone::Light,
        }
    }

    pub fn update(&mut self, message: CatalogMessage) -> Task<CatalogMessage> {
        match message {
            CatalogMessage::ItemSelected(id) => {
                self.selected_id = Some(id);
                Task::none()
            }
            CatalogMessage::GoBack => {
                self.selected_id = None;
                Task::none()
            }
            _ => Task::none(),
        }
    }

    fn build_items() -> Vec<CatalogItem<B>> {
        vec![
            CatalogItem {
                id: "typography",
                title: "Typography",
                description: "Font hierarchy and styles.",
                category: Category::Content,
                render: render_typography,
            },
            CatalogItem {
                id: "colors",
                title: "Colors",
                description: "System colors and tokens.",
                category: Category::Design,
                render: render_colors,
            },
            CatalogItem {
                id: "icons",
                title: "Icons",
                description: "System icons and symbols.",
                category: Category::Content,
                render: render_icons,
            },
        ]
    }
}

impl View<CatalogMessage, IcedBackend> for Catalog<IcedBackend> {
    fn view(&self, context: &Context) -> Element<'static, CatalogMessage, Theme, Renderer> {
        let items = &self.items;
        let selected_id = self.selected_id;

        let sidebar_content = VStack::<CatalogMessage, IcedBackend>::new()
            .padding(12.0)
            .spacing(12.0)
            .push(Text::<IcedBackend>::new("PeakUI").headline())
            .push(render_category::<IcedBackend>(
                items,
                selected_id,
                Category::Design,
            ))
            .push(render_category::<IcedBackend>(
                items,
                selected_id,
                Category::Content,
            ));

        let detail_view: Box<dyn View<CatalogMessage, IcedBackend>> = if let Some(sid) = selected_id
        {
            if let Some(item) = items.iter().find(|i| i.id == sid) {
                (item.render)(context)
            } else {
                Box::new(Text::<IcedBackend>::new("Item not found").large_title())
            }
        } else {
            Box::new(Text::<IcedBackend>::new("Select an item").large_title())
        };

        NavigationSplitView::new(sidebar_content, detail_view)
            .on_back(CatalogMessage::GoBack)
            .on_none(CatalogMessage::None)
            .view(context)
    }
}

impl View<CatalogMessage, TermBackend> for Catalog<TermBackend> {
    fn view(&self, context: &Context) -> String {
        let items = &self.items;
        let selected_id = self.selected_id;

        let mut out = String::new();
        out.push_str("\x1b[1;37;44m  PEAK OS CATALOG  \x1b[0m\n");
        out.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n");

        let sidebar = VStack::<CatalogMessage, TermBackend>::new_tui()
            .push(render_category::<TermBackend>(
                items,
                selected_id,
                Category::Design,
            ))
            .push(render_category::<TermBackend>(
                items,
                selected_id,
                Category::Content,
            ));

        let detail = if let Some(sid) = selected_id {
            if let Some(item) = items.iter().find(|i| i.id == sid) {
                format!(
                    "\x1b[1m{}\x1b[0m\n{}\n────────────────────\n{}",
                    item.title,
                    item.description,
                    (item.render)(context).view(context)
                )
            } else {
                "\x1b[31;1mError: Item not found\x1b[0m".to_string()
            }
        } else {
            "\x1b[2mSelect an item to view\x1b[0m".to_string()
        };

        out.push_str(&sidebar.view(context));
        out.push_str("\n\n");
        out.push_str(&detail);
        out
    }
}
fn render_category<B: Backend>(
    items: &[CatalogItem<B>],
    selected_id: Option<&'static str>,
    category: Category,
) -> impl View<CatalogMessage, B> {
    let mut list = VStack::<CatalogMessage, B>::new_generic().spacing(2.0);

    for item in items {
        if item.category != category {
            continue;
        }

        let id = item.id;
        let title = item.title.to_string();
        let is_selected = Some(id) == selected_id;

        let icon = match id {
            "typography" => "text_fields",
            "colors" => "palette",
            "icons" => "stars",
            _ => "circle",
        }
        .to_string();

        list = list.push(ProxyView::<CatalogMessage, B>::new(move |context| {
            B::button(
                B::sidebar_item(title.clone(), icon.clone(), is_selected, context),
                Some(CatalogMessage::ItemSelected(id)),
                Variant::Ghost,
                Intent::Neutral,
                context,
            )
        }));
    }

    VStack::<CatalogMessage, B>::new_generic()
        .spacing(8.0)
        .padding(iced::Padding {
            top: 16.0,
            right: 0.0,
            bottom: 8.0,
            left: 0.0,
        })
        .push(
            Text::<B>::new(category.title().to_uppercase())
                .caption2()
                .secondary()
                .bold(),
        )
        .push(list)
}

fn render_typography<B: Backend>(_ctx: &Context) -> Box<dyn View<CatalogMessage, B>> {
    Box::new(
        VStack::<CatalogMessage, B>::new_generic()
            .spacing(12.0)
            .push(Text::<B>::new("Title 1").title1())
            .push(Text::<B>::new("Headline").headline())
            .push(Text::<B>::new("Body").body()),
    )
}

fn render_colors<B: Backend>(ctx: &Context) -> Box<dyn View<CatalogMessage, B>> {
    let colors = ctx.theme.colors;
    Box::new(
        VStack::<CatalogMessage, B>::new_generic()
            .spacing(12.0)
            .push(Text::<B>::new("Colors").title2())
            .push(Text::<B>::new(format!("Primary: {:?}", colors.primary)).body()),
    )
}

fn render_icons<B: Backend>(_ctx: &Context) -> Box<dyn View<CatalogMessage, B>> {
    Box::new(
        HStack::<CatalogMessage, B>::new_generic()
            .spacing(16.0)
            .push(Icon::<B>::new("settings").size(32.0))
            .push(Icon::<B>::new("terminal").size(32.0)),
    )
}
