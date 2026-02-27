//! One-time data/config migrations.

pub mod autostart;

use crate::config::user::Config;
use anyhow::{bail, Result};
use log::info;

pub struct Migration {
    pub id: &'static str,
    pub name: &'static str,
    pub run: fn(&mut Config) -> Result<()>,
}

const MIGRATIONS: &[Migration] = &[autostart::MIGRATION];

/// Run all known startup migrations and record successful ones.
pub fn run_startup_migrations(config: &mut Config) -> Result<()> {
    for migration in MIGRATIONS {
        if config.migrations.is_applied(migration.id) {
            continue;
        }

        info!("Applying migration {} ({})", migration.id, migration.name);
        (migration.run)(config)?;
        config.migrations.mark_applied(migration.id);
        info!("Migration {} applied successfully", migration.id);
    }

    ensure_no_duplicate_applied_ids(config)?;
    Ok(())
}

fn ensure_no_duplicate_applied_ids(config: &mut Config) -> Result<()> {
    let mut seen = std::collections::HashSet::new();
    for id in &config.migrations.applied {
        if !seen.insert(id) {
            bail!("Duplicate migration id found in config: {}", id);
        }
    }
    Ok(())
}
