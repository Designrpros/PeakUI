use crate::prelude::*;
use std::borrow::Cow;

#[derive(Debug, Clone, serde::Serialize, PartialEq)]
pub enum ChartType {
    Line,
    Bar,
    Area,
    Pie,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ChartDataPoint {
    pub label: String,
    pub value: f32,
}

pub struct Chart<Message: 'static, B: Backend = IcedBackend> {
    pub chart_type: ChartType,
    pub data: Vec<ChartDataPoint>,
    pub title: Option<String>,
    pub color: Color,
    pub palette: Vec<Color>,
    pub width: Length,
    pub height: Length,
    _phantom: std::marker::PhantomData<(Message, B)>,
}

impl<Message: 'static, B: Backend> Chart<Message, B> {
    pub fn new(chart_type: ChartType, data: Vec<ChartDataPoint>) -> Self {
        Self {
            chart_type,
            data,
            title: None,
            color: Color::from_rgb(0.31, 0.98, 0.48), // Default Peak accent
            palette: Vec::new(),
            width: Length::Fill,
            height: Length::Fixed(300.0),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn palette(mut self, palette: Vec<Color>) -> Self {
        self.palette = palette;
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }
}

impl<Message: Clone + 'static, B: Backend> View<Message, B> for Chart<Message, B> {
    fn view(&self, context: &Context) -> B::AnyView<Message> {
        let mut children = Vec::new();

        if let Some(title) = &self.title {
            children.push(B::text(
                title.clone(),
                14.0,
                None,
                true,
                false,
                None,
                None,
                Length::Shrink,
                Alignment::Start,
                context,
            ));
        }

        let chart_content = match self.chart_type {
            ChartType::Bar => self.render_bar(context),
            ChartType::Line => self.render_line(context),
            ChartType::Pie => self.render_pie(context),
            _ => B::rectangle(
                Length::Fill,
                Length::Fill,
                None,
                8.0,
                1.0,
                Some(context.theme.colors.border),
                context,
            ),
        };

        children.push(chart_content);

        B::vstack(
            children,
            16.0,
            Padding::ZERO,
            self.width,
            self.height,
            Alignment::Start,
            Alignment::Start,
            context,
        )
    }

    fn describe(&self, _context: &Context) -> SemanticNode {
        let mut node = SemanticNode::new("chart");
        node.label = self.title.as_ref().map(|t| Cow::Owned(t.to_string()));

        // Export data for AI perception
        let data_json = serde_json::to_string(&self.data).unwrap_or_default();
        node.content = Some(Cow::Owned(format!(
            "Type: {:?}, Data: {}",
            self.chart_type, data_json
        )));

        node
    }

    fn describe_iced(&self, context: &Context) -> SemanticNode {
        self.describe(context)
    }
}

