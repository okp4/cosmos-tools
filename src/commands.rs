//! VestingGenerator Subcommands

mod vesting;

use self::vesting::VestingCmd;
use crate::config::CosmosToolsConfig;
use abscissa_core::config::Override;
use abscissa_core::{Command, Configurable, FrameworkError, Runnable};
use clap::Parser;
use std::path::PathBuf;

/// Cosmos tools Configuration Filename
pub const CONFIG_FILE: &str = "cosmos_tools.toml";

/// Cosmos tools Subcommands
#[derive(Command, Debug, Parser, Runnable)]
pub enum CosmosToolsCmd {
    /// Utilities for the vesting part of cosmos
    Vesting(VestingCmd),
}

/// Entry point for the application.
#[derive(Command, Debug, Parser)]
#[clap(author, about, version)]
pub struct EntryPoint {
    #[clap(subcommand)]
    cmd: CosmosToolsCmd,

    /// Enable verbose logging
    #[clap(short, long)]
    pub verbose: bool,

    /// Use the specified config file
    #[clap(short, long)]
    pub config: Option<String>,
}

impl Runnable for EntryPoint {
    fn run(&self) {
        self.cmd.run()
    }
}

impl Configurable<CosmosToolsConfig> for EntryPoint {
    fn config_path(&self) -> Option<PathBuf> {
        // Check if the config file exists, and if it does not, ignore it.
        // If you'd like for a missing configuration file to be a hard error
        // instead, always return `Some(CONFIG_FILE)` here.
        let filename = self
            .config
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(|| CONFIG_FILE.into());

        if filename.exists() {
            Some(filename)
        } else {
            None
        }
    }

    fn process_config(
        &self,
        config: CosmosToolsConfig,
    ) -> Result<CosmosToolsConfig, FrameworkError> {
        match &self.cmd {
            CosmosToolsCmd::Vesting(cmd) => cmd.override_config(config),
        }
    }
}
