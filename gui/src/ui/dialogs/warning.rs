//! Warning confirmation dialog for experimental features.

use crate::ui::utils::extract_widget;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{Builder, Button, Label, Window};
use log::info;
use std::cell::RefCell;
use std::rc::Rc;

/// Show a warning confirmation dialog with cancel and continue buttons.
/// Calls on_confirm callback if user clicks continue.
pub fn show_warning_confirmation<F>(parent: &Window, heading: &str, message: &str, on_confirm: F)
where
    F: FnOnce() + 'static,
{
    info!("Showing warning confirmation dialog: {}", heading);

    // Load the UI from resource
    let builder = Builder::from_resource(crate::config::resources::dialogs::WARNING);

    // Get the dialog window
    let dialog: Window = extract_widget(&builder, "warning_dialog");

    // Set transient parent
    dialog.set_transient_for(Some(parent));

    // Get UI elements
    let heading_label: Label = extract_widget(&builder, "dialog_heading");
    let warning_message: Label = extract_widget(&builder, "warning_message");
    let cancel_button: Button = extract_widget(&builder, "cancel_button");
    let continue_button: Button = extract_widget(&builder, "continue_button");

    // Set heading (remove emoji from heading since we have an icon now)
    heading_label.set_label(heading);
    continue_button.set_label("Continue");

    // Set message with Pango markup
    warning_message.set_markup(message);
    warning_message.connect_activate_link(|_, uri| {
        if let Err(e) = crate::core::package::open_url(uri) {
            log::error!("Failed to open URL {}: {}", uri, e);
        }
        glib::Propagation::Stop
    });

    // Setup callbacks
    let dialog_clone = dialog.clone();
    cancel_button.connect_clicked(move |_| {
        info!("Warning dialog cancelled");
        dialog_clone.close();
    });

    let dialog_clone = dialog.clone();
    let on_confirm_rc = Rc::new(RefCell::new(Some(on_confirm)));

    continue_button.connect_clicked(move |_| {
        info!("Warning dialog confirmed");
        if let Some(on_confirm) = on_confirm_rc.borrow_mut().take() {
            on_confirm();
        }
        dialog_clone.close();
    });

    // Show the dialog
    dialog.present();
}
