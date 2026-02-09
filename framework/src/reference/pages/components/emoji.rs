use crate::core::{AIBackend, Backend, IcedBackend, SpatialBackend, TermBackend};
use crate::prelude::*;
use crate::reference::app::{EmojiLabState, Message, RenderMode};
use crate::reference::views::ComponentDoc;
use crate::reference::AppPageResult;
use std::sync::Arc;

pub fn view(
    ctx: &Context,
    lab: &EmojiLabState,
    render_mode: RenderMode,
    search_query: String,
) -> AppPageResult {
    // --- 1. Emoji Library ---
    let emojis = [
        (
            "Smileys",
            vec![
                "😀", "😃", "😄", "😁", "😆", "😅", "😂", "🤣", "😊", "😇", "🙂", "🙃", "😉", "😌",
                "😍", "🥰", "😘", "😗", "😙", "😚", "😋", "😛", "😝", "😜", "🤪", "🤨", "🧐", "🤓",
                "😎", "🤩", "🥳", "😏", "😒", "😞", "😔", "😟", "😕", "🙁", "☹️", "😣", "😖", "😫",
                "😩", "🥺", "😢", "😭", "😤", "😠", "😡", "🤬", "🤯", "😳", "🥵", "🥶", "😱", "😨",
                "😰", "😥", "😓", "🤗", "🤔", "🤭", "🤫", "🤥", "😶", "😐", "😑", "😬", "🙄", "😯",
                "😦", "😧", "😮", "😲", "🥱", "😴", "🤤", "😪", "😵", "🤐", "🥴", "🤢", "🤮", "🤧",
                "🥵", "🥶", "😷", "🤒", "🤕",
            ],
        ),
        (
            "Animals",
            vec![
                "🐶",
                "🐱",
                "🐭",
                "🐹",
                "🐰",
                "🦊",
                "🐻",
                "🐼",
                "🐻‍❄️",
                "🐨",
                "🐯",
                "🦁",
                "🐮",
                "🐷",
                "🐽",
                "🐸",
                "🐵",
                "🐒",
                "🦍",
                "🦧",
                "🐶",
                "🐕",
                "🦮",
                "🐕‍🦺",
                "🐩",
                "🐺",
                "🦊",
                "🦝",
                "🐱",
                "🐈",
                "🐈‍⬛",
                "🦁",
                "🐯",
                "🐅",
                "🐆",
                "🐴",
                "🐎",
                "🦄",
                "🦓",
                "🦌",
                "🦬",
                "🐮",
                "牛",
                "🐃",
                "🐄",
                "🐷",
                "🐖",
                "🐗",
                "🐽",
                "🐏",
                "🐑",
                "🐐",
                "🐪",
                "🐫",
                "🦙",
                "🦒",
                "🐘",
                "🦣",
                "🦏",
                "🦛",
                "🐭",
                "🐁",
                "🐀",
                "🐹",
                "🐰",
                "🐇",
                "🐿️",
                "🦫",
                "🦔",
                "🦇",
                "🐻",
                "🐻‍❄️",
                "🐨",
                "🐼",
                "🦥",
                "🦦",
                "🦨",
                "🦘",
                "🦡",
                "🐾",
            ],
        ),
        (
            "Food",
            vec![
                "🍏", "🍎", "🍐", "🍊", "🍋", "🍌", "🍉", "🍇", "🍓", "🫐", "🍈", "🍒", "🍑", "🥭",
                "🍍", "🥥", "🥝", "🍅", "🍆", "🥑", "🥦", "🥬", "🥒", "🌶️", "🫑", "🌽", "🥕", "🫒",
                "🧄", "🧅", "🍄", "🥜", "🫘", "🌰", "🍞", "🥐", "🥖", "🫓", "🥨", "🥯", "🥞", "🧇",
                "🧀", "🍖", "🍗", "🥩", "🥓", "🍔", "🍟", "🍕", "🌭", "🥪", "🌮", "🌯", "🫔", "🥙",
                "🧆", "🥚", "🍳", "🥘", "🍲", "🥣", "🥗", "🍿", "🧈", "🧂", "🥫", "🍱", "🍘", "🍙",
                "🍚", "🍛", "🍜", "🍝", "🍠", "🍢", "🍣", "🍤", "🍥", "🥮", "🍡", "🥟", "🥠", "🥡",
                "🦀", "🦞", "🦐", "🦑", "🦪", "🍦", "🍧", "🍨", "🍩", "🍪", "🎂", "🍰", "🧁", "🥧",
                "🍫", "🍬", "🍭", "🍮", "🍯", "🍼", "🥛", "☕", "🫖", "🍵", "🍶", "🍾", "🍷", "🍸",
                "🍹", "🍺", "🍻", "🥂", "🥃", "🥤", "🧋", "🧃", "🧉", "🧊", "🥢", "🍽️", "🍴", "🥄",
            ],
        ),
        (
            "Activities",
            vec![
                "⚽", "🏀", "🏈", "⚾", "🥎", "🎾", "🏐", "🏉", "🎱", "🥏", "🏓", "🏸", "🏒", "🏑",
                "🥍", "🏏", "🪃", "🥅", "⛳", "🪁", "🏹", "🎣", "🤿", "🥊", "🥋", "🎽", "🛹", "🛼",
                "🛷", "⛸️", "🎿", "🏂", "🪂", "🏋️", "🤼", "🤸", "⛹️", "🤺", "🤾", "🏌️", "🏇", "🧘",
                "🏄", "🏊", "🤽", "🚣", "🧗", "🚵", "🚴", "🏆", "🥇", "🥈", "🥉", "🏅", "🎖️", "🏵️",
                "🎫", "🎟️", "🎭", "🎨", "🖼️", "🧵", "🪡", "🧶", "🎹", "🥁", "🪘", "🎷", "🎺", "🎸",
                "🪕", "🎻", "🎲", "♟️", "🎯", "🎳", "🎮", "🎰", "🧩",
            ],
        ),
        (
            "Travel",
            vec![
                "🚗", "🚕", "🚙", "🚌", "🚎", "🏎️", "🚓", "🚑", "🚒", "🚐", "🛻", "🚚", "🚛", "🚜",
                "🛵", "🏍️", "🛺", "🚲", "🛴", "🛹", "🚏", "🛣️", "🛤️", "🛢️", "⛽", "🚨", "🚥", "🚦",
                "🛑", "🚧", "⚓", "⛵", "🛶", "🚤", "🛳️", "⛴️", "🚢", "✈️", "🛩️", "🛫", "🛬", "🪂",
                "💺", "🚁", "🚟", "🚠", "🚡", "🛰️", "🚀", "🛸", "🪐", "🌠", "🌌", "☀️", "🌤️", "⛅",
                "🌥️", "☁️", "🌦️", "🌧️", "🌨️", "🌩️", "🌪️", "🌫️", "🌬️", "🌀", "🌈", "🌂", "☂️", "☔",
                "⛱️", "⚡", "❄️", "☃️", "⛄", "☄️", "🔥", "💧", "🌊",
            ],
        ),
        (
            "Objects",
            vec![
                "⌚", "📱", "📲", "💻", "⌨️", "🖱️", "🖨️", "🖥️", "🖲️", "🕹️", "🗜️", "💽", "💾", "💿",
                "DVD", "📼", "📷", "📸", "📹", "🎥", "📽️", "🎞️", "📞", "☎️", "📟", "📠", "📺",
                "📻", "🎙️", "🎚️", "🎛️", "🧭", "⏱️", "⏲️", "⏰", "🕰️", "⌛", "⏳", "📡", "🔋", "🔌",
                "💡", "🔦", "🕯️", "🪔", "🧯", "🛢️", "💸", "💵", "💴", "💶", "💷", "🪙", "💰", "💳",
                "💎", "⚖️", "🪜", "🧰", "🪛", "🔧", "🔨", "⚒️", "🛠️", "⛏️", "🪚", "🔩", "⚙️", "🪤",
                "🧱", "⛓️", "🧲", "🔫", "💣", "🧨", "🪓", "🔪", "🗡️", "⚔️", "🛡️", "🚬", "⚰️", "🪦",
                "⚱️", "🏺", "🔮", "📿", "🧿", "💈", "⚗️", "🔭", "🔬", "🕳️", "🩹", "🩺", "💊", "💉",
                "🩸", "🧬", "🦠", "🧫", "🧪", "🌡️", "🧹", "🧺", "🧻", "🧼", "🧽", "🪣", "🧴", "🔑",
                "🗝️", "🔐", "🔏", "🔓", "🔒", "🚪", "🪑", "🛋️", "🛏️", "🧸", "🪆", "🖼️", "🪞", "🪟",
                "🛍️", "🛒", "🎁", "🎈", "🎏", "🎀", "🪄", "🎊", "🎉", "🎊", "🏮", "🧧", "✉️", "📩",
                "📨", "📧", "💌", "📥", "📤", "📦", "🏷️", "🪪", "📪", "📫", "📬", "📭", "📮", "📯",
                "📜", "📃", "📄", "📑", "🧾", "📊", "📈", "📉", "🗒️", "🗓️", "📅", "🗑️", "📇", "🗃️",
                "🗳️", "🗄️", "📋", "📁", "📂", "🗂️", "🗞️", "📰", "📓", "📔", "📒", "📕", "📗", "📘",
                "📙", "📚", "📖", "🔖", "🧷", "🔗", "📎", "🖇️", "📐", "📏", "🧮", "📌", "📍", "✂️",
                "🖊️", "🖋️", "✒️", "🖌️", "🖍️", "📝", "✏️", "🔍", "🔎", "🔏", "🔐", "🔒", "🔓",
            ],
        ),
    ];

    let query = search_query.to_lowercase();
    let mut filtered_sections = Vec::new();
    let mut total_count = 0;

    for (name, items) in emojis.iter() {
        let filtered: Vec<_> = items
            .iter()
            .filter(|&e| {
                // In a real app we might want a mapping of emoji to names,
                // but for now we search the emoji character itself or the category name.
                query.is_empty() || name.to_lowercase().contains(&query) || e.contains(&query)
            })
            .cloned()
            .collect();

        if !filtered.is_empty() {
            total_count += filtered.len();
            filtered_sections.push((*name, filtered));
        }
    }

    // --- 2. Preview Construction ---
    let preview_view = create_preview::<IcedBackend>(lab);
    let terminal_preview = create_preview::<TermBackend>(lab).view(ctx);
    let neural_preview = create_preview::<AIBackend>(lab).view(ctx);
    let spatial_preview = create_preview::<SpatialBackend>(lab).view(ctx);

    // --- 3. Code Snippet ---
    let code_snippet = generate_code(lab);

    let library_section = {
        let mut sections_stack = vstack::<Message, IcedBackend>().spacing(40.0);

        for (name, items) in filtered_sections {
            let mut grid = ResponsiveGrid::<Message, IcedBackend>::new()
                .columns(8)
                .mobile_columns(4)
                .spacing(12.0);

            for emoji_char in items {
                let emoji_str = emoji_char.to_string();
                let is_selected = lab.selected_emoji == emoji_str;

                let button_content = vstack::<Message, IcedBackend>()
                    .push(text::<IcedBackend>(emoji_str.clone()).size(32.0))
                    .align_x(iced::Alignment::Center)
                    .padding(8)
                    .width(Length::Fill)
                    .height(Length::Fixed(60.0));

                grid = grid.push(
                    button::<Message, IcedBackend>(button_content)
                        .variant(if is_selected {
                            Variant::Solid
                        } else {
                            Variant::Ghost
                        })
                        .on_press(Message::UpdateEmojiLabEmoji(emoji_str))
                        .width(Length::Fill)
                        .height(Length::Fixed(60.0)),
                );
            }

            sections_stack = sections_stack.push(
                vstack::<Message, IcedBackend>()
                    .push(text::<IcedBackend>(name).title3().bold())
                    .push(grid)
                    .spacing(16.0),
            );
        }

        let section_content = vstack::<Message, IcedBackend>()
            .push(
                text::<IcedBackend>(format!("Emoji Library ({} items found)", total_count))
                    .title2()
                    .bold(),
            )
            .push(if total_count == 0 {
                text::<IcedBackend>("No emojis found matching your search.")
                    .secondary()
                    .into_box()
            } else {
                sections_stack.into_box()
            })
            .spacing(24.0);

        crate::layout::containers::Section::new("Library", section_content).width(Length::Fill)
    };

    // --- 5. Component Documentation ---
    let doc = ComponentDoc::new(
        "Emoji",
        "A rich Unicode emoji component rendered using the bundled Noto Color Emoji font. Perfect for expressive UI elements and contextual indicators.",
        code_snippet,
        Arc::new(preview_view),
    )
    .terminal(terminal_preview)
    .neural(neural_preview)
    .spatial(spatial_preview)
    .render_mode(render_mode)
    .on_render_mode_change(|mode| Message::SetRenderMode(mode))
    .theory(
       "### Emotional Intelligence\nEmojis in **PeakUI** are first-class residents of the typography system.\n\n- **Color Assets**: Uses Noto Color Emoji for consistent, cross-platform appearance in the showcase.\n- **WASM Optimized**: Bundled directly to ensure they render even in sandboxed environments.\n- **Semantic Value**: AI agents treat emojis as sentiment tokens, helping them understand the 'vibe' of the interface."
    )
    .props_table(
        "| Modifier | Description |\n| :--- | :--- |\n| `.new(char)` | Initialize with a Unicode emoji string or character. |\n| `.size(f32)` | Sets the emoji size (default is 48.0). |\n| `.view(ctx)` | Renders the emoji into the current context. |"
    )
    .extra_content(library_section);

    AppPageResult::new(doc)
        .searchable("Emoji", "Search emojis...", |s| Message::Search(s))
        .inspector(EmojiInspector::new(lab))
}

