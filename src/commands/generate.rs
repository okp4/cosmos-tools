//! `start` subcommand - example of how to write a subcommand

use abscissa_core::{Command, Runnable};
use clap::Parser;
use std::path::PathBuf;

/// `start` subcommand
///
/// The `Parser` proc macro generates an option parser based on the struct
/// definition, and is defined in the `clap` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/clap/>
#[derive(Command, Debug, Parser)]
pub struct GenerateCmd {
    /// The total amount of token to vest
    total_amount: u128,
    /// The period interval (in second) which amount is split
    #[clap(short, long)]
    interval: u64,
    /// The total duration of vesting (in seconds)
    #[clap(short, long)]
    duration: u64,
    /// Cliff duration (in seconds)
    #[clap(short = 'c', long = "cliff", default_value = "0")]
    cliff_duration: u64,
    /// The path to the output file where JSON will be write
    #[clap(short = 'o', long = "output")]
    output: PathBuf,
}

impl Runnable for GenerateCmd {
    /// Start the application.
    fn run(&self) {
        println!("Hello, world!");
    }
}
