//! `generate` subcommand
mod generate_cliff;

use crate::application::APP;
use crate::config::CosmosToolsConfig;
use abscissa_core::config::Reader;
use abscissa_core::{config, Application, Command, FrameworkError, Runnable};
use clap::Parser;
use serde_derive::{Deserialize, Serialize};
use std::{path::PathBuf, process::exit};
use crate::commands::vesting::generate_cliff::GenerateCliffCmd;

/// Vesting Subcommands
#[derive(Command, Debug, Parser, Runnable)]
pub enum VestingSubCmd {
    GenerateCliff(GenerateCliffCmd)
}

/// `Vesting` subcommand
#[derive(Command, Debug, Parser, Runnable)]
pub struct VestingCmd {
    #[clap(subcommand)]
    cmd: VestingSubCmd,
}

impl config::Override<CosmosToolsConfig> for VestingCmd {
    fn override_config(
        &self,
        mut config: CosmosToolsConfig,
    ) -> Result<CosmosToolsConfig, FrameworkError> {
        match &self.cmd {
            VestingSubCmd::GenerateCliff(cmd) => cmd.override_config(config),
        }
    }
}
