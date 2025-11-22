//! Servicing and system tweaks page button handlers.
//!
//! Handles:
//! - Clear Pacman cache
//! - Unlock Pacman database
//! - Plasma X11 session installation
//! - VM guest utilities
//! - WayDroid guide
//! - Fix GPGME database
//! - Fix Arch keyring
//! - Update mirrorlist
//! - Parallel downloads adjustment

use crate::core;
use crate::ui::command_execution as progress_dialog;
use crate::ui::selection_dialog;
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Builder};
use log::{info};

/// Set up all button handlers for the servicing/system tweaks page
pub fn setup_handlers(page_builder: &Builder, _main_builder: &Builder) {
    setup_clr_pacman(page_builder);
    setup_unlock_pacman(page_builder);
    setup_plasma_x11(page_builder);
    setup_vm_guest_utils(page_builder);
    setup_waydroid_guide(page_builder);
    setup_fix_gpgme(page_builder);
    setup_fix_arch_keyring(page_builder);
    setup_update_mirrorlist(page_builder);
    setup_parallel_downloads(page_builder);
}

fn setup_clr_pacman(page_builder: &Builder) {
    if let Some(btn_clr_pacman) = page_builder.object::<gtk4::Button>("btn_clr_pacman") {
        btn_clr_pacman.connect_clicked(move |button| {
            info!("Servicing: Clear Pacman Cache button clicked");
            let commands = vec![progress_dialog::CommandStep::privileged(
                "pacman",
                &["-Scc"],
                "Clearing Pacman cache (full clean)...",
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
                    "Clear Pacman Cache",
                    None,
                );
            }
        });
    }
}

fn setup_unlock_pacman(page_builder: &Builder) {
    if let Some(btn_unlock_pacman) = page_builder.object::<gtk4::Button>("btn_unlock_pacman") {
        btn_unlock_pacman.connect_clicked(move |button| {
            info!("Servicing: Unlock Pacman DB button clicked");
            let commands = vec![progress_dialog::CommandStep::privileged(
                "rm",
                &["-f", "/var/lib/pacman/db.lck"],
                "Removing Pacman lock file...",
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
                    "Unlock Pacman Database",
                    None,
                );
            }
        });
    }
}

fn setup_plasma_x11(page_builder: &Builder) {
    if let Some(btn_plasma_x11) = page_builder.object::<gtk4::Button>("btn_plasma_x11") {
        btn_plasma_x11.connect_clicked(move |button| {
            info!("Servicing: Plasma X11 Session button clicked");
            let commands = vec![progress_dialog::CommandStep::aur(
                &["-S", "--noconfirm", "kwin-x11", "plasma-x11-session"],
                "Installing KDE Plasma X11 session components...",
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
                    "Install KDE X11 Session",
                    None,
                );
            }
        });
    }
}

fn setup_vm_guest_utils(page_builder: &Builder) {
    if let Some(btn_vm_guest_utils) = page_builder.object::<gtk4::Button>("btn_vm_guest_utils") {
        btn_vm_guest_utils.connect_clicked(move |button| {
            info!("Servicing: VM Guest Utils button clicked");
            let output = std::process::Command::new("systemd-detect-virt").output();
            let mut commands: Vec<progress_dialog::CommandStep> = vec![];
            match output {
                Ok(result) if result.status.success() => {
                    let virt = String::from_utf8_lossy(&result.stdout).trim().to_string();
                    match virt.as_str() {
                        "oracle" => commands.push(progress_dialog::CommandStep::aur(
                            &["-S", "--needed", "--noconfirm", "virtualbox-guest-utils"],
                            "Installing VirtualBox guest utilities...",
                        )),
                        "kvm" => commands.push(progress_dialog::CommandStep::aur(
                            &[
                                "-S",
                                "--needed",
                                "--noconfirm",
                                "qemu-guest-agent",
                                "spice-vdagent",
                            ],
                            "Installing KVM/QEMU guest agents...",
                        )),
                        _ => {
                            let widget = button.clone().upcast::<gtk4::Widget>();
                            if let Some(window) = widget
                                .root()
                                .and_then(|r| r.downcast::<ApplicationWindow>().ok())
                            {
                                let dialog = gtk4::MessageDialog::builder()
                                    .transient_for(&window)
                                    .modal(true)
                                    .message_type(gtk4::MessageType::Error)
                                    .buttons(gtk4::ButtonsType::Ok)
                                    .text("Error")
                                    .secondary_text("Unsupported or no virtualization detected.")
                                    .build();
                                dialog.connect_response(|d, _| d.close());
                                dialog.present();
                            }
                            return;
                        }
                    }
                }
                _ => {
                    let widget = button.clone().upcast::<gtk4::Widget>();
                    if let Some(window) = widget
                        .root()
                        .and_then(|r| r.downcast::<ApplicationWindow>().ok())
                    {
                        let dialog = gtk4::MessageDialog::builder()
                            .transient_for(&window)
                            .modal(true)
                            .message_type(gtk4::MessageType::Error)
                            .buttons(gtk4::ButtonsType::Ok)
                            .text("Error")
                            .secondary_text("Failed to detect virtualization environment.")
                            .build();
                        dialog.connect_response(|d, _| d.close());
                        dialog.present();
                    }
                    return;
                }
            }

            if !commands.is_empty() {
                let widget = button.clone().upcast::<gtk4::Widget>();
                if let Some(window) = widget
                    .root()
                    .and_then(|r| r.downcast::<ApplicationWindow>().ok())
                {
                    let window_ref = window.upcast_ref::<gtk4::Window>();
                    progress_dialog::run_commands_with_progress(
                        window_ref,
                        commands,
                        "Install VM Guest Utilities",
                        None,
                    );
                }
            }
        });
    }
}

