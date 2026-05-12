//! OpenClaw migration command dispatch.

use crate::cli::{ClawAction, Commands, MigrateSource};
use crate::command_registry::CommandRegistry;
use skilllite_commands::migrate::{cmd_claw_migrate_openclaw, OpenclawMigrateOptions};

fn run_openclaw_migrate(opts: OpenclawMigrateOptions<'_>) -> crate::Result<()> {
    cmd_claw_migrate_openclaw(opts).map_err(Into::into)
}

pub fn register(reg: &mut CommandRegistry) {
    reg.register(|cmd| {
        if let Commands::Claw { action } = cmd {
            match action {
                ClawAction::Migrate {
                    workspace,
                    openclaw_dir,
                    skills_dir,
                    skill_conflict,
                    dry_run,
                    force,
                    scan_offline,
                    migrate_secrets,
                    no_backup,
                    yes,
                    overwrite,
                } => Some(run_openclaw_migrate(OpenclawMigrateOptions {
                    workspace,
                    openclaw_dir: openclaw_dir.as_deref(),
                    skills_dir,
                    skill_conflict,
                    dry_run: *dry_run,
                    force: *force,
                    scan_offline: *scan_offline,
                    migrate_secrets: *migrate_secrets,
                    no_backup: *no_backup,
                    yes: *yes,
                    overwrite: *overwrite,
                })),
            }
        } else {
            None
        }
    });

    reg.register(|cmd| {
        if let Commands::Migrate { source } = cmd {
            match source {
                MigrateSource::Openclaw {
                    workspace,
                    openclaw_dir,
                    skills_dir,
                    skill_conflict,
                    dry_run,
                    force,
                    scan_offline,
                    migrate_secrets,
                    no_backup,
                    yes,
                    overwrite,
                } => Some(run_openclaw_migrate(OpenclawMigrateOptions {
                    workspace,
                    openclaw_dir: openclaw_dir.as_deref(),
                    skills_dir,
                    skill_conflict,
                    dry_run: *dry_run,
                    force: *force,
                    scan_offline: *scan_offline,
                    migrate_secrets: *migrate_secrets,
                    no_backup: *no_backup,
                    yes: *yes,
                    overwrite: *overwrite,
                })),
            }
        } else {
            None
        }
    });
}
