//! Containers and VMs page button handlers.
//!
//! Handles:
//! - Docker installation and setup
//! - Podman installation (with optional Desktop)
//! - VirtualBox installation
//! - DistroBox installation
//! - KVM/QEMU virtualization setup

use crate::core;
use crate::ui::command_execution as progress_dialog;
use crate::ui::selection_dialog;
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Builder};
use log::{info};

/// Set up all button handlers for the containers/VMs page
pub fn setup_handlers(page_builder: &Builder, _main_builder: &Builder) {
    setup_docker(page_builder);
    setup_podman(page_builder);
    setup_vbox(page_builder);
    setup_distrobox(page_builder);
    setup_kvm(page_builder);
}

fn setup_docker(page_builder: &Builder) {
    if let Some(btn_docker) = page_builder.object::<gtk4::Button>("btn_docker") {
        btn_docker.connect_clicked(move |button| {
            info!("Containers/VMs: Docker button clicked");
            let commands = vec![
                progress_dialog::CommandStep::aur(
                    &[
                        "-S",
                        "--noconfirm",
                        "--needed",
                        "docker",
                        "docker-compose",
                        "docker-buildx",
                    ],
                    "Installing Docker engine and tools...",
                ),
                progress_dialog::CommandStep::privileged(
                    "systemctl",
                    &["enable", "--now", "docker.service"],
                    "Enabling Docker service...",
                ),
                progress_dialog::CommandStep::privileged(
                    "groupadd",
                    &["-f", "docker"],
                    "Ensuring docker group exists...",
                ),
                progress_dialog::CommandStep::privileged(
                    "usermod",
                    &[
                        "-aG",
                        "docker",
                        &std::env::var("USER").unwrap_or_else(|_| "user".to_string()),
                    ],
                    "Adding your user to docker group...",
                ),
            ];

            // Friendly completion message via callback
            let widget = button.clone().upcast::<gtk4::Widget>();
            let window = widget
                .root()
                .and_then(|r| r.downcast::<ApplicationWindow>().ok());
            if let Some(window) = window {
                let window_ref = window.upcast_ref::<gtk4::Window>();
                progress_dialog::run_commands_with_progress(
                    window_ref,
                    commands,
                    "Docker Setup",
                    Some(Box::new(|success| {
                        if success {
                            info!("Docker setup completed");
                        }
                    })),
                );
            }
        });
    }
}

fn setup_podman(page_builder: &Builder) {
    if let Some(btn_podman) = page_builder.object::<gtk4::Button>("btn_podman") {
        btn_podman.connect_clicked(move |button| {
            info!("Containers/VMs: Podman button clicked");
            let widget = button.clone().upcast::<gtk4::Widget>();
            let window = widget
                .root()
                .and_then(|r| r.downcast::<ApplicationWindow>().ok());
            if let Some(window) = window {
                let window_clone = window.clone();
                let window_ref = window.upcast_ref::<gtk4::Window>();
                let config = selection_dialog::SelectionDialogConfig::new(
                    "Podman Installation",
                    "Podman will be installed. Optionally include the Podman Desktop GUI.",
                )
                .add_option(selection_dialog::SelectionOption::new(
                    "podman_desktop",
                    "Podman Desktop",
                    "Graphical interface for managing containers",
                    core::is_flatpak_installed("io.podman_desktop.PodmanDesktop"),
                ))
                .confirm_label("Install");

                selection_dialog::show_selection_dialog(window_ref, config, move |selected_ids| {
                    let mut commands = vec![
                        progress_dialog::CommandStep::aur(
                            &["-S", "--noconfirm", "--needed", "podman", "podman-docker"],
                            "Installing Podman container engine...",
                        ),
                        progress_dialog::CommandStep::privileged(
                            "systemctl",
                            &["enable", "--now", "podman.socket"],
                            "Enabling Podman socket...",
                        ),
                    ];
                    if selected_ids.contains(&"podman_desktop".to_string()) {
                        commands.push(progress_dialog::CommandStep::normal(
                            "flatpak",
                            &[
                                "install",
                                "-y",
                                "flathub",
                                "io.podman_desktop.PodmanDesktop",
                            ],
                            "Installing Podman Desktop GUI...",
                        ));
                    }
                    if !commands.is_empty() {
                        let window_ref2 = window_clone.upcast_ref::<gtk4::Window>();
                        progress_dialog::run_commands_with_progress(
                            window_ref2,
                            commands,
                            "Podman Setup",
                            None,
                        );
                    }
                });
            }
        });
    }
}