fn create_preview<B: Backend>(lab: &EmojiLabState) -> VStack<Message, B> {
    VStack::new_generic()
        .spacing(24.0)
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .push(
            vstack::<Message, B>()
                .push(text::<B>("Preview").caption2().secondary())
                .push(text::<B>(lab.selected_emoji.clone()).size(lab.size))
                .spacing(16.0)
                .align_x(Alignment::Center),
        )
}

fn generate_code(lab: &EmojiLabState) -> String {
    format!(
        "Text::new(\"{}\")\n    .size({:.1})",
        lab.selected_emoji, lab.size
    )
}

struct EmojiInspector {
    lab: EmojiLabState,
}

impl EmojiInspector {
    fn new(lab: &EmojiLabState) -> Self {
        Self { lab: lab.clone() }
    }
}

impl View<Message, IcedBackend> for EmojiInspector {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
        let content = vstack![
            vstack![
                Text::<IcedBackend>::new("Selected Emoji")
                    .caption2()
                    .bold()
                    .secondary(),
                TextInput::<Message, IcedBackend>::new(
                    self.lab.selected_emoji.clone(),
                    "Emoji...",
                    |s| Message::UpdateEmojiLabEmoji(s),
                ),
            ]
            .spacing(8.0),
            vstack![
                Text::<IcedBackend>::new("Size")
                    .caption2()
                    .bold()
                    .secondary(),
                hstack![
                    Slider::<Message, IcedBackend>::new(12.0..=128.0, self.lab.size, |v| {
                        Message::UpdateEmojiLabSize(v)
                    },)
                    .width(Length::Fill),
                    Text::<IcedBackend>::new(format!("{:.0}", self.lab.size))
                        .caption2()
                        .secondary(),
                ]
                .spacing(12.0),
            ]
            .spacing(8.0),
        ]
        .spacing(24.0)
        .padding(Padding::from([20, 20]))
        .width(Length::Fill);

        IcedBackend::scroll_view(
            content.view(context),
            Length::Fill,
            Length::Fill,
            None,
            false,
            ScrollDirection::Vertical,
            context,
        )
    }
}
