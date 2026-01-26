use iced::{Element, Length, Task};
use peak_ui::prelude::*;
use std::time::{Duration, Instant};

#[cfg(not(target_arch = "wasm32"))]
pub fn main() -> iced::Result {
    iced::application(
        "PeakUI - Motion Showcase",
        MotionDemo::update,
        MotionDemo::view,
    )
    .subscription(MotionDemo::subscription)
    .run()
}

#[cfg(target_arch = "wasm32")]
pub fn main() {}

struct MotionDemo {
    button_scale: MotionState,
    card_offset: (MotionState, MotionState),
    last_tick: Instant,
}

impl Default for MotionDemo {
    fn default() -> Self {
        Self {
            button_scale: MotionState::new(1.0),
            card_offset: (MotionState::new(0.0), MotionState::new(0.0)),
            last_tick: Instant::now(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(Instant),
    ButtonPressed,
    ButtonReleased,
}

impl MotionDemo {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick(now) => {
                let dt = now - self.last_tick;
                self.last_tick = now;

                self.button_scale.update(dt, Spring::bouncy());

                // Card floating
                let t = now.duration_since(self.last_tick).as_secs_f32() * 2.0;
                self.card_offset.0.set_target(t.sin() * 20.0);
                self.card_offset.1.set_target(t.cos() * 10.0);

                self.card_offset.0.update(dt, Spring::soft());
                self.card_offset.1.update(dt, Spring::soft());
            }
            Message::ButtonPressed => {
                self.button_scale.set_target(0.9);
            }
            Message::ButtonReleased => {
                self.button_scale.set_target(1.0);
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        responsive(
            ShellMode::Desktop,
            ThemeTokens::get(ShellMode::Desktop, ThemeTone::Dark),
            |context| {
                VStack::new()
                    .push(Text::new("Motion Engine").size(48.0))
                    .push(Text::new("Smooth Spring Physics").size(18.0))
                    .push(Space::new(Length::Fixed(0.0), Length::Fixed(40.0)))
                    .push(
                        HStack::new()
                            .push(Button::label("Pressed").on_press(Message::ButtonPressed))
                            .push(Button::label("Released").on_press(Message::ButtonReleased))
                            .spacing(20.0),
                    )
                    .push(Space::new(Length::Fixed(0.0), Length::Fixed(40.0)))
                    .push(
                        GlassCard::new(
                            VStack::new()
                                .push(Text::new("Phase 3: Hardware & Motion"))
                                .push(Text::new("Real-time interpolation demo").size(14.0)),
                        )
                        .width(Length::Fixed(300.0))
                        .height(Length::Fixed(180.0)),
                    )
                    .spacing(10.0)
                    .view(&context)
            },
        )
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::time::every(Duration::from_millis(16)).map(Message::Tick)
    }
}
