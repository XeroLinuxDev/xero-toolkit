//! Gaming tools page button handlers.
//!
//! Handles:
//! - Steam AiO installation
//! - Game controller drivers
//! - Gamescope configuration
//! - LACT GPU overclocking
//! - Game launchers (Lutris, Heroic, Bottles)

use crate::core;
use crate::ui::dialogs::selection::{
    show_selection_dialog, SelectionDialogConfig, SelectionOption,
};
use crate::ui::task_runner::{self, Command};
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Builder, Button};
use log::info;

/// Set up all button handlers for the gaming tools page.
pub fn setup_handlers(page_builder: &Builder, _main_builder: &Builder) {
    setup_steam_aio(page_builder);
    setup_controllers(page_builder);
    setup_gamescope_cfg(page_builder);
    setup_lact_oc(page_builder);
    setup_lutris(page_builder);
    setup_heroic(page_builder);
    setup_bottles(page_builder);
}

fn setup_steam_aio(builder: &Builder) {
    let Some(button) = builder.object::<Button>("btn_steam_aio") else {
        return;
    };

    button.connect_clicked(move |btn| {
        info!("Steam AiO button clicked");

        let Some(window) = get_window(btn) else {
            return;
        };

        let commands = vec![Command::aur(
            &[
                "-S",
                "--noconfirm",
                "--needed",
                "steam",
                "lib32-pipewire-jack",
                "gamemode",
                "gamescope",
                "mangohud",
                "mangoverlay",
                "lib32-mangohud",
                "wine-meta",
                "wine-nine",
                "ttf-liberation",
                "lib32-fontconfig",
                "wqy-zenhei",
                "vkd3d",
                "giflib",
                "lib32-giflib",
                "libpng",
                "lib32-libpng",
                "libldap",
                "lib32-libldap",
                "gnutls",
                "lib32-gnutls",
                "mpg123",
                "lib32-mpg123",
                "openal",
                "lib32-openal",
                "v4l-utils",
                "lib32-v4l-utils",
                "libpulse",
                "lib32-libpulse",
                "libgpg-error",
                "lib32-libgpg-error",
                "alsa-plugins",
                "lib32-alsa-plugins",
                "alsa-lib",
                "lib32-alsa-lib",
                "libjpeg-turbo",
                "lib32-libjpeg-turbo",
                "sqlite",
                "lib32-sqlite",
                "libxcomposite",
                "lib32-libxcomposite",
                "libxinerama",
                "lib32-libgcrypt",
                "libgcrypt",
                "lib32-libxinerama",
                "ncurses",
                "lib32-ncurses",
                "ocl-icd",
                "lib32-ocl-icd",
                "libxslt",
                "lib32-libxslt",
                "libva",
                "lib32-libva",
                "gtk3",
                "lib32-gtk3",
                "gst-plugins-base-libs",
                "lib32-gst-plugins-base-libs",
                "vulkan-icd-loader",
                "lib32-vulkan-icd-loader",
                "cups",
                "dosbox",
                "lib32-opencl-icd-loader",
                "lib32-vkd3d",
                "opencl-icd-loader",
            ],
            "Installing Steam and gaming dependencies...",
        )];

        task_runner::run(
            window.upcast_ref(),
            commands,
            "Steam AiO Installation",
            None,
        );
    });
}

fn setup_controllers(builder: &Builder) {
    let Some(button) = builder.object::<Button>("btn_controllers") else {
        return;
    };

    button.connect_clicked(move |btn| {
        info!("Controllers button clicked");

        let Some(window) = get_window(btn) else {
            return;
        };

        let window_clone = window.clone();

        let config = SelectionDialogConfig::new(
            "Game Controller Drivers",
            "Select which controller drivers to install.",
        )
        .add_option(SelectionOption::new(
            "dualsense",
            "DualSense Controller",
            "PlayStation 5 DualSense controller driver",
            core::is_package_installed("dualsensectl"),
        ))
        .add_option(SelectionOption::new(
            "dualshock4",
            "DualShock 4 Controller",
            "PlayStation 4 DualShock 4 controller driver",
            core::is_package_installed("ds4drv"),
        ))
        .add_option(SelectionOption::new(
            "xboxone",
            "Xbox One Controller",
            "Xbox One wireless controller driver",
            core::is_package_installed("xone-dkms"),
        ))
        .confirm_label("Install");

        show_selection_dialog(window.upcast_ref(), config, move |selected| {
            let commands = build_controller_commands(&selected);

            if !commands.is_empty() {
                task_runner::run(
                    window_clone.upcast_ref(),
                    commands,
                    "Controller Driver Installation",
                    None,
                );
            }
        });
    });
}

