//! Gamescope configuration page.
//!
//! Handles the logic for the Gamescope command generator.

use crate::ui::utils::extract_widget;
use adw::prelude::*;
use adw::{ComboRow, EntryRow};
use gtk4::{ApplicationWindow, Builder, Button, StringObject, Switch};
use log::info;
use std::rc::Rc;

/// Set up all handlers for the gamescope page.
pub fn setup_handlers(page_builder: &Builder, _main_builder: &Builder, _window: &ApplicationWindow) {
    // Extract all widgets
    let entry_output_width = extract_widget::<EntryRow>(page_builder, "entry_output_width");
    let entry_output_height = extract_widget::<EntryRow>(page_builder, "entry_output_height");
    let entry_max_scale = extract_widget::<EntryRow>(page_builder, "entry_max_scale");

    let entry_nested_width = extract_widget::<EntryRow>(page_builder, "entry_nested_width");
    let entry_nested_height = extract_widget::<EntryRow>(page_builder, "entry_nested_height");
    let entry_nested_refresh = extract_widget::<EntryRow>(page_builder, "entry_nested_refresh");

    let combo_scaler = extract_widget::<ComboRow>(page_builder, "combo_scaler");
    let combo_filter = extract_widget::<ComboRow>(page_builder, "combo_filter");
    let entry_fsr_sharpness = extract_widget::<EntryRow>(page_builder, "entry_fsr_sharpness");

    // Flags
    let check_fullscreen = extract_widget::<Switch>(page_builder, "check_fullscreen");
    let check_grab = extract_widget::<Switch>(page_builder, "check_grab");
    let check_force_grab_cursor = extract_widget::<Switch>(page_builder, "check_force_grab_cursor");
    let check_adaptive_sync = extract_widget::<Switch>(page_builder, "check_adaptive_sync");
    let check_immediate_flips = extract_widget::<Switch>(page_builder, "check_immediate_flips");
    let check_expose_wayland = extract_widget::<Switch>(page_builder, "check_expose_wayland");
    let check_force_windows_fullscreen = extract_widget::<Switch>(page_builder, "check_force_windows_fullscreen");

    // Backend / HDR / Misc
    let combo_backend = extract_widget::<ComboRow>(page_builder, "combo_backend");
    let check_hdr_enabled = extract_widget::<Switch>(page_builder, "check_hdr_enabled");
    let entry_cursor_path = extract_widget::<EntryRow>(page_builder, "entry_cursor_path");
    let entry_framerate_limit = extract_widget::<EntryRow>(page_builder, "entry_framerate_limit");

    // Debug & Extra
    let check_debug_layers = extract_widget::<Switch>(page_builder, "check_debug_layers");
    let check_mangoapp = extract_widget::<Switch>(page_builder, "check_mangoapp");
    let check_realtime = extract_widget::<Switch>(page_builder, "check_realtime");
    let entry_extra_flags = extract_widget::<EntryRow>(page_builder, "entry_extra_flags");

    // Output
    let text_command_output = extract_widget::<EntryRow>(page_builder, "text_command_output");
    let btn_copy_command = extract_widget::<Button>(page_builder, "btn_copy_command");

    // Clone all widgets for closures
    let widgets = Rc::new(GamescopeWidgets {
        entry_output_width: entry_output_width.clone(),
        entry_output_height: entry_output_height.clone(),
        entry_max_scale: entry_max_scale.clone(),
        entry_nested_width: entry_nested_width.clone(),
        entry_nested_height: entry_nested_height.clone(),
        entry_nested_refresh: entry_nested_refresh.clone(),
        combo_scaler: combo_scaler.clone(),
        combo_filter: combo_filter.clone(),
        entry_fsr_sharpness: entry_fsr_sharpness.clone(),
        check_fullscreen: check_fullscreen.clone(),
        check_grab: check_grab.clone(),
        check_force_grab_cursor: check_force_grab_cursor.clone(),
        check_adaptive_sync: check_adaptive_sync.clone(),
        check_immediate_flips: check_immediate_flips.clone(),
        check_expose_wayland: check_expose_wayland.clone(),
        check_force_windows_fullscreen: check_force_windows_fullscreen.clone(),
        combo_backend: combo_backend.clone(),
        check_hdr_enabled: check_hdr_enabled.clone(),
        entry_cursor_path: entry_cursor_path.clone(),
        entry_framerate_limit: entry_framerate_limit.clone(),
        check_debug_layers: check_debug_layers.clone(),
        check_mangoapp: check_mangoapp.clone(),
        check_realtime: check_realtime.clone(),
        entry_extra_flags: entry_extra_flags.clone(),
        text_command_output: text_command_output.clone(),
    });

    // Command generation function
    let generate_command = {
        let widgets = widgets.clone();
        move || {
            let cmd = build_gamescope_command(&widgets);
            widgets.text_command_output.set_text(&cmd);
        }
    };

    // Connect all entry fields to auto-generate
    let entries = vec![
        &entry_output_width,
        &entry_output_height,
        &entry_max_scale,
        &entry_nested_width,
        &entry_nested_height,
        &entry_nested_refresh,
        &entry_fsr_sharpness,
        &entry_cursor_path,
        &entry_framerate_limit,
        &entry_extra_flags,
    ];

    for entry in entries {
        let gen = generate_command.clone();
        entry.connect_notify_local(Some("text"), move |_: &EntryRow, _| gen());
    }

    // Connect all checkboxes (Switches)
    let checks = vec![
        &check_fullscreen,
        &check_grab,
        &check_force_grab_cursor,
        &check_adaptive_sync,
        &check_immediate_flips,
        &check_expose_wayland,
        &check_force_windows_fullscreen,
        &check_hdr_enabled,
        &check_debug_layers,
        &check_mangoapp,
        &check_realtime,
    ];

    for check in checks {
        let gen = generate_command.clone();
        check.connect_active_notify(move |_| gen());
    }

    // Connect dropdowns
    let gen = generate_command.clone();
    combo_scaler.connect_selected_notify(move |_| gen());

    let gen = generate_command.clone();
    combo_filter.connect_selected_notify(move |_| gen());

    let gen = generate_command.clone();
    combo_backend.connect_selected_notify(move |_| gen());

    // Copy button
    btn_copy_command.connect_clicked(move |_| {
        let text = text_command_output.text();

        if let Some(display) = gtk4::gdk::Display::default() {
            let clipboard = display.clipboard();
            clipboard.set(&text);
            info!("Copied gamescope command to clipboard");
        }
    });

    // Initial generation
    generate_command();
}

