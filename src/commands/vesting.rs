//! `generate` subcommand
mod generate_cliff;

use crate::commands::vesting::generate_cliff::GenerateCliffCmd;
use crate::config::CosmosToolsConfig;
use abscissa_core::{config, Command, FrameworkError, Runnable};
use clap::Parser;

/// Vesting Subcommands
#[derive(Command, Debug, Parser, Runnable)]
pub enum VestingSubCmd {
    /// Generate a JSON file containing all vesting periods based on interval and cliff duration
    /// configured
    GenerateCliff(GenerateCliffCmd),
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
        config: CosmosToolsConfig,
    ) -> Result<CosmosToolsConfig, FrameworkError> {
        match &self.cmd {
            VestingSubCmd::GenerateCliff(cmd) => cmd.override_config(config),
        }
    }
}
