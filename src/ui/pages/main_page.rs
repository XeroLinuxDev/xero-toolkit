//! Main page button handlers.
//!
//! Handles:
//! - System update
//! - Package manager GUI installation
//! - External links (Discord, YouTube, Website, Donate)

use crate::core;
use crate::ui::command_execution as progress_dialog;
use crate::ui::selection_dialog;
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Builder, Button};
use log::{info};

/// Set up all button handlers for the main page
pub fn setup_handlers(page_builder: &Builder, _main_builder: &Builder) {
    setup_update_system_button(page_builder);
    setup_pkg_manager_button(page_builder);
    setup_external_links(page_builder);
}

/// Setup system update button
fn setup_update_system_button(page_builder: &Builder) {
    if let Some(btn_update_system) = page_builder.object::<Button>("btn_update_system") {
        btn_update_system.connect_clicked(move |button| {
            info!("Main page: Update System button clicked");
            let widget = button.clone().upcast::<gtk4::Widget>();
            let window = widget
                .root()
                .and_then(|root| root.downcast::<ApplicationWindow>().ok());

            if let Some(window) = window {
                let commands = vec![progress_dialog::CommandStep::privileged(
                    "/usr/local/bin/upd",
                    &[],
                    "Updating system packages...",
                )];

                let window_ref = window.upcast_ref::<gtk4::Window>();
                progress_dialog::run_commands_with_progress(
                    window_ref,
                    commands,
                    "System Update",
                    None,
                );
            }
        });
    }
}

/// Setup package manager GUI button
fn setup_pkg_manager_button(page_builder: &Builder) {
    if let Some(btn_pkg_manager) = page_builder.object::<Button>("btn_pkg_manager") {
        btn_pkg_manager.connect_clicked(move |button| {
            info!("Main page: PKG Manager GUI button clicked");
            show_pkg_manager_dialog(button);
        });
    }
}

/// Setup external link buttons
fn setup_external_links(page_builder: &Builder) {
    if let Some(link_discord) = page_builder.object::<Button>("link_discord") {
        link_discord.connect_clicked(move |_| {
            info!("Main page: Discord link clicked");
            let _ = std::process::Command::new("xdg-open")
                .arg("https://discord.xerolinux.xyz/")
                .spawn();
        });
    }

    if let Some(link_youtube) = page_builder.object::<Button>("link_youtube") {
        link_youtube.connect_clicked(move |_| {
            info!("Main page: YouTube link clicked");
            let _ = std::process::Command::new("xdg-open")
                .arg("https://www.youtube.com/@XeroLinux")
                .spawn();
        });
    }

    if let Some(link_website) = page_builder.object::<Button>("link_website") {
        link_website.connect_clicked(move |_| {
            info!("Main page: XeroLinux website link clicked");
            let _ = std::process::Command::new("xdg-open")
                .arg("https://xerolinux.xyz/")
                .spawn();
        });
    }

    if let Some(link_donate) = page_builder.object::<Button>("link_donate") {
        link_donate.connect_clicked(move |_| {
            info!("Main page: Donate link clicked");
            let _ = std::process::Command::new("xdg-open")
                .arg("https://ko-fi.com/xerolinux")
                .spawn();
        });
    }
}

/// Show package manager selection dialog
fn show_pkg_manager_dialog(button: &Button) {
    let widget = button.clone().upcast::<gtk4::Widget>();
    let window = widget
        .root()
        .and_then(|root| root.downcast::<ApplicationWindow>().ok());

    if let Some(window) = window {
        let window_clone = window.clone();
        let window_ref = window.upcast_ref::<gtk4::Window>();

        // Check which package managers are already installed
        let octopi_installed = core::is_package_installed("octopi");
        let pacseek_installed = core::is_package_installed("pacseek");
        let bauh_installed = core::is_package_installed("bauh");
        let warehouse_installed = core::is_flatpak_installed("io.github.flattool.Warehouse");
        let flatseal_installed = core::is_flatpak_installed("com.github.tchx84.Flatseal");
        let bazaar_installed = core::is_flatpak_installed("io.github.kolunmi.Bazaar");

        let config = selection_dialog::SelectionDialogConfig::new(
            "Package Manager GUI Applications",
            "Select which package manager GUIs to install. Multiple selections allowed.",
        )
        .add_option(selection_dialog::SelectionOption::new(
            "octopi",
            "Octopi",
            "Powerful Pacman GUI with AUR support",
            octopi_installed,
        ))
        .add_option(selection_dialog::SelectionOption::new(
            "pacseek",
            "PacSeek",
            "Terminal UI package manager with search",
            pacseek_installed,
        ))
        .add_option(selection_dialog::SelectionOption::new(
            "bauh",
            "Bauh",
            "Manage Pacman, AUR, Flatpak, Snap packages",
            bauh_installed,
        ))
        .add_option(selection_dialog::SelectionOption::new(
            "warehouse",
            "Warehouse",
            "Flatpak package manager (Flatpak)",
            warehouse_installed,
        ))
        .add_option(selection_dialog::SelectionOption::new(
            "flatseal",
            "Flatseal",
            "Flatpak permissions manager (Flatpak)",
            flatseal_installed,
        ))
        .add_option(selection_dialog::SelectionOption::new(
            "bazaar",
            "Bazaar",
            "Browse and install Flatpak apps (Flatpak)",
            bazaar_installed,
        ))
        .confirm_label("Install");

        selection_dialog::show_selection_dialog(window_ref, config, move |selected_ids| {
            let mut commands = vec![];

            if selected_ids.contains(&"octopi".to_string()) {
                commands.push(progress_dialog::CommandStep::aur(
                    &["-S", "--noconfirm", "--needed", "octopi"],
                    "Installing Octopi package manager...",
                ));
            }

            if selected_ids.contains(&"pacseek".to_string()) {
                commands.push(progress_dialog::CommandStep::aur(
                    &["-S", "--noconfirm", "--needed", "pacseek", "pacfinder"],
                    "Installing PacSeek package browser...",
                ));
            }

            if selected_ids.contains(&"bauh".to_string()) {
                commands.push(progress_dialog::CommandStep::aur(
                    &["-S", "--noconfirm", "--needed", "bauh"],
                    "Installing Bauh package manager...",
                ));
            }

            if selected_ids.contains(&"warehouse".to_string()) {
                commands.push(progress_dialog::CommandStep::normal(
                    "flatpak",
                    &["install", "-y", "io.github.flattool.Warehouse"],
                    "Installing Warehouse from Flathub...",
                ));
            }

            if selected_ids.contains(&"flatseal".to_string()) {
                commands.push(progress_dialog::CommandStep::normal(
                    "flatpak",
                    &["install", "-y", "com.github.tchx84.Flatseal"],
                    "Installing Flatseal from Flathub...",
                ));
            }

            if selected_ids.contains(&"bazaar".to_string()) {
                commands.push(progress_dialog::CommandStep::normal(
                    "flatpak",
                    &["install", "-y", "io.github.kolunmi.Bazaar"],
                    "Installing Bazaar from Flathub...",
                ));
            }

            if !commands.is_empty() {
                let window_ref = window_clone.upcast_ref::<gtk4::Window>();
                progress_dialog::run_commands_with_progress(
                    window_ref,
                    commands,
                    "Package Manager GUI Installation",
                    None,
                );
            }
        });
    }
}
