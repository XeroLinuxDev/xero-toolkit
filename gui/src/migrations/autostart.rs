//! Autostart-related config migrations.

use super::Migration;
use crate::config::user::Config;
use crate::core;
use anyhow::Result;
use log::info;

pub const MIGRATION_ID: &str = "2026-02-26-autostart-state-from-desktop-entry";

pub const MIGRATION: Migration = Migration {
    id: MIGRATION_ID,
    name: "Migrate legacy autostart desktop-entry state into config",
    run: migrate_legacy_autostart_state,
};

fn migrate_legacy_autostart_state(config: &mut Config) -> Result<()> {
    let detected_state = core::autostart::get_autostart_path()
        .symlink_metadata()
        .is_ok()
        || crate::config::paths::system_autostart().exists();
    let configured_state = config.general.autostart;

    if configured_state != detected_state {
        info!(
            "Migrating legacy autostart state: config={}, detected={}. Updating config.",
            configured_state, detected_state
        );
        config.general.autostart = detected_state;
    }

    Ok(())
}
