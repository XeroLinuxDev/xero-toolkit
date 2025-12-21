//! About dialog showing creator information and credits.

use crate::core::package;
use crate::ui::app::extract_widget;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{Builder, Button, Label, Window};

/// Show the about dialog.
pub fn show_about_dialog(parent: &Window) {
    // Load the UI from resource
    let builder = Builder::from_resource("/xyz/xerolinux/xero-toolkit/ui/dialogs/about_dialog.ui");

    // Get the dialog window
    let dialog: Window = extract_widget(&builder, "about_window");

    // Get the close button
    let close_button: Button = extract_widget(&builder, "close_button");

    // Get labels with links and set up link activation
    if let Some(darkxero_label) = builder.object::<Label>("darkxero_donate_label") {
        darkxero_label.connect_activate_link(|_, uri| {
            if let Err(e) = package::open_url(uri) {
                log::error!("Failed to open URL {}: {}", uri, e);
            }
            glib::Propagation::Stop
        });
    }

    if let Some(synse_label) = builder.object::<Label>("synse_donate_label") {
        synse_label.connect_activate_link(|_, uri| {
            if let Err(e) = package::open_url(uri) {
                log::error!("Failed to open URL {}: {}", uri, e);
            }
            glib::Propagation::Stop
        });
    }

    // Set dialog as transient for parent
    dialog.set_transient_for(Some(parent));

    // Connect close button
    let dialog_clone = dialog.clone();
    close_button.connect_clicked(move |_| {
        dialog_clone.close();
    });

    // Show the dialog
    dialog.present();
}
