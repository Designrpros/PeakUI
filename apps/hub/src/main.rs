use peak_ui::prelude::{application, Result, Task};
use peak_ui::reference;

fn main() -> Result {
    env_logger::try_init().ok();
    log::info!("Peak Suite Hub started");

    application(
        || {
            let mut app = reference::App::default();

            // Initialization: Generate/Load PeakID
            if let Ok(cloud) = peak_cloud::PeakCloudService::new() {
                app.peak_id = cloud.peer_id().to_string();
                log::info!("Swarm Identity (PeakID): {}", app.peak_id);
            }

            // Start in the Swarm Dashboard
            app.active_tab = reference::model::Page::SwarmDashboard;
            app.navigation_mode = "Data".to_string();

            (app, Task::none())
        },
        reference::App::update,
        reference::App::view,
    )
    .title("Peak Suite Hub")
    .subscription(reference::App::subscription)
    .run()
}
