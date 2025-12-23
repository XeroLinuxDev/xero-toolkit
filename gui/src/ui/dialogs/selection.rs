//! Selection dialog for multi-choice options.
//!
//! This module provides a reusable dialog window for presenting users with
//! multiple options to select from, with customizable title, description, and actions.

use crate::ui::utils::extract_widget;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Builder, Button, CheckButton, Label, Separator, Window};
use log::info;
use std::cell::RefCell;
use std::rc::Rc;

/// Represents a selectable option in the dialog
#[derive(Clone, Debug)]
pub struct SelectionOption {
    pub id: String,
    pub label: String,
    pub description: String,
    pub installed: bool,
}

impl SelectionOption {
    /// Create a new selection option
    pub fn new(id: &str, label: &str, description: &str, installed: bool) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            description: description.to_string(),
            installed,
        }
    }
}

/// Selection type for the dialog
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectionType {
    /// Single selection (radio buttons)
    Single,
    /// Multiple selection (checkboxes)
    Multi,
}

/// Configuration for the selection dialog
pub struct SelectionDialogConfig {
    pub title: String,
    pub description: String,
    pub options: Vec<SelectionOption>,
    pub confirm_label: String,
    pub selection_type: SelectionType,
    pub selection_required: bool,
}

impl SelectionDialogConfig {
    /// Create a new dialog configuration
    pub fn new(title: &str, description: &str) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            options: Vec::new(),
            confirm_label: "Install".to_string(),
            selection_type: SelectionType::Multi,
            selection_required: true,
        }
    }

    /// Add an option to the dialog
    pub fn add_option(mut self, option: SelectionOption) -> Self {
        self.options.push(option);
        self
    }

    /// Set the confirm button label
    pub fn confirm_label(mut self, label: &str) -> Self {
        self.confirm_label = label.to_string();
        self
    }

    /// Set the selection type (single or multi)
    pub fn selection_type(mut self, selection_type: SelectionType) -> Self {
        self.selection_type = selection_type;
        self
    }

    /// Set whether selection is required to confirm
    pub fn selection_required(mut self, required: bool) -> Self {
        self.selection_required = required;
        self
    }
}

