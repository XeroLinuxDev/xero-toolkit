//! Gaming tools page button handlers.
//!
//! Handles:
//! - Steam AiO installation
//! - Game controller drivers
//! - Gamescope configuration
//! - LACT GPU overclocking
//! - Game launchers (Lutris, Heroic, Bottles)

use crate::core;
use crate::ui::command_execution as progress_dialog;
use crate::ui::selection_dialog;
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Builder};
use log::{info};

/// Set up all button handlers for the gaming tools page
pub fn setup_handlers(page_builder: &Builder, _main_builder: &Builder) {
    setup_steam_aio(page_builder);
    setup_controllers(page_builder);
    setup_gamescope_cfg(page_builder);
    setup_lact_oc(page_builder);
    setup_lutris(page_builder);
    setup_heroic(page_builder);
    setup_bottles(page_builder);
}

fn setup_steam_aio(page_builder: &Builder) {
    if let Some(btn_steam_aio) = page_builder.object::<gtk4::Button>("btn_steam_aio") {
        btn_steam_aio.connect_clicked(move |button| {
            info!("Gaming tools: Steam AiO button clicked");
            let commands = vec![progress_dialog::CommandStep::aur(
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

            let widget = button.clone().upcast::<gtk4::Widget>();
            if let Some(window) = widget
                .root()
                .and_then(|r| r.downcast::<ApplicationWindow>().ok())
            {
                let window_ref = window.upcast_ref::<gtk4::Window>();
                progress_dialog::run_commands_with_progress(
                    window_ref,
                    commands,
                    "Steam AiO Installation",
                    None,
                );
            }
        });
    }
}

fn setup_controllers(page_builder: &Builder) {
    if let Some(btn_controllers) = page_builder.object::<gtk4::Button>("btn_controllers") {
        btn_controllers.connect_clicked(move |button| {
            info!("Gaming tools: Controllers button clicked");
            let dualsense_installed = core::is_package_installed("dualsensectl");
            let dualshock4_installed = core::is_package_installed("ds4drv");
            let xboxone_installed = core::is_package_installed("xone-dkms");

            let widget = button.clone().upcast::<gtk4::Widget>();
            let window = widget
                .root()
                .and_then(|r| r.downcast::<ApplicationWindow>().ok());

            if let Some(window) = window {
                let window_clone = window.clone();
                let window_ref = window.upcast_ref::<gtk4::Window>();
                let config = selection_dialog::SelectionDialogConfig::new(
                    "Game Controller Drivers",
                    "Select which controller drivers to install.",
                )
                .add_option(selection_dialog::SelectionOption::new(
                    "dualsense",
                    "DualSense Controller",
                    "PlayStation 5 DualSense controller driver",
                    dualsense_installed,
                ))
                .add_option(selection_dialog::SelectionOption::new(
                    "dualshock4",
                    "DualShock 4 Controller",
                    "PlayStation 4 DualShock 4 controller driver",
                    dualshock4_installed,
                ))
                .add_option(selection_dialog::SelectionOption::new(
                    "xboxone",
                    "Xbox One Controller",
                    "Xbox One wireless controller driver",
                    xboxone_installed,
                ))
                .confirm_label("Install");

                selection_dialog::show_selection_dialog(window_ref, config, move |selected_ids| {
                    let mut commands = Vec::new();
                    for id in selected_ids {
                        match id.as_str() {
                            "dualsense" => commands.push(progress_dialog::CommandStep::aur(
                                &[
                                    "-S",
                                    "--noconfirm",
                                    "--needed",
                                    "dualsensectl",
                                    "game-devices-udev",
                                ],
                                "Installing DualSense driver...",
                            )),
                            "dualshock4" => commands.push(progress_dialog::CommandStep::aur(
                                &[
                                    "-S",
                                    "--noconfirm",
                                    "--needed",
                                    "ds4drv",
                                    "game-devices-udev",
                                ],
                                "Installing DualShock 4 driver...",
                            )),
                            "xboxone" => commands.push(progress_dialog::CommandStep::aur(
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
                    if !commands.is_empty() {
                        let window_ref2 = window_clone.upcast_ref::<gtk4::Window>();
                        progress_dialog::run_commands_with_progress(
                            window_ref2,
                            commands,
                            "Controller Driver Installation",
                            None,
                        );
                    }
                });
            }
        });
    }
}

fn setup_gamescope_cfg(page_builder: &Builder) {
    if let Some(btn_gamescope_cfg) = page_builder.object::<gtk4::Button>("btn_gamescope_cfg") {
        btn_gamescope_cfg.connect_clicked(move |_| {
            info!("Gaming tools: Gamescope CFG button clicked - opening gamescope-gui");
            let _ = std::process::Command::new("xdg-open")
                .arg("https://sidewalksndskeletons.github.io/gamescope-gui/")
                .spawn();
        });
    }
}

fn setup_lact_oc(page_builder: &Builder) {
    if let Some(btn_lact_oc) = page_builder.object::<gtk4::Button>("btn_lact_oc") {
        btn_lact_oc.connect_clicked(move |button| {
            info!("Gaming tools: LACT OC button clicked");
            let commands = vec![
                progress_dialog::CommandStep::aur(
                    &["-S", "--noconfirm", "--needed", "lact"],
                    "Installing LACT GPU control utility...",
                ),
                progress_dialog::CommandStep::privileged(
                    "systemctl",
                    &["enable", "--now", "lactd"],
                    "Enabling LACT background service...",
                ),
            ];

            let widget = button.clone().upcast::<gtk4::Widget>();
            if let Some(window) = widget
                .root()
                .and_then(|r| r.downcast::<ApplicationWindow>().ok())
            {
                let window_ref = window.upcast_ref::<gtk4::Window>();
                progress_dialog::run_commands_with_progress(
                    window_ref,
                    commands,
                    "LACT GPU Tools",
                    None,
                );
            }
        });
    }
}

fn setup_lutris(page_builder: &Builder) {
    if let Some(btn_lutris) = page_builder.object::<gtk4::Button>("btn_lutris") {
        btn_lutris.connect_clicked(move |button| {
            info!("Gaming tools: Lutris button clicked");
            let commands = vec![progress_dialog::CommandStep::normal(
                "flatpak",
                &[
                    "install",
                    "-y",
                    "net.lutris.Lutris",
                    "org.freedesktop.Platform.VulkanLayer.gamescope/x86_64/24.08",
                    "org.freedesktop.Platform.VulkanLayer.MangoHud",
                ],
                "Installing Lutris and Vulkan layers...",
            )];

            let widget = button.clone().upcast::<gtk4::Widget>();
            if let Some(window) = widget
                .root()
                .and_then(|r| r.downcast::<ApplicationWindow>().ok())
            {
                let window_ref = window.upcast_ref::<gtk4::Window>();
                progress_dialog::run_commands_with_progress(
                    window_ref,
                    commands,
                    "Lutris Installation",
                    None,
                );
            }
        });
    }
}

fn setup_heroic(page_builder: &Builder) {
    if let Some(btn_heroic) = page_builder.object::<gtk4::Button>("btn_heroic") {
        btn_heroic.connect_clicked(move |button| {
            info!("Gaming tools: Heroic button clicked");
            let commands = vec![progress_dialog::CommandStep::normal(
                "flatpak",
                &[
                    "install",
                    "-y",
                    "com.heroicgameslauncher.hgl",
                    "org.freedesktop.Platform.VulkanLayer.gamescope/x86_64/24.08",
                    "org.freedesktop.Platform.VulkanLayer.MangoHud",
                ],
                "Installing Heroic Games Launcher...",
            )];

            let widget = button.clone().upcast::<gtk4::Widget>();
            if let Some(window) = widget
                .root()
                .and_then(|r| r.downcast::<ApplicationWindow>().ok())
            {
                let window_ref = window.upcast_ref::<gtk4::Window>();
                progress_dialog::run_commands_with_progress(
                    window_ref,
                    commands,
                    "Heroic Launcher Installation",
                    None,
                );
            }
        });
    }
}

fn setup_bottles(page_builder: &Builder) {
    if let Some(btn_bottles) = page_builder.object::<gtk4::Button>("btn_bottles") {
        btn_bottles.connect_clicked(move |button| {
            info!("Gaming tools: Bottles button clicked");
            let commands = vec![progress_dialog::CommandStep::normal(
                "flatpak",
                &[
                    "install",
                    "-y",
                    "com.usebottles.bottles",
                    "org.freedesktop.Platform.VulkanLayer.gamescope",
                    "org.freedesktop.Platform.VulkanLayer.MangoHud",
                ],
                "Installing Bottles and Vulkan layers...",
            )];

            let widget = button.clone().upcast::<gtk4::Widget>();
            if let Some(window) = widget
                .root()
                .and_then(|r| r.downcast::<ApplicationWindow>().ok())
            {
                let window_ref = window.upcast_ref::<gtk4::Window>();
                progress_dialog::run_commands_with_progress(
                    window_ref,
                    commands,
                    "Bottles Installation",
                    None,
                );
            }
        });
    }
}
