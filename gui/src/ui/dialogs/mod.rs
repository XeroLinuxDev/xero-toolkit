//! Dialog windows for user interaction.
//!
//! This module contains all dialog-related UI components:
//! - `about`: About dialog with creator information
//! - `button_info`: Explanatory info dialogs for action buttons
//! - `error`: Simple error message dialogs
//! - `selection`: Multi-choice selection dialogs
//! - `download`: ISO download dialogs
//! - `terminal`: Interactive terminal dialogs

pub mod about;
pub mod button_info;
pub mod download;
pub mod error;
pub mod selection;
pub mod terminal;
pub mod warning;
