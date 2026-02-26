//! Reusable button information dialog system.
//!
//! Button info metadata is loaded from page-scoped TOML files embedded in
//! GResources so content can be maintained outside Rust source.

use crate::ui::utils::extract_widget;
use gtk4::prelude::*;
use gtk4::{
    gio, Box as GtkBox, Builder, Button, EventSequenceState, GestureClick, Image, Label,
    Orientation, Window,
};
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
    click.connect_pressed(|gesture, _, _, _| {
        // Claim secondary-click sequences so the button does not keep a pressed highlight.
        gesture.set_state(EventSequenceState::Claimed);
    });
    click.connect_released(move |_, _, _, _| {
        show_button_info_dialog(&parent, &button_id);
    });
    button.add_controller(click);
}

/// Show info dialog for a specific button id.
pub fn show_button_info_dialog(parent: &Window, button_id: &str) {
    let Some(info) = get_button_info(button_id) else {
        return;
    };

    let builder = Builder::from_resource(crate::config::resources::dialogs::BUTTON_INFO);

    let dialog: Window = extract_widget(&builder, "button_info_dialog");
    dialog.set_transient_for(Some(parent));
    dialog.set_title(Some(&info.title));

    let title_label: Label = extract_widget(&builder, "info_title_label");
    let summary_label: Label = extract_widget(&builder, "info_summary_label");
    let details_header: GtkBox = extract_widget(&builder, "info_details_header_row");
    let details_frame: gtk4::Frame = extract_widget(&builder, "info_details_frame");
    let details_box: GtkBox = extract_widget(&builder, "info_details_box");
    let caution_header: GtkBox = extract_widget(&builder, "info_caution_header_row");
    let caution_card: GtkBox = extract_widget(&builder, "info_caution_card");
    let caution_label: Label = extract_widget(&builder, "info_caution_label");
    let close_button: Button = extract_widget(&builder, "info_close_button");

    title_label.set_label(&info.title);
    summary_label.set_label(&info.summary);

    if !info.details.is_empty() {
        for detail in &info.details {
            add_detail_row(&details_box, detail);
        }
    } else {
        details_header.set_visible(false);
        details_frame.set_visible(false);
        details_box.set_visible(false);
    }

    if let Some(caution) = &info.caution {
        caution_label.set_label(caution);
    } else {
        caution_header.set_visible(false);
        caution_card.set_visible(false);
    }

    let dialog_clone = dialog.clone();
    close_button.connect_clicked(move |_| dialog_clone.close());

    dialog.present();
}

fn add_detail_row(details_box: &GtkBox, detail: &str) {
    let row = GtkBox::new(Orientation::Horizontal, 8);

    let icon = Image::from_icon_name("circle-check-symbolic");
    icon.set_pixel_size(14);
    row.append(&icon);

    let label = Label::new(Some(detail));
    label.set_wrap(true);
    label.set_xalign(0.0);
    row.append(&label);

    details_box.append(&row);
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
