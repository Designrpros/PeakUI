use iced::Result;
use peak_ui::reference;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

fn main() -> Result {
    #[cfg(not(target_arch = "wasm32"))]
    {
        iced::application(
            "PeakUI Showcase",
            reference::App::update,
            reference::App::view,
        )
        .subscription(reference::App::subscription)
        .run()
    }

    #[cfg(target_arch = "wasm32")]
    {
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("Console log failed");
    log::info!("PeakUI Showcase WASM started");

    // Enable context menu in WASM by preventing default on the window
    // to allow right-click to work even if iced/winit tries to block it.
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        let window = web_sys::window().expect("window not found");
        let document = window.document().expect("document not found");
        let body = document.body().expect("body not found");

        let on_context_menu =
            wasm_bindgen::prelude::Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
                e.prevent_default();
                e.stop_propagation();
            })
                as Box<dyn FnMut(web_sys::MouseEvent)>);

        body.add_event_listener_with_callback(
            "contextmenu",
            on_context_menu.as_ref().unchecked_ref(),
        )
        .expect("failed to add context menu listener");
        on_context_menu.forget();
    }

    let result = iced::application(
        "PeakUI Showcase",
        reference::App::update,
        reference::App::view,
    )
    .window(iced::window::Settings {
        visible: true,
        #[cfg(target_arch = "wasm32")]
        platform_specific: iced::window::settings::PlatformSpecific {
            target: None, // Use the default canvas target
        },
        ..Default::default()
    })
    .style(|_theme, _style| iced::application::Appearance {
        background_color: iced::Color::BLACK,
        text_color: iced::Color::WHITE,
    })
    .font(include_bytes!("../assets/fonts/Fira_Sans/FiraSans-Bold.ttf").as_slice())
    .subscription(reference::App::subscription)
    .run_with(|| {
        #[cfg(target_arch = "wasm32")]
        let (initial_page, hash, path) = {
            let h = web_sys::window()
                .and_then(|w| w.location().hash().ok())
                .unwrap_or_default();

            // Remove '#' if present
            let p = if h.starts_with('#') {
                h[1..].to_string()
            } else {
                h.clone()
            };

            let page = reference::model::Page::from_path(&p);
            (page, h, p)
        };

        #[cfg(target_arch = "wasm32")]
        log::info!(
            "BOOTING - Hash: '{}', Path: '{}', Page: {:?}, Mode: {}",
            hash,
            path,
            initial_page,
            initial_page.navigation_mode()
        );

        #[cfg(not(target_arch = "wasm32"))]
        let initial_page = reference::model::Page::default();

        let mut app = reference::App::default();
        app.navigation_mode = initial_page.navigation_mode();
        app.active_tab = initial_page;

        (
            app,
            iced::font::load(
                include_bytes!("../assets/fonts/Fira_Sans/FiraSans-Bold.ttf").as_slice(),
            )
            .map(|_| reference::app::Message::FontLoaded(Ok(()))),
        )
    });

    #[cfg(target_arch = "wasm32")]
    if let Err(e) = result {
        log::error!("Iced run failed: {:?}", e);
    }
}
