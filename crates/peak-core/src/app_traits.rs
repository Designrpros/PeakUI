// Trait definitions for PeakOS apps
// This module defines the common interface that all built-in apps implement

use crate::theme::Theme;
use iced::{Element, Subscription, Task};

/// Theme information passed to apps for consistent styling
/// Information about an app's window state
#[derive(Debug, Clone, Copy)]
pub struct WindowInfo {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub is_focused: bool,
    pub is_minimized: bool,
}

impl Default for WindowInfo {
    fn default() -> Self {
        Self {
            x: 100.0,
            y: 100.0,
            width: 800.0,
            height: 600.0,
            is_focused: false,
            is_minimized: false,
        }
    }
}

/// Interface for apps to interact with the shell
pub trait ShellContext {
    /// Send a system notification
    fn notify(&self, title: &str, message: &str);

    /// Request an AI operation
    fn ask_intelligence(&self, prompt: &str) -> Task<String>;

    /// Close the calling app
    fn close_app(&self);

    /// Request access to the file system
    fn request_file_system(&self) -> bool;

    /// Get the absolute position of the root shell window
    fn get_root_window_position(&self) -> (f32, f32) {
        (0.0, 0.0)
    }
}

/// Core trait that all PeakOS apps implement
/// This provides a standardized interface for app lifecycle management
pub trait PeakApp {
    /// The message type this app handles
    type Message: Clone + std::fmt::Debug;

    /// The title of the app
    fn title(&self) -> String;

    /// Update the app state based on a message
    fn update(&mut self, message: Self::Message, context: &dyn ShellContext)
        -> Task<Self::Message>;

    /// Render the app UI
    fn view(&self, theme: &Theme) -> Element<'_, Self::Message>;

    /// Subscribe to external events
    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    /// Get current window information
    fn window_info(&self) -> WindowInfo {
        WindowInfo::default()
    }

    /// Handle window resizing or moving
    fn on_window_change(&mut self, _info: WindowInfo) {}
}
