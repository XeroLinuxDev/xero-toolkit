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
pub fn setup_handlers(
    page_builder: &Builder,
    _main_builder: &Builder,
    _window: &ApplicationWindow,
) {
    let widgets = Rc::new(extract_all_widgets(page_builder));

    connect_widget_signals(&widgets);
    setup_copy_button(page_builder, &widgets);

    // Generate initial command
    update_command_output(&widgets);
}

/// Extract all widgets from the UI builder.
fn extract_all_widgets(builder: &Builder) -> GamescopeWidgets {
    GamescopeWidgets {
        // Output (Visual)
        entry_output_width: extract_widget(builder, "entry_output_width"),
        entry_output_height: extract_widget(builder, "entry_output_height"),
        entry_max_scale: extract_widget(builder, "entry_max_scale"),

        // Nested (Game)
        entry_nested_width: extract_widget(builder, "entry_nested_width"),
        entry_nested_height: extract_widget(builder, "entry_nested_height"),
        entry_nested_refresh: extract_widget(builder, "entry_nested_refresh"),

        // Scaler / Filter
        combo_scaler: extract_widget(builder, "combo_scaler"),
        combo_filter: extract_widget(builder, "combo_filter"),
        entry_fsr_sharpness: extract_widget(builder, "entry_fsr_sharpness"),

        // Flags
        check_fullscreen: extract_widget(builder, "check_fullscreen"),
        check_grab: extract_widget(builder, "check_grab"),
        check_force_grab_cursor: extract_widget(builder, "check_force_grab_cursor"),
        check_adaptive_sync: extract_widget(builder, "check_adaptive_sync"),
        check_immediate_flips: extract_widget(builder, "check_immediate_flips"),
        check_expose_wayland: extract_widget(builder, "check_expose_wayland"),
        check_force_windows_fullscreen: extract_widget(builder, "check_force_windows_fullscreen"),

        // Backend / HDR / Misc
        combo_backend: extract_widget(builder, "combo_backend"),
        check_hdr_enabled: extract_widget(builder, "check_hdr_enabled"),
        entry_cursor_path: extract_widget(builder, "entry_cursor_path"),
        entry_framerate_limit: extract_widget(builder, "entry_framerate_limit"),

        // Debug & Extra
        check_debug_layers: extract_widget(builder, "check_debug_layers"),
        check_mangoapp: extract_widget(builder, "check_mangoapp"),
        check_realtime: extract_widget(builder, "check_realtime"),
        entry_extra_flags: extract_widget(builder, "entry_extra_flags"),

        // Output
        text_command_output: extract_widget(builder, "text_command_output"),
    }
}

/// Connect all widget signals to regenerate the command on changes.
fn connect_widget_signals(widgets: &Rc<GamescopeWidgets>) {
    // Connect entry fields
    connect_entry_signal(widgets, &widgets.entry_output_width);
    connect_entry_signal(widgets, &widgets.entry_output_height);
    connect_entry_signal(widgets, &widgets.entry_max_scale);
    connect_entry_signal(widgets, &widgets.entry_nested_width);
    connect_entry_signal(widgets, &widgets.entry_nested_height);
    connect_entry_signal(widgets, &widgets.entry_nested_refresh);
    connect_entry_signal(widgets, &widgets.entry_fsr_sharpness);
    connect_entry_signal(widgets, &widgets.entry_cursor_path);
    connect_entry_signal(widgets, &widgets.entry_framerate_limit);
    connect_entry_signal(widgets, &widgets.entry_extra_flags);

    // Connect switches
    connect_switch_signal(widgets, &widgets.check_fullscreen);
    connect_switch_signal(widgets, &widgets.check_grab);
    connect_switch_signal(widgets, &widgets.check_force_grab_cursor);
    connect_switch_signal(widgets, &widgets.check_adaptive_sync);
    connect_switch_signal(widgets, &widgets.check_immediate_flips);
    connect_switch_signal(widgets, &widgets.check_expose_wayland);
    connect_switch_signal(widgets, &widgets.check_force_windows_fullscreen);
    connect_switch_signal(widgets, &widgets.check_hdr_enabled);
    connect_switch_signal(widgets, &widgets.check_debug_layers);
    connect_switch_signal(widgets, &widgets.check_mangoapp);
    connect_switch_signal(widgets, &widgets.check_realtime);

    // Connect combo rows
    connect_combo_signal(widgets, &widgets.combo_scaler);
    connect_combo_signal(widgets, &widgets.combo_filter);
    connect_combo_signal(widgets, &widgets.combo_backend);
}

/// Connect an entry row to trigger command regeneration.
fn connect_entry_signal(widgets: &Rc<GamescopeWidgets>, entry: &EntryRow) {
    let widgets = widgets.clone();
    entry.connect_notify_local(Some("text"), move |_, _| {
        update_command_output(&widgets);
    });
}

/// Connect a switch to trigger command regeneration.
fn connect_switch_signal(widgets: &Rc<GamescopeWidgets>, switch: &Switch) {
    let widgets = widgets.clone();
    switch.connect_active_notify(move |_| {
        update_command_output(&widgets);
    });
}

/// Connect a combo row to trigger command regeneration.
fn connect_combo_signal(widgets: &Rc<GamescopeWidgets>, combo: &ComboRow) {
    let widgets = widgets.clone();
    combo.connect_selected_notify(move |_| {
        update_command_output(&widgets);
    });
}

