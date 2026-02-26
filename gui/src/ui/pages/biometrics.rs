//! Biometrics page button handlers.
//!
//! Handles:
//! - Fingerprint reader setup (xfprintd-gui)
//! - Howdy facial recognition setup (xero-howdy-qt)

use crate::core;
use crate::ui::task_runner::{self, Command, CommandSequence};
use crate::ui::utils::extract_widget;
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Builder};
use log::{error, info};
use std::process::{Command as StdCommand, Stdio};

/// Set up all button handlers for the biometrics page
pub fn setup_handlers(page_builder: &Builder, _main_builder: &Builder, window: &ApplicationWindow) {
    crate::ui::dialogs::button_info::attach_to_builder(page_builder, window.upcast_ref());
    setup_fingerprint(page_builder, window);
    setup_howdy(page_builder, window);
}

/// Helper to update button appearance based on installation status and control uninstall button visibility
fn update_button_state(
    install_button: &gtk4::Button,
    uninstall_button: &gtk4::Button,
    is_installed: bool,
) {
    if is_installed {
        install_button.set_label("Launch App");
        install_button.add_css_class("suggested-action");
        // Show uninstall when installed; UI defines icon/styling
        uninstall_button.set_visible(true);
        uninstall_button.set_sensitive(true);
    } else {
        install_button.set_label("Install");
        install_button.remove_css_class("suggested-action");
        // Hide uninstall when not installed
        uninstall_button.set_visible(false);
        uninstall_button.set_sensitive(false);
    }
}

fn setup_fingerprint(page_builder: &Builder, window: &ApplicationWindow) {
    // Both buttons are expected to be defined in the UI; we will simply toggle visibility.
    let btn_fingerprint_setup =
        extract_widget::<gtk4::Button>(page_builder, "btn_fingerprint_setup");
    let btn_fingerprint_uninstall =
        extract_widget::<gtk4::Button>(page_builder, "btn_fingerprint_uninstall");

    // Initial check
    let is_installed = core::is_package_installed("xfprintd-gui");
    update_button_state(
        &btn_fingerprint_setup,
        &btn_fingerprint_uninstall,
        is_installed,
    );

    // Update on window focus (e.g. after installation completes)
    let btn_clone = btn_fingerprint_setup.clone();
    let uninstall_clone = btn_fingerprint_uninstall.clone();
    window.connect_is_active_notify(move |window| {
        if window.is_active() {
            let is_installed = core::is_package_installed("xfprintd-gui");
            update_button_state(&btn_clone, &uninstall_clone, is_installed);
        }
    });

    let window_clone = window.clone();
    btn_fingerprint_setup.connect_clicked(move |_| {
        info!("Biometrics: Fingerprint setup button clicked");

        // Check again at click time
        if core::is_package_installed("xfprintd-gui") {
            info!("Launching xfprintd-gui...");
            if let Err(e) = StdCommand::new("xfprintd-gui")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
            {
                error!("Failed to launch xfprintd-gui: {}", e);
            }
        } else {
            let commands = CommandSequence::new()
                .then(
                    Command::builder()
                        .aur()
                        .args(&["-S", "--noconfirm", "--needed", "xfprintd-gui"])
                        .description("Installing Fingerprint GUI Tool...")
                        .build(),
                )
                .build();

            task_runner::run(
                window_clone.upcast_ref(),
                commands,
                "Install Fingerprint GUI Tool",
            );
        }
    });

    // Uninstall action (button is defined in UI; visibility is toggled by update_button_state)
    let window_uninstall = window.clone();
    btn_fingerprint_uninstall.connect_clicked(move |_| {
        info!("Biometrics: Fingerprint uninstall clicked");

        // Build a removal command sequence via the AUR helper (same pattern as installs)
        let commands = CommandSequence::new()
            .then(
                Command::builder()
                    .aur()
                    .args(&["-R", "--noconfirm", "xfprintd-gui"])
                    .description("Removing Fingerprint GUI Tool...")
                    .build(),
            )
            .build();

        task_runner::run(
            window_uninstall.upcast_ref(),
            commands,
            "Remove Fingerprint GUI Tool",
        );
    });
}

fn setup_howdy(page_builder: &Builder, _window: &ApplicationWindow) {
    let btn_howdy_setup = extract_widget::<gtk4::Button>(page_builder, "btn_howdy_setup");

    // Disable Howdy setup as it's not ready yet
    btn_howdy_setup.set_sensitive(false);
}

// fn setup_howdy(page_builder: &Builder, window: &ApplicationWindow) {
//     let btn_howdy_setup = extract_widget::<gtk4::Button>(page_builder, "btn_howdy_setup");

//     // Initial check
//     let is_installed = core::is_package_installed("xero-howdy-qt");
//     update_button_state(&btn_howdy_setup, is_installed);

//     // Update on window focus (e.g. after installation completes)
//     let btn_clone = btn_howdy_setup.clone();
//     window.connect_is_active_notify(move |window| {
//         if window.is_active() {
//             let is_installed = core::is_package_installed("xero-howdy-qt");
//             update_button_state(&btn_clone, is_installed);
//         }
//     });

//     let window = window.clone();
//     btn_howdy_setup.connect_clicked(move |_| {
//         info!("Biometrics: Howdy setup button clicked");

//         // Check again at click time
//         if core::is_package_installed("xero-howdy-qt") {
//             info!("Launching xero-howdy-qt...");
//             if let Err(e) = StdCommand::new("xero-howdy-qt")
//                 .stdin(Stdio::null())
//                 .stdout(Stdio::null())
//                 .stderr(Stdio::null())
//                 .spawn()
//             {
//                 error!("Failed to launch xero-howdy-qt: {}", e);
//             }
//         } else {
//             let commands = CommandSequence::new()
//                 .then(
//                     Command::builder()
//                         .aur()
//                         .args(&["-S", "--noconfirm", "--needed", "xero-howdy-qt"])
//                         .description("Installing Xero Howdy Qt...")
//                         .build(),
//                 )
//                 .build();

//             task_runner::run(window.upcast_ref(), commands, "Install Xero Howdy Qt");
//         }
//     });
// }
