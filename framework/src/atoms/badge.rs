use crate::prelude::*;

pub struct Badge {
    pub label: String,
    pub intent: Intent,
    pub variant: Variant,
}

impl Badge {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            intent: Intent::Neutral,
            variant: Variant::Soft,
        }
    }

    pub fn intent(mut self, intent: Intent) -> Self {
        self.intent = intent;
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }
}

// Support for generic messages
impl<M: 'static + Clone> View<M, IcedBackend> for Badge {
    fn view(&self, context: &Context) -> Element<'static, M, Theme, Renderer> {
        let theme = context.theme;
        let palette = theme.colors;

        let color = match self.intent {
            Intent::Primary => palette.primary,
            Intent::Success => palette.success,
            Intent::Warning => palette.warning,
            Intent::Danger => palette.danger,
            Intent::Info => palette.info,
            Intent::Neutral | Intent::Secondary | Intent::Accent => palette.text_secondary,
        };

        container(View::<M, IcedBackend>::view(
            &Text::<IcedBackend>::new(self.label.clone())
                .caption2()
                .bold()
                .color(color),
            context,
        ))
        .padding([4, 10])
        .style(move |_| container::Style {
            background: Some(color.scale_alpha(0.1).into()),
            border: Border {
                radius: 12.0.into(),
                width: 1.0,
                color: color.scale_alpha(0.2),
            },
            ..Default::default()
        })
        .into()
    }

    fn describe(&self, _context: &Context) -> SemanticNode {
        SemanticNode::new("badge").with_label(self.label.clone())
    }
}
