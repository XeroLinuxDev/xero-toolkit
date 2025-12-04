//! Application context and UI state management.
//!
//! This module contains the application-wide context and UI component
//! references used for navigation and state management.

use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Paned, Stack};

/// Main application context with UI elements.
#[derive(Clone)]
pub struct AppContext {
    pub ui: UiComponents,
}

impl AppContext {
    /// Create a new application context with UI components.
    pub fn new(ui: UiComponents) -> Self {
        Self { ui }
    }

    /// Navigate to a specific page in the stack.
    pub fn navigate_to_page(&self, page_name: &str) {
        self.ui.stack.set_visible_child_name(page_name);
    }
}

/// UI components grouped by functionality.
#[derive(Clone)]
pub struct UiComponents {
    pub stack: Stack,
    pub tabs_container: GtkBox,
    pub main_paned: Paned,
}

impl UiComponents {
    /// Create UI components from individual widgets.
    pub fn new(stack: Stack, tabs_container: GtkBox, main_paned: Paned) -> Self {
        Self {
            stack,
            tabs_container,
            main_paned,
        }
    }

    /// Configure the sidebar paned widget with size constraints.
    pub fn configure_sidebar(&self, min_width: i32, max_width: i32) {
        self.main_paned.set_wide_handle(true);
        self.main_paned.set_shrink_start_child(false);
        self.main_paned.set_resize_start_child(false);

        self.tabs_container.set_size_request(min_width, -1);

        let position = self.main_paned.position();
        if position < min_width {
            self.main_paned.set_position(min_width);
        } else if position > max_width {
            self.main_paned.set_position(max_width);
        }

        self.main_paned
            .connect_notify_local(Some("position"), move |paned, _| {
                let pos = paned.position();
                if pos < min_width {
                    paned.set_position(min_width);
                } else if pos > max_width {
                    paned.set_position(max_width);
                }
            });
    }

    /// Get the tabs container for tab management.
    #[allow(dead_code)]
    pub fn tabs_container(&self) -> &GtkBox {
        &self.tabs_container
    }

    /// Get the stack widget for page navigation.
    #[allow(dead_code)]
    pub fn stack(&self) -> &Stack {
        &self.stack
    }
}
