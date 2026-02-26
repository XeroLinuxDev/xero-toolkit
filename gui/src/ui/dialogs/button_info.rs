//! Reusable button information dialog system.
//!
//! Button info metadata is loaded from page-scoped TOML files embedded in
//! GResources so content can be maintained outside Rust source.

use adw::prelude::*;
use adw::AlertDialog;
use gtk4::{gio, Builder, Button, GestureClick, Window};
use log::{error, warn};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

const BUTTON_INFO_RESOURCES: &[&str] = &[
    "/xyz/xerolinux/xero-toolkit/data/button_info/main_page.toml",
    "/xyz/xerolinux/xero-toolkit/data/button_info/drivers.toml",
    "/xyz/xerolinux/xero-toolkit/data/button_info/gaming_tools.toml",
    "/xyz/xerolinux/xero-toolkit/data/button_info/containers_vms.toml",
    "/xyz/xerolinux/xero-toolkit/data/button_info/customization.toml",
    "/xyz/xerolinux/xero-toolkit/data/button_info/servicing.toml",
    "/xyz/xerolinux/xero-toolkit/data/button_info/biometrics.toml",
    "/xyz/xerolinux/xero-toolkit/data/button_info/gamescope.toml",
    "/xyz/xerolinux/xero-toolkit/data/button_info/kernel_schedulers.toml",
];

/// Informational content for a button action.
#[derive(Clone, Debug, Deserialize)]
pub struct ButtonInfo {
    pub title: String,
    pub summary: String,
    #[serde(default)]
    pub details: Vec<String>,
    pub caution: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ButtonInfoDocument {
    #[serde(default)]
    button: Vec<ButtonInfoEntry>,
}

#[derive(Debug, Deserialize)]
struct ButtonInfoEntry {
    id: String,
    title: String,
    summary: String,
    #[serde(default)]
    details: Vec<String>,
    caution: Option<String>,
}

static BUTTON_INFO_REGISTRY: OnceLock<HashMap<String, ButtonInfo>> = OnceLock::new();

/// Attach info-dialog behavior to all known buttons present in this builder.
///
/// Users can right-click any attached button to open a detailed info dialog.
pub fn attach_to_builder(builder: &Builder, parent: &Window) {
    let mut button_ids: Vec<&str> = button_info_registry().keys().map(String::as_str).collect();
    button_ids.sort_unstable();

    for button_id in button_ids {
        if let Some(button) = builder.object::<Button>(button_id) {
            attach_to_button(&button, parent, button_id);
        }
    }
}

/// Attach info behavior to a single button.
pub fn attach_to_button(button: &Button, parent: &Window, button_id: &str) {
    if let Some(info) = get_button_info(button_id) {
        button.set_tooltip_text(Some(&format!("{} Right-click for details.", info.summary)));
    }

    let parent = parent.clone();
    let button_id = button_id.to_string();
    let click = GestureClick::builder().button(3).build();
    click.connect_pressed(move |_, _, _, _| {
        show_button_info_dialog(&parent, &button_id);
    });
    button.add_controller(click);
}

/// Show info dialog for a specific button id.
pub fn show_button_info_dialog(parent: &Window, button_id: &str) {
    let Some(info) = get_button_info(button_id) else {
        return;
    };

    let mut body = info.summary.clone();

    if !info.details.is_empty() {
        body.push_str("\n\nDetails:\n");
        for detail in &info.details {
            body.push_str("- ");
            body.push_str(detail);
            body.push('\n');
        }
    }

    if let Some(caution) = &info.caution {
        body.push_str("\nNote: ");
        body.push_str(caution);
    }

    let dialog = AlertDialog::new(Some(&info.title), Some(body.trim_end()));
    dialog.add_response("ok", "OK");
    dialog.set_default_response(Some("ok"));
    dialog.set_close_response("ok");
    dialog.present(Some(parent));
}

fn get_button_info(button_id: &str) -> Option<&'static ButtonInfo> {
    button_info_registry().get(button_id)
}

fn button_info_registry() -> &'static HashMap<String, ButtonInfo> {
    BUTTON_INFO_REGISTRY.get_or_init(load_button_info_registry)
}

fn load_button_info_registry() -> HashMap<String, ButtonInfo> {
    let mut registry = HashMap::new();

    for resource_path in BUTTON_INFO_RESOURCES {
        let Some(document) = load_button_info_document(resource_path) else {
            continue;
        };

        for entry in document.button {
            let info = ButtonInfo {
                title: entry.title,
                summary: entry.summary,
                details: entry.details,
                caution: entry.caution,
            };

            if registry.insert(entry.id.clone(), info).is_some() {
                warn!(
                    "Duplicate button info id '{}' found in {}. Latest entry wins.",
                    entry.id, resource_path
                );
            }
        }
    }

    registry
}

fn load_button_info_document(resource_path: &str) -> Option<ButtonInfoDocument> {
    let bytes = match gio::resources_lookup_data(resource_path, gio::ResourceLookupFlags::NONE) {
        Ok(bytes) => bytes,
        Err(e) => {
            error!(
                "Failed to load button info resource '{}': {}",
                resource_path, e
            );
            return None;
        }
    };

    let content = match std::str::from_utf8(bytes.as_ref()) {
        Ok(content) => content,
        Err(e) => {
            error!(
                "Button info resource '{}' is not valid UTF-8: {}",
                resource_path, e
            );
            return None;
        }
    };

    match toml::from_str::<ButtonInfoDocument>(content) {
        Ok(doc) => Some(doc),
        Err(e) => {
            error!(
                "Failed to parse button info TOML from '{}': {}",
                resource_path, e
            );
            None
        }
    }
}
