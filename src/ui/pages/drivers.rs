//! Drivers and hardware tools page button handlers.
//!
//! Handles:
//! - NVIDIA GPU drivers (closed and open source) via selection dialog
//! - Tailscale VPN
//! - ASUS ROG laptop tools

use crate::ui::command_execution as progress_dialog;
use crate::ui::selection_dialog;
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Builder};
use log::{info, warn};

/// Set up all button handlers for the drivers page
pub fn setup_handlers(page_builder: &Builder, _main_builder: &Builder) {
    setup_gpu_drivers(&page_builder);
    setup_tailscale(&page_builder);
    setup_asus_rog(&page_builder);
}

fn setup_gpu_drivers(page_builder: &Builder) {
    if let Some(btn_gpu_drivers) = page_builder.object::<gtk4::Button>("btn_gpu_drivers") {
        btn_gpu_drivers.connect_clicked(move |button| {
            info!("Drivers: GPU Drivers button clicked");

            show_gpu_driver_selection(button);
        });
    }
}

fn show_gpu_driver_selection(button: &gtk4::Button) {
    let widget = button.clone().upcast::<gtk4::Widget>();
    let window = widget
        .root()
        .and_then(|root| root.downcast::<ApplicationWindow>().ok());

    if let Some(window) = window {
        let window_ref = window.upcast_ref::<gtk4::Window>();

        let config = selection_dialog::SelectionDialogConfig::new(
            "NVIDIA Driver Selection",
            "Select which NVIDIA driver version to install.",
        )
        .add_option(selection_dialog::SelectionOption::new(
            "nvidia_closed",
            "NVIDIA Closed Source",
            "Proprietary NVIDIA drivers",
            false,
        ))
        .add_option(selection_dialog::SelectionOption::new(
            "nvidia_open",
            "NVIDIA Open Source",
            "Open source NVIDIA drivers (Turing+ GPUs)",
            false,
        ))
        .add_option(selection_dialog::SelectionOption::new(
            "cuda",
            "CUDA Toolkit",
            "NVIDIA CUDA Toolkit for GPU-accelerated computing",
            false,
        ))
        .confirm_label("Install");

        let window_clone = window.clone();
        selection_dialog::show_selection_dialog(window_ref, config, move |selected_ids| {
            // Check if both drivers are selected (conflict)
            if selected_ids.contains(&"nvidia_closed".to_string())
                && selected_ids.contains(&"nvidia_open".to_string())
            {
                warn!("Both NVIDIA drivers selected - conflict");
                show_error(&window_clone, "Cannot install both closed and open source NVIDIA drivers.\nPlease select only one.");
                return;
            }

            let mut commands = vec![];

            if selected_ids.contains(&"nvidia_closed".to_string()) {
                commands.push(progress_dialog::CommandStep::aur(
                    &[
                        "-S",
                        "--needed",
                        "--noconfirm",
                        "libvdpau",
                        "egl-wayland",
                        "nvidia-dkms",
                        "nvidia-utils",
                        "opencl-nvidia",
                        "libvdpau-va-gl",
                        "nvidia-settings",
                        "vulkan-icd-loader",
                        "lib32-nvidia-utils",
                        "lib32-opencl-nvidia",
                        "linux-firmware-nvidia",
                        "lib32-vulkan-icd-loader",
                    ],
                    "Installing NVIDIA proprietary drivers...",
                ));
            }

            if selected_ids.contains(&"nvidia_open".to_string()) {
                commands.push(progress_dialog::CommandStep::aur(
                    &[
                        "-S",
                        "--needed",
                        "--noconfirm",
                        "libvdpau",
                        "egl-wayland",
                        "nvidia-utils",
                        "opencl-nvidia",
                        "libvdpau-va-gl",
                        "nvidia-settings",
                        "nvidia-open-dkms",
                        "vulkan-icd-loader",
                        "lib32-nvidia-utils",
                        "lib32-opencl-nvidia",
                        "linux-firmware-nvidia",
                        "lib32-vulkan-icd-loader",
                    ],
                    "Installing NVIDIA open source drivers...",
                ));
            }

            if selected_ids.contains(&"cuda".to_string()) {
                commands.push(progress_dialog::CommandStep::aur(
                    &["-S", "--needed", "--noconfirm", "cuda", "cudnn"],
                    "Installing CUDA Toolkit...",
                ));
            }

            // Run NVIDIA post-install configuration script only if a driver was selected
            let driver_selected = selected_ids.contains(&"nvidia_closed".to_string())
                || selected_ids.contains(&"nvidia_open".to_string());

            if driver_selected {
                commands.push(progress_dialog::CommandStep::privileged(
                    "bash",
                    &["/opt/xero-toolkit/scripts/nv-setup.sh"],
                    "Configuring NVIDIA drivers...",
                ));
            }

            if !commands.is_empty() {
                let window_ref = window_clone.upcast_ref::<gtk4::Window>();
                progress_dialog::run_commands_with_progress(
                    window_ref,
                    commands,
                    "GPU Driver Installation",
                    None,
                );
            }
        });
    }
}

fn setup_tailscale(page_builder: &Builder) {
    if let Some(btn_tailscale) = page_builder.object::<gtk4::Button>("btn_tailscale") {
        btn_tailscale.connect_clicked(move |button| {
            info!("Drivers: Tailscale VPN button clicked");


            let widget = button.clone().upcast::<gtk4::Widget>();
            let window = widget
                .root()
                .and_then(|root| root.downcast::<ApplicationWindow>().ok());

            if let Some(window) = window {
                let commands = vec![progress_dialog::CommandStep::privileged(
                    "bash",
                    &["-c", "curl -fsSL https://raw.githubusercontent.com/xerolinux/xero-fixes/main/conf/install.sh | bash"],
                    "Installing Tailscale VPN...",
                )];

                let window_ref = window.upcast_ref::<gtk4::Window>();
                progress_dialog::run_commands_with_progress(
                    window_ref,
                    commands,
                    "Install Tailscale VPN",
                    None,
                );
            }
        });
    }
}

fn setup_asus_rog(page_builder: &Builder) {
    if let Some(btn_asus_rog) = page_builder.object::<gtk4::Button>("btn_asus_rog") {
        btn_asus_rog.connect_clicked(move |button| {
            info!("Drivers: ASUS ROG Tools button clicked");

            let widget = button.clone().upcast::<gtk4::Widget>();
            let window = widget
                .root()
                .and_then(|root| root.downcast::<ApplicationWindow>().ok());

            if let Some(window) = window {
                let commands = vec![
                    progress_dialog::CommandStep::aur(
                        &[
                            "-S",
                            "--noconfirm",
                            "--needed",
                            "rog-control-center",
                            "asusctl",
                            "supergfxctl",
                        ],
                        "Installing ASUS ROG control tools...",
                    ),
                    progress_dialog::CommandStep::privileged(
                        "systemctl",
                        &["enable", "--now", "asusd", "supergfxd"],
                        "Enabling ASUS ROG services...",
                    ),
                ];

                let window_ref = window.upcast_ref::<gtk4::Window>();
                progress_dialog::run_commands_with_progress(
                    window_ref,
                    commands,
                    "Install ASUS ROG Tools",
                    None,
                );
            }
        });
    }
}

fn show_error(window: &ApplicationWindow, message: &str) {
    let dialog = gtk4::MessageDialog::builder()
        .transient_for(window)
        .modal(true)
        .message_type(gtk4::MessageType::Error)
        .buttons(gtk4::ButtonsType::Ok)
        .text("Error")
        .secondary_text(message)
        .build();

    dialog.connect_response(|dialog, _| dialog.close());
    dialog.present();
}
