#[cfg(not(target_arch = "wasm32"))]
use super::message::Command;
use super::message::Message;
use super::state::*;
use crate::prelude::*;

impl App {
    pub fn subscription(&self) -> Subscription<Message> {
        let events = event::listen().map(|event| {
            // Debug logging disabled for performance
            // if let Event::Keyboard(_) = event {
            //     log::info!("RAW EVENT: {:?}", event);
            // }
            match event {
                // Cursor tracking disabled - was causing re-render on every mouse move
                // Event::Mouse(mouse::Event::CursorMoved { position }) => {
                //     Message::UpdateCursorPos(position)
                // }
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) => {
                    Message::OpenContextMenu(Point::ORIGIN)
                }
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    Message::CloseContextMenu
                }
                _ => Message::None,
            }
        });

        let hotkeys = event::listen().map(|event| {
            if let Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) = event {
                let _is_cmd = modifiers.command() || modifiers.logo();
                let _is_ctrl = modifiers.control();

                let _is_backspace =
                    matches!(key, keyboard::Key::Named(keyboard::key::Named::Backspace));
                let _is_delete_forward =
                    matches!(key, keyboard::Key::Named(keyboard::key::Named::Delete));
                let _is_d = matches!(key, keyboard::Key::Character(ref c) if c.as_str() == "d");
                let _is_u = matches!(key, keyboard::Key::Character(ref c) if c.as_str() == "u");

                // if is_backspace && is_cmd {
                //    return Message::Back;
                // }

                // Cmd+D -> Toggle Dark Mode
                // if is_d && is_cmd {
                //    return Message::ToggleTheme;
                // }

                // Ctrl+U -> Close Context Menu
                if _is_u && _is_ctrl {
                    return Message::CloseContextMenu;
                }
            }
            Message::None
        });
        let window_events = event::listen_with(|event, _status, _window| match event {
            Event::Window(window::Event::Resized(size)) => Some(Message::WindowResized(size)),
            _ => None,
        });

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;

            let hash_sub = Subscription::run(|| {
                let (sender, receiver) = crate::prelude::futures::channel::mpsc::channel(1);
                let window = web_sys::window().expect("window not found");

                let on_hash_change = wasm_bindgen::prelude::Closure::wrap(Box::new(move || {
                    let hash = web_sys::window()
                        .and_then(|w| w.location().hash().ok())
                        .unwrap_or_default();

                    let path = if hash.starts_with('#') {
                        &hash[1..]
                    } else {
                        &hash
                    };

                    let page = crate::reference::AppPage::from_path(path);

                    // Defer the message sending to the next event loop tick
                    // to avoid RefCell borrowing conflicts in winit/iced
                    let mut sender = sender.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let _ = sender.try_send(Message::SetTab(page));
                    });
                })
                    as Box<dyn FnMut()>);

                window.set_onhashchange(Some(on_hash_change.as_ref().unchecked_ref()));
                on_hash_change.forget(); // Keep closure alive

                receiver
            });

            Subscription::batch(vec![
                events,
                hash_sub,
                hotkeys,
                window_events,
                iced::time::every(std::time::Duration::from_millis(100))
                    .map(|_| Message::Heartbeat),
                iced::time::every(std::time::Duration::from_millis(100)).map(|_t| {
                    #[cfg(target_arch = "wasm32")]
                    let t = wasmtimer::std::Instant::now(); // iced::time returns std::time::Instant, but we need wasmtimer's

                    Message::TypewriterTick(t)
                }),
            ])
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let command_sub = Subscription::run(|| {
                let (mut sender, receiver) = crate::prelude::futures::channel::mpsc::channel(1);

                tokio::spawn(async move {
                    loop {
                        if let Ok(content) = std::fs::read_to_string(".peak/command.json") {
                            if let Ok(cmd) = serde_json::from_str::<Command>(&content) {
                                let _ = sender.try_send(cmd.into_message());
                                let _ = std::fs::remove_file(".peak/command.json");
                            }
                        }
                        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                    }
                });

                receiver
            });

            let exposure_sub = {
                #[cfg(feature = "intelligence")]
                {
                    if self.enable_exposure {
                        Subscription::run(|| {
                            let (sender, receiver) =
                                crate::prelude::futures::channel::mpsc::channel(100);
                            tokio::spawn(crate::reference::intelligence::exposure::run_server(
                                sender,
                            ));
                            receiver
                        })
                    } else {
                        Subscription::none()
                    }
                }
                #[cfg(not(feature = "intelligence"))]
                Subscription::none()
            };

            Subscription::batch(vec![
                events,
                command_sub,
                exposure_sub,
                hotkeys,
                window_events,
                iced::time::every(std::time::Duration::from_millis(100))
                    .map(|_| Message::Heartbeat),
                iced::time::every(std::time::Duration::from_millis(100))
                    .map(Message::TypewriterTick),
            ])
        }
    }
}
