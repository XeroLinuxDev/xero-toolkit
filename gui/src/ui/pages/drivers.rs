//! Drivers and hardware tools page button handlers.
//!
//! Handles:
//! - Tailscale VPN
//! - ASUS ROG laptop tools

use crate::ui::task_runner::{self, Command, CommandSequence};
use crate::ui::utils::extract_widget;
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Builder, Button};
use log::info;

/// Set up all button handlers for the drivers page.
pub fn setup_handlers(page_builder: &Builder, _main_builder: &Builder, window: &ApplicationWindow) {
    setup_tailscale(page_builder, window);
    setup_asus_rog(page_builder, window);
}

fn setup_tailscale(builder: &Builder, window: &ApplicationWindow) {
    let button = extract_widget::<Button>(builder, "btn_tailscale");
    let window = window.clone();

    button.connect_clicked(move |_| {
        info!("Tailscale VPN button clicked");

        let commands = CommandSequence::new()
            .then(Command::builder()
                .privileged()
                .program("bash")
                .args(&[
                    "-c",
                    "curl -fsSL https://raw.githubusercontent.com/xerolinux/xero-fixes/main/conf/install.sh | bash",
                ])
                .description("Installing Tailscale VPN...")
                .build())
            .build();

        task_runner::run(window.upcast_ref(), commands, "Install Tailscale VPN");
    });
}

fn setup_asus_rog(builder: &Builder, window: &ApplicationWindow) {
    let button = extract_widget::<Button>(builder, "btn_asus_rog");
    let window = window.clone();

    button.connect_clicked(move |_| {
        info!("ASUS ROG Tools button clicked");

        let commands = CommandSequence::new()
            .then(
                Command::builder()
                    .aur()
                    .args(&[
                        "-S",
                        "--noconfirm",
                        "--needed",
                        "rog-control-center",
                        "asusctl",
                        "supergfxctl",
                    ])
                    .description("Installing ASUS ROG control tools...")
                    .build(),
            )
            .then(
                Command::builder()
                    .privileged()
                    .program("systemctl")
                    .args(&["enable", "--now", "asusd", "supergfxd"])
                    .description("Enabling ASUS ROG services...")
                    .build(),
            )
            .build();

        task_runner::run(window.upcast_ref(), commands, "Install ASUS ROG Tools");
    });
}