fn setup_waydroid_guide(page_builder: &Builder) {
    if let Some(btn_waydroid_guide) = page_builder.object::<gtk4::Button>("btn_waydroid_guide") {
        btn_waydroid_guide.connect_clicked(move |_| {
            info!("Servicing: WayDroid Guide button clicked - opening guide");
            let _ = std::process::Command::new("xdg-open")
                .arg("https://xerolinux.xyz/posts/waydroid-guide/")
                .spawn();
        });
    }
}

fn setup_fix_gpgme(page_builder: &Builder) {
    if let Some(btn_fix_gpgme) = page_builder.object::<gtk4::Button>("btn_fix_gpgme") {
        btn_fix_gpgme.connect_clicked(move |button| {
            info!("Servicing: Fix GPGME Database button clicked");
            let commands = vec![
                progress_dialog::CommandStep::privileged(
                    "rm",
                    &["-rf", "/var/lib/pacman/sync"],
                    "Removing sync database...",
                ),
                progress_dialog::CommandStep::privileged(
                    "pacman",
                    &["-Syy"],
                    "Refreshing package databases...",
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
                    "Fix GPGME Database Issue",
                    None,
                );
            }
        });
    }
}

fn setup_fix_arch_keyring(page_builder: &Builder) {
    if let Some(btn_fix_arch_keyring) = page_builder.object::<gtk4::Button>("btn_fix_arch_keyring")
    {
        btn_fix_arch_keyring.connect_clicked(move |button| {
            info!("Servicing: Fix Arch Keyring button clicked");            let commands = vec![
                progress_dialog::CommandStep::privileged(
                    "rm",
                    &["-rf", "/etc/pacman.d/gnupg"],
                    "Removing existing GnuPG keyring...",
                ),
                progress_dialog::CommandStep::privileged(
                    "pacman-key",
                    &["--init"],
                    "Initializing new keyring...",
                ),
                progress_dialog::CommandStep::privileged(
                    "pacman-key",
                    &["--populate"],
                    "Populating keyring...",
                ),
                progress_dialog::CommandStep::privileged(
                    "sh",
                    &["-c", "echo 'keyserver hkp://keyserver.ubuntu.com:80' >> /etc/pacman.d/gnupg/gpg.conf"],
                    "Setting keyserver...",
                ),
                progress_dialog::CommandStep::privileged(
                    "pacman",
                    &["-Syy", "--noconfirm", "archlinux-keyring"],
                    "Reinstalling Arch Linux keyring...",
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
                    "Fix GnuPG Keyring",
                    None,
                );
            }
        });
    }
}

fn setup_update_mirrorlist(page_builder: &Builder) {
    if let Some(btn_update_mirrorlist) =
        page_builder.object::<gtk4::Button>("btn_update_mirrorlist")
    {
        btn_update_mirrorlist.connect_clicked(move |button| {
            info!("Servicing: Update Mirrorlist button clicked");            let widget = button.clone().upcast::<gtk4::Widget>();
            let window = widget.root().and_then(|r| r.downcast::<ApplicationWindow>().ok());
            if let Some(window) = window {
                let window_clone = window.clone();
                let window_ref = window.upcast_ref::<gtk4::Window>();

                let rate_mirrors_installed = core::is_package_installed("rate-mirrors");
                let config = selection_dialog::SelectionDialogConfig::new(
                    "Update Mirrorlist",
                    "Select which mirrorlists to update. rate-mirrors will be installed if needed.",
                )
                .add_option(selection_dialog::SelectionOption::new(
                    "chaotic",
                    "Chaotic-AUR Mirrorlist",
                    "Also update Chaotic-AUR mirrorlist (optional)",
                    false,
                ))
                .confirm_label("Update");

                selection_dialog::show_selection_dialog(window_ref, config, move |selected_ids| {
                    let mut commands: Vec<progress_dialog::CommandStep> = vec![];

                    if !rate_mirrors_installed {
                        commands.push(progress_dialog::CommandStep::aur(
                             &["-S", "--needed", "--noconfirm", "rate-mirrors"],
                             "Installing rate-mirrors utility...",
                         ));
                    }

                    commands.push(progress_dialog::CommandStep::privileged(
                        "sh",
                        &["-c", "rate-mirrors --allow-root --protocol https arch | tee /etc/pacman.d/mirrorlist"],
                        "Updating Arch mirrorlist...",
                    ));

                    if selected_ids.contains(&"chaotic".to_string()) {
                        commands.push(progress_dialog::CommandStep::privileged(
                            "sh",
                            &["-c", "rate-mirrors --allow-root --protocol https chaotic-aur | tee /etc/pacman.d/chaotic-mirrorlist"],
                            "Updating Chaotic-AUR mirrorlist...",
                        ));
                    }

                    commands.push(progress_dialog::CommandStep::privileged(
                        "pacman",
                        &["-Syy"],
                        "Refreshing package databases...",
                    ));

                    if !commands.is_empty() {
                        let window_ref2 = window_clone.upcast_ref::<gtk4::Window>();
                        progress_dialog::run_commands_with_progress(window_ref2, commands, "Update System Mirrorlist", None);
                    }
                });
            }
        });
    }
}

fn setup_parallel_downloads(page_builder: &Builder) {
    if let Some(btn_parallel_downloads) =
        page_builder.object::<gtk4::Button>("btn_parallel_downloads")
    {
        btn_parallel_downloads.connect_clicked(move |button| {
            info!("Servicing: Change Parallel Downloads button clicked");
            let commands = vec![progress_dialog::CommandStep::privileged(
                "pmpd",
                &[],
                "Adjusting parallel downloads setting...",
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
                    "Change Parallel Downloads",
                    None,
                );
            }
        });
    }
}
