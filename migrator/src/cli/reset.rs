use super::config::{RuntimeConfig, SharedAll};
use super::up::Up;

use clap::{ArgAction, Parser};
use std::fmt::Display;
use std::fs;
use std::str::FromStr;

use surreal_query_builder::statements::info_for;
use surreal_query_builder::{DbResources, Runnable};
use surrealdb::engine::any::{connect, Any};

use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use super::{config::setup_db, init::Init};
use crate::{DbInfo, MigrationConfig, MigrationFlag, MigrationRunner, RollbackOptions};

/// Resets migrations. Deletes all migration files, migration table and reinitializes
/// migrations.
#[derive(Parser, Debug)]
pub struct Reset {
    /// Name of the first migration file(s) to reinitialize to
    #[clap(long)]
    pub(crate) name: String,

    /// Whether or not to run the migrations after reinitialization. Reinitalization
    /// is done by deleting all migration files, and regenerating
    /// the first migration file(s) which include queries to delete all old
    /// migration metadata in the database before creating the new ones.
    #[clap(long)]
    pub(crate) run: bool,

    /// Two way migration
    #[clap(
        short,
        long,
        help = "Whether to reinitialize as Unidirectional(Up only) Bidirectional(up & down) migration(S)"
    )]
    pub(crate) reversible: bool,

    #[clap(flatten)]
    pub(crate) shared_all: SharedAll,

    #[clap(flatten)]
    pub(crate) shared_run_and_rollback: RuntimeConfig,
}

impl Reset {
    pub async fn run(&self, codebase_resources: impl DbResources) {
        let mut files_config = MigrationConfig::new().make_strict();

        if let Some(path) = self.shared_all.migrations_dir.clone() {
            files_config = files_config.custom_path(path)
        };

        let dir = files_config.get_migration_dir_create_if_none();
        match dir {
            Ok(dir) => {
                if dir.exists() {
                    let removed = fs::remove_dir_all(&dir);
                    if let Err(e) = removed {
                        log::error!("Failed to remove dir: {e}");
                        panic!();
                    } else {
                        fs::create_dir(&dir).expect("Problem creating migration directory");
                        log::info!("Migration directory recreated.");
                    }
                } else {
                    fs::create_dir(dir).expect("Problem creating migration directory");
                    log::info!("Migration directory recreated.");
                }
            }
            Err(e) => {
                log::error!("Failed to get migration dir: {e}");
                panic!();
            }
        };

        let init = Init {
            name: self.name.clone(),
            run: self.run,
            reversible: self.reversible.clone(),
            shared_all: self.shared_all.clone(),
            shared_run_and_rollback: self.shared_run_and_rollback.clone(),
        };
        init.run(codebase_resources).await;

        log::info!("Reset successful");
    }
}