fn setup_vbox(page_builder: &Builder) {
    if let Some(btn_vbox) = page_builder.object::<gtk4::Button>("btn_vbox") {
        btn_vbox.connect_clicked(move |button| {
            info!("Containers/VMs: VirtualBox button clicked");
            let commands = vec![progress_dialog::CommandStep::aur(
                &["-S", "--noconfirm", "--needed", "virtualbox-meta"],
                "Installing VirtualBox...",
            )];

            let widget = button.clone().upcast::<gtk4::Widget>();
            let window = widget
                .root()
                .and_then(|r| r.downcast::<ApplicationWindow>().ok());
            if let Some(window) = window {
                let window_ref = window.upcast_ref::<gtk4::Window>();
                progress_dialog::run_commands_with_progress(
                    window_ref,
                    commands,
                    "VirtualBox Setup",
                    None,
                );
            }
        });
    }
}

fn setup_distrobox(page_builder: &Builder) {
    if let Some(btn_distrobox) = page_builder.object::<gtk4::Button>("btn_distrobox") {
        btn_distrobox.connect_clicked(move |button| {
            info!("Containers/VMs: DistroBox button clicked");
            let commands = vec![
                progress_dialog::CommandStep::aur(
                    &["-S", "--noconfirm", "--needed", "distrobox"],
                    "Installing DistroBox...",
                ),
                progress_dialog::CommandStep::normal(
                    "flatpak",
                    &["install", "-y", "io.github.dvlv.boxbuddyrs"],
                    "Installing BoxBuddy GUI...",
                ),
            ];

            let widget = button.clone().upcast::<gtk4::Widget>();
            let window = widget
                .root()
                .and_then(|r| r.downcast::<ApplicationWindow>().ok());
            if let Some(window) = window {
                let window_ref = window.upcast_ref::<gtk4::Window>();
                progress_dialog::run_commands_with_progress(
                    window_ref,
                    commands,
                    "DistroBox Setup",
                    None,
                );
            }
        });
    }
}

fn setup_kvm(page_builder: &Builder) {
    if let Some(btn_kvm) = page_builder.object::<gtk4::Button>("btn_kvm") {
        btn_kvm.connect_clicked(move |button| {
            info!("Containers/VMs: KVM button clicked");
            let mut commands: Vec<progress_dialog::CommandStep> = vec![];

            if core::is_package_installed("iptables") {
                commands.push(progress_dialog::CommandStep::aur(
                    &["-Rdd", "--noconfirm", "iptables"],
                    "Removing conflicting iptables...",
                ));
            }
            if core::is_package_installed("gnu-netcat") {
                commands.push(progress_dialog::CommandStep::aur(
                    &["-Rdd", "--noconfirm", "gnu-netcat"],
                    "Removing conflicting gnu-netcat...",
                ));
            }

            commands.push(progress_dialog::CommandStep::aur(
                &[
                    "-S",
                    "--noconfirm",
                    "--needed",
                    "virt-manager-meta",
                    "openbsd-netcat",
                ],
                "Installing virtualization packages...",
            ));
            commands.push(progress_dialog::CommandStep::privileged(
                "sh",
                &[
                    "-c",
                    "echo 'options kvm-intel nested=1' > /etc/modprobe.d/kvm-intel.conf",
                ],
                "Enabling nested virtualization...",
            ));
            commands.push(progress_dialog::CommandStep::privileged(
                "systemctl",
                &["restart", "libvirtd.service"],
                "Restarting libvirtd service...",
            ));

            let widget = button.clone().upcast::<gtk4::Widget>();
            let window = widget
                .root()
                .and_then(|r| r.downcast::<ApplicationWindow>().ok());
            if let Some(window) = window {
                let window_ref = window.upcast_ref::<gtk4::Window>();
                progress_dialog::run_commands_with_progress(
                    window_ref,
                    commands,
                    "KVM / QEMU Setup",
                    None,
                );
            }
        });
    }
}