/// Build commands for selected controller drivers.
fn build_controller_commands(selected: &[String]) -> Vec<Command> {
    let mut commands = Vec::new();

    for id in selected {
        match id.as_str() {
            "dualsense" => commands.push(Command::aur(
                &[
                    "-S",
                    "--noconfirm",
                    "--needed",
                    "dualsensectl",
                    "game-devices-udev",
                ],
                "Installing DualSense driver...",
            )),
            "dualshock4" => commands.push(Command::aur(
                &[
                    "-S",
                    "--noconfirm",
                    "--needed",
                    "ds4drv",
                    "game-devices-udev",
                ],
                "Installing DualShock 4 driver...",
            )),
            "xboxone" => commands.push(Command::aur(
                &[
                    "-S",
                    "--noconfirm",
                    "--needed",
                    "xone-dkms",
                    "game-devices-udev",
                ],
                "Installing Xbox One controller driver...",
            )),
            _ => {}
        }
    }

    commands
}

fn setup_gamescope_cfg(builder: &Builder) {
    let Some(button) = builder.object::<Button>("btn_gamescope_cfg") else {
        return;
    };

    button.connect_clicked(|_| {
        info!("Gamescope CFG button clicked");
        let _ = std::process::Command::new("xdg-open")
            .arg("https://sidewalksndskeletons.github.io/gamescope-gui/")
            .spawn();
    });
}

fn setup_lact_oc(builder: &Builder) {
    let Some(button) = builder.object::<Button>("btn_lact_oc") else {
        return;
    };

    button.connect_clicked(move |btn| {
        info!("LACT OC button clicked");

        let Some(window) = get_window(btn) else {
            return;
        };

        let commands = vec![
            Command::aur(
                &["-S", "--noconfirm", "--needed", "lact"],
                "Installing LACT GPU control utility...",
            ),
            Command::privileged(
                "systemctl",
                &["enable", "--now", "lactd"],
                "Enabling LACT background service...",
            ),
        ];

        task_runner::run(window.upcast_ref(), commands, "LACT GPU Tools", None);
    });
}

fn setup_lutris(builder: &Builder) {
    let Some(button) = builder.object::<Button>("btn_lutris") else {
        return;
    };

    button.connect_clicked(move |btn| {
        info!("Lutris button clicked");

        let Some(window) = get_window(btn) else {
            return;
        };

        let commands = vec![Command::normal(
            "flatpak",
            &[
                "install",
                "-y",
                "net.lutris.Lutris",
                "org.freedesktop.Platform.VulkanLayer.gamescope/x86_64/25.08",
                "org.freedesktop.Platform.VulkanLayer.MangoHud/x86_64/25.08",
            ],
            "Installing Lutris and Vulkan layers...",
        )];

        task_runner::run(window.upcast_ref(), commands, "Lutris Installation", None);
    });
}

fn setup_heroic(builder: &Builder) {
    let Some(button) = builder.object::<Button>("btn_heroic") else {
        return;
    };

    button.connect_clicked(move |btn| {
        info!("Heroic button clicked");

        let Some(window) = get_window(btn) else {
            return;
        };

        let commands = vec![Command::normal(
            "flatpak",
            &[
                "install",
                "-y",
                "com.heroicgameslauncher.hgl",
                "org.freedesktop.Platform.VulkanLayer.gamescope/x86_64/25.08",
                "org.freedesktop.Platform.VulkanLayer.MangoHud/x86_64/25.08",
            ],
            "Installing Heroic Games Launcher...",
        )];

        task_runner::run(
            window.upcast_ref(),
            commands,
            "Heroic Launcher Installation",
            None,
        );
    });
}

fn setup_bottles(builder: &Builder) {
    let Some(button) = builder.object::<Button>("btn_bottles") else {
        return;
    };

    button.connect_clicked(move |btn| {
        info!("Bottles button clicked");

        let Some(window) = get_window(btn) else {
            return;
        };

        let commands = vec![Command::normal(
            "flatpak",
            &[
                "install",
                "-y",
                "com.usebottles.bottles",
                "org.freedesktop.Platform.VulkanLayer.gamescope/x86_64/25.08",
                "org.freedesktop.Platform.VulkanLayer.MangoHud/x86_64/25.08",
            ],
            "Installing Bottles and Vulkan layers...",
        )];

        task_runner::run(window.upcast_ref(), commands, "Bottles Installation", None);
    });
}

/// Helper to get the parent window from a button.
fn get_window(button: &Button) -> Option<ApplicationWindow> {
    button
        .root()
        .and_then(|root| root.downcast::<ApplicationWindow>().ok())
}
