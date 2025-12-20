//! Interactive terminal dialog for running shell commands.

use gtk4::prelude::*;
use vte4::prelude::*;
use vte4::Terminal;
use gtk4::{Button, Window};
use log::{info, error};

/// Shows an interactive terminal window for the given command.
pub fn show_terminal_dialog(parent: &Window, title: &str, command: &str, args: &[&str]) {
    // Load the UI
    let builder = gtk4::Builder::from_resource(
        "/xyz/xerolinux/xero-toolkit/ui/dialogs/terminal_dialog.ui",
    );

    let window: adw::Window = builder
        .object("terminal_window")
        .expect("Failed to get terminal_window");
    let terminal: Terminal = builder
        .object("terminal")
        .expect("Failed to get terminal");
    let close_button: Button = builder
        .object("close_button")
        .expect("Failed to get close_button");

    window.set_transient_for(Some(parent));
    window.set_title(Some(title));
    
    // Set a nice monospace font
    let font_desc = gtk4::pango::FontDescription::from_string("Monospace 11");
    terminal.set_font(Some(&font_desc));

    // Setup close button
    let window_clone = window.clone();
    close_button.connect_clicked(move |_| {
        window_clone.close();
    });

    // Spawn the command
    let mut argv = vec![command.to_string()];
    argv.extend(args.iter().map(|s| s.to_string()));
    let argv_refs: Vec<&str> = argv.iter().map(|s| s.as_str()).collect();

    info!("Terminal: Spawning {:?} in interactive window", argv_refs);

    let close_button_clone = close_button.clone();
    terminal.spawn_async(
        vte4::PtyFlags::DEFAULT,
        None,
        &argv_refs,
        &[],
        gtk4::glib::SpawnFlags::SEARCH_PATH,
        || {}, // child setup
        -1,
        None::<&gtk4::gio::Cancellable>,
        move |result| {
            if let Err(e) = result {
                error!("Failed to spawn terminal command: {}", e);
            }
        }
    );

    // Enable close button when child exits
    terminal.connect_child_exited(move |_, _| {
        close_button_clone.set_sensitive(true);
    });

    window.present();
}