/// Set up the copy button to copy the command to clipboard.
fn setup_copy_button(builder: &Builder, widgets: &Rc<GamescopeWidgets>) {
    let btn_copy_command = extract_widget::<Button>(builder, "btn_copy_command");
    let text_output = widgets.text_command_output.clone();
    btn_copy_command.connect_clicked(move |_| {
        let text = text_output.text();
        if let Some(display) = gtk4::gdk::Display::default() {
            let clipboard = display.clipboard();
            clipboard.set(&text);
            info!("Copied gamescope command to clipboard");
        }
    });
}

/// Update the command output field with the generated command.
fn update_command_output(widgets: &GamescopeWidgets) {
    let command = build_gamescope_command(widgets);
    widgets.text_command_output.set_text(&command);
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

    add_resolution_flags(&mut parts, widgets);
    add_scaler_flags(&mut parts, widgets);
    add_general_flags(&mut parts, widgets);
    add_backend_flags(&mut parts, widgets);
    add_debug_flags(&mut parts, widgets);
    add_extra_flags(&mut parts, widgets);

    // Add command separator
    parts.push("--".to_string());
    parts.push("%command%".to_string());

    parts.join(" ")
}

/// Add resolution and refresh rate flags.
fn add_resolution_flags(parts: &mut Vec<String>, widgets: &GamescopeWidgets) {
    // Output (Visual)
    add_flag_if_not_empty(parts, "-W", &widgets.entry_output_width.text());
    add_flag_if_not_empty(parts, "-H", &widgets.entry_output_height.text());
    add_flag_if_not_empty(parts, "-m", &widgets.entry_max_scale.text());

    // Nested (Game)
    add_flag_if_not_empty(parts, "-w", &widgets.entry_nested_width.text());
    add_flag_if_not_empty(parts, "-h", &widgets.entry_nested_height.text());
    add_flag_if_not_empty(parts, "-r", &widgets.entry_nested_refresh.text());
}

/// Add scaler and filter flags.
fn add_scaler_flags(parts: &mut Vec<String>, widgets: &GamescopeWidgets) {
    // Scaler
    if let Some(scaler) = get_combo_value(&widgets.combo_scaler) {
        if scaler != "auto" {
            parts.push(format!("-S {}", scaler));
        }
    }

    // Filter
    if let Some(filter) = get_combo_value(&widgets.combo_filter) {
        if filter != "linear" {
            parts.push(format!("-F {}", filter));
        }
    }

    // FSR sharpness
    add_flag_if_not_empty(
        parts,
        "--fsr-sharpness",
        &widgets.entry_fsr_sharpness.text(),
    );
}

/// Add general gameplay flags.
fn add_general_flags(parts: &mut Vec<String>, widgets: &GamescopeWidgets) {
    add_switch_flag(parts, "-f", &widgets.check_fullscreen);
    add_switch_flag(parts, "-g", &widgets.check_grab);
    add_switch_flag(
        parts,
        "--force-grab-cursor",
        &widgets.check_force_grab_cursor,
    );
    add_switch_flag(parts, "--adaptive-sync", &widgets.check_adaptive_sync);
    add_switch_flag(parts, "--immediate-flips", &widgets.check_immediate_flips);
    add_switch_flag(parts, "--expose-wayland", &widgets.check_expose_wayland);
    add_switch_flag(
        parts,
        "--force-windows-fullscreen",
        &widgets.check_force_windows_fullscreen,
    );
}

/// Add backend and rendering flags.
fn add_backend_flags(parts: &mut Vec<String>, widgets: &GamescopeWidgets) {
    // Backend
    if let Some(backend) = get_combo_value(&widgets.combo_backend) {
        if backend != "auto" {
            parts.push(format!("--backend {}", backend));
        }
    }

    // HDR
    add_switch_flag(parts, "--hdr-enabled", &widgets.check_hdr_enabled);

    // Cursor
    add_flag_if_not_empty(parts, "--cursor", &widgets.entry_cursor_path.text());

    // Framerate limit
    add_flag_if_not_empty(
        parts,
        "--framerate-limit",
        &widgets.entry_framerate_limit.text(),
    );
}

/// Add debug and performance flags.
fn add_debug_flags(parts: &mut Vec<String>, widgets: &GamescopeWidgets) {
    add_switch_flag(parts, "--debug-layers", &widgets.check_debug_layers);
    add_switch_flag(parts, "--mangoapp", &widgets.check_mangoapp);
    add_switch_flag(parts, "--rt", &widgets.check_realtime);
}

/// Add user-provided extra flags.
fn add_extra_flags(parts: &mut Vec<String>, widgets: &GamescopeWidgets) {
    let extra = widgets.entry_extra_flags.text();
    if !extra.is_empty() {
        parts.push(extra.to_string());
    }
}

/// Add a flag with a value if the value is not empty.
fn add_flag_if_not_empty(parts: &mut Vec<String>, flag: &str, value: &str) {
    if !value.is_empty() {
        parts.push(format!("{} {}", flag, value));
    }
}

/// Add a flag if the switch is active.
fn add_switch_flag(parts: &mut Vec<String>, flag: &str, switch: &Switch) {
    if switch.is_active() {
        parts.push(flag.to_string());
    }
}

/// Get the selected value from a combo row.
fn get_combo_value(combo: &ComboRow) -> Option<String> {
    combo.selected_item().and_then(|item| {
        item.downcast_ref::<StringObject>()
            .map(|obj| obj.string().to_string())
    })
}