impl<Message: Clone + 'static, B: Backend> Chart<Message, B> {
    fn render_bar(&self, context: &Context) -> B::AnyView<Message> {
        let max_value = self
            .data
            .iter()
            .map(|d| d.value)
            .fold(0.0f32, |a, b| a.max(b));

        let chart_height = 140.0; // Slightly reduced to give label area more room
        let mut bars = Vec::new();

        for (i, point) in self.data.iter().enumerate() {
            let ratio = if max_value > 0.0 {
                point.value / max_value
            } else {
                0.0
            };

            let bar_h = (ratio * chart_height).max(1.0);
            let spacer_h = chart_height - bar_h;

            let color = if self.palette.is_empty() {
                self.color
            } else {
                self.palette[i % self.palette.len()]
            };

            let bar_item = B::vstack(
                vec![
                    // Bar Area
                    B::vstack(
                        vec![
                            B::rectangle(
                                Length::Fixed(1.0),
                                Length::Fixed(spacer_h),
                                None,
                                0.0,
                                0.0,
                                None,
                                context,
                            ),
                            B::rectangle(
                                Length::Fixed(40.0),
                                Length::Fixed(bar_h),
                                Some(color),
                                4.0,
                                0.0,
                                None,
                                context,
                            ),
                        ],
                        0.0,
                        Padding::ZERO,
                        Length::Fixed(40.0),
                        Length::Fixed(chart_height),
                        Alignment::Center,
                        Alignment::End,
                        context,
                    ),
                    // Label Area
                    B::vstack(
                        vec![
                            B::text(
                                point.label.clone(),
                                11.0,
                                None,
                                true,
                                false,
                                None,
                                None,
                                Length::Shrink,
                                Alignment::Center,
                                context,
                            ),
                            B::text(
                                format!("{:.0}", point.value),
                                10.0,
                                None,
                                false,
                                true,
                                None,
                                None,
                                Length::Shrink,
                                Alignment::Center,
                                context,
                            ),
                        ],
                        2.0,
                        Padding::ZERO,
                        Length::Shrink,
                        Length::Shrink,
                        Alignment::Center,
                        Alignment::Start,
                        context,
                    ),
                ],
                8.0,
                Padding::ZERO,
                Length::Fixed(60.0), // Give more width to labels
                Length::Shrink,
                Alignment::Center,
                Alignment::Start,
                context,
            );
            bars.push(bar_item);
        }

        B::hstack(
            bars,
            16.0,
            Padding::ZERO,
            Length::Fill,
            Length::Fill,
            Alignment::Center,
            Alignment::End,
            context,
        )
    }

    fn render_line(&self, context: &Context) -> B::AnyView<Message> {
        let max_value = self
            .data
            .iter()
            .map(|d| d.value)
            .fold(0.0f32, |a, b| a.max(b));

        let chart_height = 140.0;
        let step_width = 80.0;
        let mut markers = Vec::new();
        let mut path_points = Vec::new();

        for (i, point) in self.data.iter().enumerate() {
            let ratio = if max_value > 0.0 {
                point.value / max_value
            } else {
                0.0
            };

            let x = i as f32 * step_width + step_width / 2.0;
            let y = chart_height - (ratio * chart_height);
            path_points.push(iced::Point::new(x, y));

            let col_item = B::vstack(
                vec![
                    // Chart Drawing Area (Fixed Height)
                    B::vstack(
                        vec![
                            B::rectangle(
                                Length::Fixed(1.0),
                                Length::Fixed((y - 4.0).max(0.0)),
                                None,
                                0.0,
                                0.0,
                                None,
                                context,
                            ),
                            B::circle(4.0, Some(self.color), context),
                            B::rectangle(
                                Length::Fixed(1.0),
                                Length::Fixed((chart_height - y - 4.0).max(0.0)),
                                Some(self.color.scale_alpha(0.2)),
                                0.0,
                                0.0,
                                None,
                                context,
                            ),
                        ],
                        0.0,
                        Padding::ZERO,
                        Length::Fixed(step_width),
                        Length::Fixed(chart_height),
                        Alignment::Center,
                        Alignment::End,
                        context,
                    ),
                    // Label and Value Area
                    B::vstack(
                        vec![
                            B::text(
                                point.label.clone(),
                                11.0,
                                None,
                                true, // Bold Title
                                false,
                                None,
                                None,
                                Length::Shrink,
                                Alignment::Center,
                                context,
                            ),
                            B::text(
                                format!("{:.0}", point.value),
                                10.0,
                                None,
                                false,
                                true, // Dim Value
                                None,
                                None,
                                Length::Shrink,
                                Alignment::Center,
                                context,
                            ),
                        ],
                        2.0,
                        Padding::ZERO,
                        Length::Shrink,
                        Length::Shrink,
                        Alignment::Center,
                        Alignment::Start,
                        context,
                    ),
                ],
                8.0,
                Padding::ZERO,
                Length::Fixed(step_width),
                Length::Shrink,
                Alignment::Center,
                Alignment::Start,
                context,
            );
            markers.push(col_item);
        }

        let line_width = step_width * self.data.len() as f32;

        B::zstack(
            vec![
                B::path(path_points, Some(self.color), 2.0, context),
                B::hstack(
                    markers,
                    0.0,
                    Padding::ZERO,
                    Length::Fixed(line_width),
                    Length::Fill,
                    Alignment::Start,
                    Alignment::End,
                    context,
                ),
            ],
            Length::Fixed(line_width),
            Length::Fill,
            Alignment::End,
            context,
        )
    }

    fn render_pie(&self, context: &Context) -> B::AnyView<Message> {
        let total_value: f32 = self.data.iter().map(|d| d.value).sum();
        let mut current_angle = -std::f32::consts::FRAC_PI_2;
        let mut arcs = Vec::new();
        let mut segments = Vec::new();
        let radius = 60.0;

        for (i, point) in self.data.iter().enumerate() {
            let color = if self.palette.is_empty() {
                self.color.scale_alpha(1.0 - (i as f32 * 0.2))
            } else {
                self.palette[i % self.palette.len()]
            };

            let sweep = if total_value > 0.0 {
                (point.value / total_value) * 2.0 * std::f32::consts::PI
            } else {
                0.0
            };

            arcs.push(B::arc(
                radius,
                current_angle,
                current_angle + sweep,
                Some(color),
                context,
            ));
            current_angle += sweep;

            segments.push(B::hstack(
                vec![
                    B::circle(6.0, Some(color), context),
                    B::text(
                        format!("{}: {:.0}", point.label, point.value),
                        12.0,
                        None,
                        false,
                        false,
                        None,
                        None,
                        Length::Shrink,
                        Alignment::Start,
                        context,
                    ),
                ],
                8.0,
                Padding::ZERO,
                Length::Shrink,
                Length::Shrink,
                Alignment::Center,
                Alignment::Start,
                context,
            ));
        }

        B::hstack(
            vec![
                B::container(
                    B::zstack(
                        arcs,
                        Length::Fixed(radius * 2.0),
                        Length::Fixed(radius * 2.0),
                        Alignment::Center,
                        context,
                    ),
                    Padding::ZERO,
                    Length::Fixed(140.0),
                    Length::Fixed(140.0),
                    None,
                    0.0,
                    0.0,
                    None,
                    None,
                    Alignment::Center,
                    Alignment::Center,
                    context,
                ),
                B::vstack(
                    segments,
                    8.0,
                    Padding::ZERO,
                    Length::Shrink,
                    Length::Shrink,
                    Alignment::Start,
                    Alignment::Center,
                    context,
                ),
            ],
            32.0,
            Padding::ZERO,
            Length::Fill,
            Length::Fill,
            Alignment::Center,
            Alignment::Center,
            context,
        )
    }
}
