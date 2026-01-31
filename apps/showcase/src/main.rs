use peak_ui::prelude::{Result, Task};
// use peak_ui::core::App as _;
use peak_ui::reference; // Import trait for method usage

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

fn main() -> Result {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::try_init().ok();
        log::info!("PeakUI Showcase Native started");

        let icon_bytes = include_bytes!("../assets/app_logo.png");

        #[cfg(target_os = "macos")]
        {
            use muda::{AboutMetadata, Menu, MenuItem, PredefinedMenuItem, Submenu};
            use objc2::ClassType;
            use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSImage};
            use objc2_foundation::{MainThreadMarker, NSData};

            if let Some(mtm) = MainThreadMarker::new() {
                let app = NSApplication::sharedApplication(mtm);
                // Force regular activation policy to ensure dock presence
                app.setActivationPolicy(NSApplicationActivationPolicy::Regular);

                let data = NSData::with_bytes(icon_bytes);
                if let Some(image) = NSImage::initWithData(NSImage::alloc(), &data) {
                    unsafe { app.setApplicationIconImage(Some(&image)) };
                }

                // Initialize Native Menu
                let app_menu = Submenu::new("PeakUI", true);
                let _ = app_menu.append_items(&[
                    &PredefinedMenuItem::about(
                        Some("About PeakUI"),
                        Some(AboutMetadata::default()),
                    ),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::services(None),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::hide(None),
                    &PredefinedMenuItem::hide_others(None),
                    &PredefinedMenuItem::show_all(None),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::quit(None),
                ]);

                let file_menu = Submenu::new("File", true);
                let _ = file_menu
                    .append_items(&[&PredefinedMenuItem::close_window(Some("Close Window"))]);

                let edit_menu = Submenu::new("Edit", true);
                let _ = edit_menu.append_items(&[
                    &PredefinedMenuItem::undo(None),
                    &PredefinedMenuItem::redo(None),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::cut(None),
                    &PredefinedMenuItem::copy(None),
                    &PredefinedMenuItem::paste(None),
                    &PredefinedMenuItem::select_all(None),
                ]);

                let view_menu = Submenu::new("View", true);
                let _ = view_menu.append_items(&[&PredefinedMenuItem::fullscreen(None)]);

                let window_menu = Submenu::new("Window", true);
                let _ = window_menu.append_items(&[
                    &PredefinedMenuItem::minimize(None),
                    &PredefinedMenuItem::separator(),
                    &PredefinedMenuItem::bring_all_to_front(None),
                ]);

                let help_menu = Submenu::new("Help", true);
                let _ = help_menu.append_items(&[&MenuItem::new("Documentation", true, None)]);

                let menu = Menu::new();
                let _ = menu.append_items(&[
                    &app_menu,
                    &file_menu,
                    &edit_menu,
                    &view_menu,
                    &window_menu,
                    &help_menu,
                ]);

                let _ = menu.init_for_nsapp();

                // Initialize Tray Icon
                use tray_icon::{Icon, TrayIconBuilder};
                let tray_icon_img = image::load_from_memory(icon_bytes).ok().and_then(|img| {
                    let img = img.to_rgba8();
                    let (width, height) = img.dimensions();
                    let rgba = img.into_raw();
                    Icon::from_rgba(rgba, width, height).ok()
                });

                if let Some(icon) = tray_icon_img {
                    let _tray_icon = TrayIconBuilder::new()
                        .with_tooltip("PeakUI Showcase")
                        .with_icon(icon)
                        .build()
                        .unwrap();
                    // Leak it to keep it alive
                    Box::leak(Box::new(_tray_icon));
                }
            }
        }

        let icon = image::load_from_memory(icon_bytes).ok().and_then(|img| {
            let img = img.to_rgba8();
            let (width, height) = img.dimensions();
            let rgba = img.into_raw();
            iced::window::icon::from_rgba(rgba, width, height).ok()
        });

        iced::application(
            || {
                let mut app = reference::App::default();
                let ftl = include_str!("../assets/locales/en-US/main.ftl");
                app.localization =
                    peak_ui::prelude::Localization::new("en-US", vec![ftl.to_string()]);
                (
                    app,
                    Task::perform(async {}, |_| {
                        peak_ui::reference::Message::ApplyNativeVibrancy
                    }),
                )
            },
            reference::App::update,
            reference::App::view,
        )
        .title("PeakUI Showcase")
        .window(iced::window::Settings {
            icon,
            transparent: true,
            platform_specific: iced::window::settings::PlatformSpecific {
                title_hidden: true,
                titlebar_transparent: true,
                fullsize_content_view: true,
            },
            ..Default::default()
        })
        .subscription(reference::App::subscription)
        .run()
    }

    #[cfg(target_arch = "wasm32")]
    {
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
fn app_title(_: &reference::App) -> String {
    "PeakUI Showcase".to_string()
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

    let result = peak_ui::prelude::application(
        || {
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

            // Load Showcase Localizations
            let ftl = include_str!("../assets/locales/en-US/main.ftl");
            app.localization = peak_ui::prelude::Localization::new("en-US", vec![ftl.to_string()]);

            (app, Task::none())
        },
        reference::App::update,
        reference::App::view,
    )
    .title(app_title)
    .window(peak_ui::prelude::window::Settings {
        visible: true,
        platform_specific: peak_ui::prelude::window::settings::PlatformSpecific {
            target: Some("iced-canvas".to_string()),
            ..Default::default()
        },
        ..Default::default()
    })
    .font(include_bytes!("../assets/fonts/Fira_Sans/FiraSans-Bold.ttf").as_slice())
    .font(include_bytes!("../assets/fonts/Fira_Sans/FiraSans-Regular.ttf").as_slice())
    .font(include_bytes!("../assets/fonts/Fira_Sans/FiraSans-Medium.ttf").as_slice())
    .font(include_bytes!("../assets/fonts/Fira_Sans/FiraSans-SemiBold.ttf").as_slice())
    .font(include_bytes!("../assets/fonts/Fira_Sans/FiraSans-Light.ttf").as_slice())
    .font(include_bytes!("../assets/fonts/Noto_Color_Emoji/NotoColorEmoji.ttf").as_slice())
    .subscription(reference::App::subscription)
    .run();

    #[cfg(target_arch = "wasm32")]
    if let Err(e) = result {
        log::error!("Iced run failed: {:?}", e);
    }
}