/// All widgets needed for command generation
struct GamescopeWidgets {
    entry_output_width: EntryRow,
    entry_output_height: EntryRow,
    entry_max_scale: EntryRow,
    entry_nested_width: EntryRow,
    entry_nested_height: EntryRow,
    entry_nested_refresh: EntryRow,
    combo_scaler: ComboRow,
    combo_filter: ComboRow,
    entry_fsr_sharpness: EntryRow,
    check_fullscreen: Switch,
    check_grab: Switch,
    check_force_grab_cursor: Switch,
    check_adaptive_sync: Switch,
    check_immediate_flips: Switch,
    check_expose_wayland: Switch,
    check_force_windows_fullscreen: Switch,
    combo_backend: ComboRow,
    check_hdr_enabled: Switch,
    entry_cursor_path: EntryRow,
    entry_framerate_limit: EntryRow,
    check_debug_layers: Switch,
    check_mangoapp: Switch,
    check_realtime: Switch,
    entry_extra_flags: EntryRow,
    text_command_output: EntryRow,
}

/// Build the gamescope command from widget values
fn build_gamescope_command(widgets: &GamescopeWidgets) -> String {
    let mut parts = vec!["gamescope".to_string()];

    // Output (Visual)
    let w = widgets.entry_output_width.text();
    if !w.is_empty() {
        parts.push(format!("-W {}", w));
    }

    let h = widgets.entry_output_height.text();
    if !h.is_empty() {
        parts.push(format!("-H {}", h));
    }

    let m = widgets.entry_max_scale.text();
    if !m.is_empty() {
        parts.push(format!("-m {}", m));
    }

    // Nested (Game)
    let w = widgets.entry_nested_width.text();
    if !w.is_empty() {
        parts.push(format!("-w {}", w));
    }

    let h = widgets.entry_nested_height.text();
    if !h.is_empty() {
        parts.push(format!("-h {}", h));
    }

    let r = widgets.entry_nested_refresh.text();
    if !r.is_empty() {
        parts.push(format!("-r {}", r));
    }

    // Scaler
    if let Some(item) = widgets.combo_scaler.selected_item() {
        if let Some(string_obj) = item.downcast_ref::<StringObject>() {
            let val = string_obj.string();
            if val != "auto" {
                parts.push(format!("-S {}", val));
            }
        }
    }

    // Filter
    if let Some(item) = widgets.combo_filter.selected_item() {
        if let Some(string_obj) = item.downcast_ref::<StringObject>() {
            let val = string_obj.string();
            if val != "linear" {  // Don't add default
                parts.push(format!("-F {}", val));
            }
        }
    }

    // FSR sharpness
    let fsr = widgets.entry_fsr_sharpness.text();
    if !fsr.is_empty() {
        parts.push(format!("--fsr-sharpness {}", fsr));
    }

    // Flags
    if widgets.check_fullscreen.is_active() {
        parts.push("-f".to_string());
    }
    if widgets.check_grab.is_active() {
        parts.push("-g".to_string());
    }
    if widgets.check_force_grab_cursor.is_active() {
        parts.push("--force-grab-cursor".to_string());
    }
    if widgets.check_adaptive_sync.is_active() {
        parts.push("--adaptive-sync".to_string());
    }
    if widgets.check_immediate_flips.is_active() {
        parts.push("--immediate-flips".to_string());
    }
    if widgets.check_expose_wayland.is_active() {
        parts.push("--expose-wayland".to_string());
    }
    if widgets.check_force_windows_fullscreen.is_active() {
        parts.push("--force-windows-fullscreen".to_string());
    }

    // Backend
    if let Some(item) = widgets.combo_backend.selected_item() {
        if let Some(string_obj) = item.downcast_ref::<StringObject>() {
            let val = string_obj.string();
            if val != "auto" {
                parts.push(format!("--backend {}", val));
            }
        }
    }

    // HDR
    if widgets.check_hdr_enabled.is_active() {
        parts.push("--hdr-enabled".to_string());
    }

    // Cursor path
    let cursor = widgets.entry_cursor_path.text();
    if !cursor.is_empty() {
        parts.push(format!("--cursor {}", cursor));
    }

    // Framerate limit
    let fr = widgets.entry_framerate_limit.text();
    if !fr.is_empty() {
        parts.push(format!("--framerate-limit {}", fr));
    }

    // Debug flags
    if widgets.check_debug_layers.is_active() {
        parts.push("--debug-layers".to_string());
    }
    if widgets.check_mangoapp.is_active() {
        parts.push("--mangoapp".to_string());
    }
    if widgets.check_realtime.is_active() {
        parts.push("--rt".to_string());
    }

    // Extra flags
    let extra = widgets.entry_extra_flags.text();
    if !extra.is_empty() {
        parts.push(extra.to_string());
    }

    // Add command separator
    parts.push("--".to_string());
    parts.push("%command%".to_string());

    parts.join(" ")
}