/// Show a selection dialog and call the callback with selected option IDs
pub fn show_selection_dialog<F>(parent: &Window, config: SelectionDialogConfig, on_confirm: F)
where
    F: Fn(Vec<String>) + 'static,
{
    info!("Opening selection dialog: {}", config.title);

    // Load the UI from resource
    let builder = Builder::from_resource(crate::config::resources::dialogs::SELECTION);

    // Get the dialog window
    let dialog: Window = extract_widget(&builder, "selection_dialog");

    // Set transient parent
    dialog.set_transient_for(Some(parent));

    // Get UI elements
    let title_label: Label = extract_widget(&builder, "dialog_title");
    let description_label: Label = extract_widget(&builder, "dialog_description");
    let options_container: GtkBox = extract_widget(&builder, "options_container");
    let cancel_button: Button = extract_widget(&builder, "cancel_button");
    let confirm_button: Button = extract_widget(&builder, "confirm_button");

    // Set title and description
    title_label.set_label(&config.title);
    description_label.set_label(&config.description);
    confirm_button.set_label(&config.confirm_label);

    let checkboxes: Rc<RefCell<Vec<(String, CheckButton)>>> = Rc::new(RefCell::new(Vec::new()));
    let radio_buttons: Rc<RefCell<Vec<(String, CheckButton)>>> = Rc::new(RefCell::new(Vec::new()));
    let selection_type = config.selection_type;
    let selection_required = config.selection_required;

    let mut first_radio: Option<CheckButton> = None;

    for (i, option) in config.options.iter().enumerate() {
        // Horizontal box: checkbox/radio on left, text on right
        let option_row = GtkBox::new(gtk4::Orientation::Horizontal, 12);
        option_row.set_margin_start(12);
        option_row.set_margin_end(12);
        option_row.set_margin_top(8);
        option_row.set_margin_bottom(8);

        // Create checkbox or radio button based on selection type
        match selection_type {
            SelectionType::Multi => {
                let checkbox = CheckButton::new();
                checkbox.set_active(option.installed);
                checkbox.set_sensitive(!option.installed);
                checkboxes
                    .borrow_mut()
                    .push((option.id.clone(), checkbox.clone()));

                // Vertical box for title and description
                let text_box = GtkBox::new(gtk4::Orientation::Vertical, 4);
                text_box.set_hexpand(true);

                let title_label = Label::new(Some(&option.label));
                title_label.set_halign(gtk4::Align::Start);
                title_label.set_wrap(true);
                if option.installed {
                    title_label.set_css_classes(&["dim"]);
                }

                let desc_label = Label::new(Some(&option.description));
                desc_label.set_css_classes(&["dim", "caption"]);
                desc_label.set_halign(gtk4::Align::Start);
                desc_label.set_wrap(true);

                text_box.append(&title_label);
                text_box.append(&desc_label);

                option_row.append(&checkbox);
                option_row.append(&text_box);
            }
            SelectionType::Single => {
                let radio = if let Some(ref first) = first_radio {
                    let radio = CheckButton::new();
                    radio.set_group(Some(first));
                    radio
                } else {
                    let radio = CheckButton::new();
                    first_radio = Some(radio.clone());
                    radio
                };
                radio.set_active(option.installed);
                radio.set_sensitive(!option.installed);
                radio_buttons
                    .borrow_mut()
                    .push((option.id.clone(), radio.clone()));

                // Vertical box for title and description
                let text_box = GtkBox::new(gtk4::Orientation::Vertical, 4);
                text_box.set_hexpand(true);

                let title_label = Label::new(Some(&option.label));
                title_label.set_halign(gtk4::Align::Start);
                title_label.set_wrap(true);
                if option.installed {
                    title_label.set_css_classes(&["dim"]);
                }

                let desc_label = Label::new(Some(&option.description));
                desc_label.set_css_classes(&["dim", "caption"]);
                desc_label.set_halign(gtk4::Align::Start);
                desc_label.set_wrap(true);

                text_box.append(&title_label);
                text_box.append(&desc_label);

                option_row.append(&radio);
                option_row.append(&text_box);
            }
        }

        options_container.append(&option_row);

        // Add separator between options (not after the last one)
        if i < config.options.len() - 1 {
            let sep = Separator::new(gtk4::Orientation::Horizontal);
            options_container.append(&sep);
        }
    }

    // Set initial state of confirm button based on selection_required
    if selection_required {
        confirm_button.set_sensitive(false);
    }

    // Cancel button - just close the dialog
    let dialog_clone = dialog.clone();
    cancel_button.connect_clicked(move |_| {
        info!("Selection dialog cancelled");
        dialog_clone.close();
    });

    // Update confirm button sensitivity based on selection
    let update_confirm_button = {
        let confirm_button_clone = confirm_button.clone();
        let checkboxes_clone = checkboxes.clone();
        let radio_buttons_clone = radio_buttons.clone();

        move || {
            let has_selection = match selection_type {
                SelectionType::Multi => checkboxes_clone
                    .borrow()
                    .iter()
                    .any(|(_, checkbox)| checkbox.is_active()),
                SelectionType::Single => radio_buttons_clone
                    .borrow()
                    .iter()
                    .any(|(_, radio)| radio.is_active()),
            };

            if selection_required {
                confirm_button_clone.set_sensitive(has_selection);
            } else {
                confirm_button_clone.set_sensitive(true);
            }
        }
    };

    // Connect selection change handlers
    let connect_toggle_handler = |button: &CheckButton| {
        let update = update_confirm_button.clone();
        button.connect_toggled(move |_| {
            update();
        });
    };

    for (_, checkbox) in checkboxes.borrow().iter() {
        connect_toggle_handler(checkbox);
    }

    for (_, radio) in radio_buttons.borrow().iter() {
        connect_toggle_handler(radio);
    }

    // Confirm button - collect selected options and call callback
    let dialog_clone = dialog.clone();
    let checkboxes_clone = checkboxes.clone();
    let radio_buttons_clone = radio_buttons.clone();
    confirm_button.connect_clicked(move |_| {
        let collect_selected = |(id, button): &(String, CheckButton)| {
            (button.is_active() && button.is_sensitive()).then_some(id.clone())
        };

        let selected: Vec<String> = match selection_type {
            SelectionType::Multi => checkboxes_clone
                .borrow()
                .iter()
                .filter_map(collect_selected)
                .collect(),
            SelectionType::Single => radio_buttons_clone
                .borrow()
                .iter()
                .filter_map(collect_selected)
                .collect(),
        };

        info!(
            "Selection dialog confirmed with {} selections",
            selected.len()
        );

        on_confirm(selected);

        dialog_clone.close();
    });

    // Show the dialog
    dialog.present();
}
