//! Customization page button handlers.
//!
//! Handles:
//! - ZSH All-in-One setup
//! - Save Desktop tool
//! - GRUB theme installation
//! - Plasma wallpapers
//! - Layan GTK4 patch

use crate::ui::command_execution as progress_dialog;
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Builder};
use log::{info};

/// Set up all button handlers for the customization page
pub fn setup_handlers(page_builder: &Builder, _main_builder: &Builder) {
    setup_zsh_aio(&page_builder);
    setup_save_desktop(&page_builder);
    setup_grub_theme(&page_builder);
    setup_wallpapers(&page_builder);
    setup_layan_patch(&page_builder);
}

fn setup_zsh_aio(page_builder: &Builder) {
    if let Some(btn_zsh_aio) = page_builder.object::<gtk4::Button>("btn_zsh_aio") {
        btn_zsh_aio.connect_clicked(move |button| {
            info!("Customization: ZSH AiO button clicked");            let widget = button.clone().upcast::<gtk4::Widget>();
            let window = widget
                .root()
                .and_then(|root| root.downcast::<ApplicationWindow>().ok());

            if let Some(window) = window {
                let mut commands = vec![];
                commands.push(progress_dialog::CommandStep::aur(
                    &["-S", "--needed", "--noconfirm", "zsh", "grml-zsh-config", "fastfetch"],
                    "Installing ZSH and dependencies...",
                ));
                commands.push(progress_dialog::CommandStep::privileged(
                    "sh",
                    &["-c", "sh -c \"$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)\" \"\" --unattended"],
                    "Installing Oh My Zsh framework...",
                ));
                commands.push(progress_dialog::CommandStep::aur(
                    &["-S", "--noconfirm", "--needed", "pacseek", "ttf-meslo-nerd", "siji-git",
                      "otf-unifont", "bdf-unifont", "noto-color-emoji-fontconfig", "xorg-fonts-misc",
                      "ttf-dejavu", "ttf-meslo-nerd-font-powerlevel10k", "noto-fonts-emoji",
                      "powerline-fonts", "oh-my-posh-bin"],
                    "Installing fonts and terminal enhancements...",
                ));

                let home = std::env::var("HOME").unwrap_or_default();
                commands.push(progress_dialog::CommandStep::normal(
                    "git",
                    &["clone", "https://github.com/zsh-users/zsh-completions",
                      &format!("{}/.oh-my-zsh/custom/plugins/zsh-completions", home)],
                    "Installing ZSH completions plugin...",
                ));
                commands.push(progress_dialog::CommandStep::normal(
                    "git",
                    &["clone", "https://github.com/zsh-users/zsh-autosuggestions",
                      &format!("{}/.oh-my-zsh/custom/plugins/zsh-autosuggestions", home)],
                    "Installing ZSH autosuggestions plugin...",
                ));
                commands.push(progress_dialog::CommandStep::normal(
                    "git",
                    &["clone", "https://github.com/zsh-users/zsh-syntax-highlighting.git",
                      &format!("{}/.oh-my-zsh/custom/plugins/zsh-syntax-highlighting", home)],
                    "Installing ZSH syntax highlighting plugin...",
                ));
                commands.push(progress_dialog::CommandStep::normal(
                    "sh",
                    &["-c", &format!("mv -f {}/.zshrc {}/.zshrc.user 2>/dev/null || true", home, home)],
                    "Backing up existing ZSH configuration...",
                ));
                commands.push(progress_dialog::CommandStep::normal(
                    "wget",
                    &["-q", "-P", &home, "https://raw.githubusercontent.com/xerolinux/xero-fixes/main/conf/.zshrc"],
                    "Downloading XeroLinux ZSH configuration...",
                ));
                commands.push(progress_dialog::CommandStep::privileged(
                    "chsh",
                    &[&std::env::var("USER").unwrap_or_default(), "-s", "/bin/zsh"],
                    "Setting ZSH as default shell...",
                ));

                let window_ref = window.upcast_ref::<gtk4::Window>();
                progress_dialog::run_commands_with_progress(
                    window_ref,
                    commands,
                    "ZSH All-in-One Setup",
                    None,
                );
            }
        });
    }
}

fn setup_save_desktop(page_builder: &Builder) {
    if let Some(btn_save_desktop) = page_builder.object::<gtk4::Button>("btn_save_desktop") {
        btn_save_desktop.connect_clicked(move |button| {
            info!("Customization: Save Desktop Tool button clicked");
            let widget = button.clone().upcast::<gtk4::Widget>();
            let window = widget
                .root()
                .and_then(|root| root.downcast::<ApplicationWindow>().ok());

            if let Some(window) = window {
                let commands = vec![progress_dialog::CommandStep::normal(
                    "flatpak",
                    &["install", "-y", "io.github.vikdevelop.SaveDesktop"],
                    "Installing Save Desktop tool from Flathub...",
                )];

                let window_ref = window.upcast_ref::<gtk4::Window>();
                progress_dialog::run_commands_with_progress(
                    window_ref,
                    commands,
                    "Save Desktop Tool Installation",
                    None,
                );
            }
        });
    }
}

