mod app;
mod pages;
mod views;

use crate::app::HubApp;
use peak_ui::prelude::{application, Result, Task};

fn main() -> Result {
    env_logger::try_init().ok();
    log::info!("Peak Suite Hub started");

    application(
        || {
            let mut app = HubApp::default();

            // Initialization: Generate/Load PeakID
            if let Ok(cloud) = peak_cloud::PeakCloudService::new() {
                app.peak_id = cloud.peer_id().to_string();
                log::info!("Swarm Identity (PeakID): {}", app.peak_id);
            }

            (app, Task::none())
        },
        HubApp::update,
        HubApp::view,
    )
    .title("Peak Suite Hub")
    .subscription(HubApp::subscription)
    .run()
}