fn setup_grub_theme(page_builder: &Builder) {
    if let Some(btn_grub_theme) = page_builder.object::<gtk4::Button>("btn_grub_theme") {
        btn_grub_theme.connect_clicked(move |button| {
            info!("Customization: GRUB Theme button clicked");
            let widget = button.clone().upcast::<gtk4::Widget>();
            let window = widget
                .root()
                .and_then(|root| root.downcast::<ApplicationWindow>().ok());

            if let Some(window) = window {
                let home = std::env::var("HOME").unwrap_or_default();
                let commands = vec![
                    progress_dialog::CommandStep::normal(
                        "git",
                        &[
                            "clone",
                            "--depth",
                            "1",
                            "https://github.com/xerolinux/xero-grubs",
                            &format!("{}/xero-grubs", home),
                        ],
                        "Downloading GRUB theme repository...",
                    ),
                    progress_dialog::CommandStep::privileged(
                        "sh",
                        &["-c", &format!("cd {}/xero-grubs && ./install.sh", home)],
                        "Installing GRUB theme...",
                    ),
                    progress_dialog::CommandStep::normal(
                        "rm",
                        &["-rf", &format!("{}/xero-grubs", home)],
                        "Cleaning up temporary files...",
                    ),
                ];

                let window_ref = window.upcast_ref::<gtk4::Window>();
                progress_dialog::run_commands_with_progress(
                    window_ref,
                    commands,
                    "XeroLinux GRUB Theme Installation",
                    None,
                );
            }
        });
    }
}

fn setup_wallpapers(page_builder: &Builder) {
    if let Some(btn_wallpapers) = page_builder.object::<gtk4::Button>("btn_wallpapers") {
        btn_wallpapers.connect_clicked(move |button| {
            info!("Customization: Plasma Wallpapers button clicked");
            let widget = button.clone().upcast::<gtk4::Widget>();
            let window = widget
                .root()
                .and_then(|root| root.downcast::<ApplicationWindow>().ok());

            if let Some(window) = window {
                let commands = vec![progress_dialog::CommandStep::aur(
                    &["-S", "--noconfirm", "--needed", "kde-wallpapers-extra"],
                    "Installing KDE wallpapers collection (~1.2GB)...",
                )];

                let window_ref = window.upcast_ref::<gtk4::Window>();
                progress_dialog::run_commands_with_progress(
                    window_ref,
                    commands,
                    "Plasma Wallpapers Installation (~1.2GB)",
                    None,
                );
            }
        });
    }
}

fn setup_layan_patch(page_builder: &Builder) {
    if let Some(btn_layan_patch) = page_builder.object::<gtk4::Button>("btn_layan_patch") {
        btn_layan_patch.connect_clicked(move |button| {
            info!("Customization: Layan GTK4 Patch button clicked");
            let widget = button.clone().upcast::<gtk4::Widget>();
            let window = widget
                .root()
                .and_then(|root| root.downcast::<ApplicationWindow>().ok());

            if let Some(window) = window {
                let home = std::env::var("HOME").unwrap_or_default();
                let commands = vec![
                    progress_dialog::CommandStep::normal(
                        "git",
                        &[
                            "clone",
                            "--depth",
                            "1",
                            "https://github.com/vinceliuice/Layan-gtk-theme.git",
                            &format!("{}/Layan-gtk-theme", home),
                        ],
                        "Downloading Layan GTK theme...",
                    ),
                    progress_dialog::CommandStep::privileged(
                        "sh",
                        &[
                            "-c",
                            &format!(
                                "cd {}/Layan-gtk-theme && sh install.sh -l -c dark -d {}/.themes",
                                home, home
                            ),
                        ],
                        "Installing Layan GTK theme...",
                    ),
                    progress_dialog::CommandStep::normal(
                        "rm",
                        &["-rf", &format!("{}/Layan-gtk-theme", home)],
                        "Cleaning up GTK theme files...",
                    ),
                    progress_dialog::CommandStep::normal(
                        "git",
                        &[
                            "clone",
                            "--depth",
                            "1",
                            "https://github.com/vinceliuice/Layan-kde.git",
                            &format!("{}/Layan-kde", home),
                        ],
                        "Downloading Layan KDE theme...",
                    ),
                    progress_dialog::CommandStep::privileged(
                        "sh",
                        &["-c", &format!("cd {}/Layan-kde && sh install.sh", home)],
                        "Installing Layan KDE theme...",
                    ),
                    progress_dialog::CommandStep::normal(
                        "rm",
                        &["-rf", &format!("{}/Layan-kde", home)],
                        "Cleaning up KDE theme files...",
                    ),
                ];

                let window_ref = window.upcast_ref::<gtk4::Window>();
                progress_dialog::run_commands_with_progress(
                    window_ref,
                    commands,
                    "Layan GTK4 Patch & Update",
                    None,
                );
            }
        });
    }
}
