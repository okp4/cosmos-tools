//! `start` subcommand - example of how to write a subcommand

use crate::application::APP;
use crate::config::VestingGeneratorConfig;
use abscissa_core::{config, Application, Command, FrameworkError, Runnable};
use clap::Parser;
use serde_derive::{Deserialize, Serialize};
use std::{path::PathBuf, process::exit};

/// `generate` subcommand
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
    /// Cliff duration (in seconds), if not filled, vesting start immediately.
    #[clap(short = 'c', long = "cliff", default_value = "0")]
    cliff_duration: u64,
    /// The path to the output file where JSON will be write, if not filled, json will be write on
    /// stdout.
    #[clap(short = 'o', long = "output")]
    output: Option<PathBuf>,
    /// Configure the token denom used into json configuration
    #[clap(long = "denom")]
    denom: Option<String>,
}

impl GenerateCmd {
    fn get_vested_coin(&self, time: u64) -> u128 {
        if time < self.cliff_duration {
            return 0;
        } else if time >= self.duration {
            return self.total_amount;
        }
        let coef = time as f64 / self.duration as f64;
        let total = self.total_amount as f64 * coef;
        total as u128
    }

    fn build_periods(&self) -> Vec<Period> {
        let config = APP.config();
        let periods = self.duration / self.interval;

        let mut result = Vec::new();
        let mut last_vested = 0;
        for i in 1..periods + 1 {
            let time = self.interval * i as u64;
            let vested = self.get_vested_coin(time);

            let token = vested - last_vested;

            if token == 0 {
                continue;
            }

            let period = Period {
                length: time.to_string(),
                amount: Token {
                    denom: String::from(&config.generator.denom),
                    amount: format!("{}", token),
                },
            };

            result.push(period);
            last_vested = vested
        }

        result
    }
}

impl Runnable for GenerateCmd {
    /// Start the application.
    fn run(&self) {
        let result = self.build_periods();
        let json = serde_json::to_string_pretty(&result);

        match json {
            Ok(json) => {
                if self.output.is_some() {
                    let _ = std::fs::write(&self.output.as_ref().unwrap(), &json);
                } else {
                    println!("{}", json);
                }
            }
            _ => {
                println!("failed convert to json object");
                exit(1);
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct Period {
    length: String,
    amount: Token,
}

#[derive(Deserialize, Serialize, Debug)]
struct Token {
    denom: String,
    amount: String,
}

impl config::Override<VestingGeneratorConfig> for GenerateCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(
        &self,
        mut config: VestingGeneratorConfig,
    ) -> Result<VestingGeneratorConfig, FrameworkError> {
        if !self.denom.is_none() {
            config.generator.denom = self.denom.as_ref().unwrap().clone();
        }

        Ok(config)
    }
}

#[cfg(test)]
mod generate_tests {
    use super::*;
    use clap::ErrorKind::NoEquals;

    #[test]
    fn test_initial_vest_without_cliff() {
        let cmd = GenerateCmd {
            total_amount: 40000,
            interval: 86400,    // 1 day interval
            duration: 63072000, // 2 years
            cliff_duration: 0,  // no cliff
            output: None,
            denom: None,
        };

        let result = cmd.get_vested_coin(cmd.interval);

        assert_eq!(result, 54, "First vesting should be equal 54");
    }

    #[test]
    fn test_initial_vest_with_cliff() {
        let cmd = GenerateCmd {
            total_amount: 40000,
            interval: 86400,          // 1 day interval
            duration: 63072000,       // 2 years
            cliff_duration: 15768000, // 6 month cliff
            output: None,
            denom: None,
        };

        let mut result = cmd.get_vested_coin(cmd.interval);

        assert_eq!(
            result, 0,
            "First vesting should be equal 0, since cliff has not been start"
        );

        result = cmd.get_vested_coin(15811200); // 15811200 is the day after 6 month past with 86400 interval

        assert_eq!(
            result, 10027,
            "First vesting should be equal to 10027, since vesting is already start before cliff"
        );
    }

    #[test]
    fn test_build_periods_without_cliff() {
        let cmd = GenerateCmd {
            total_amount: 40000,
            interval: 86400,    // 1 day interval
            duration: 63072000, // 2 years
            cliff_duration: 0,
            output: None,
            denom: None,
        };

        let result = cmd.build_periods();
        assert_eq!(
            result.len(),
            730,
            "Should be 730 interval since there are 730 day in two years"
        );

        let total = result
            .iter()
            .map(|p| p.amount.amount.to_string().parse::<u128>().unwrap_or(0))
            .reduce(|r, p| r + p)
            .unwrap_or(0);

        assert_eq!(
            total, cmd.total_amount,
            "The total amount cumuled period should be equal to the total amount configured."
        )
    }

    #[test]
    fn test_build_periods_with_cliff() {
        let cmd = GenerateCmd {
            total_amount: 40000,
            interval: 86400,          // 1 day interval
            duration: 63072000,       // 2 years
            cliff_duration: 15768000, // 6 month cliff
            output: None,
            denom: None,
        };

        let result = cmd.build_periods();
        assert_eq!(result.len(), 548, "Should be 548 interval since there are 730 day in two years minus the 182 day of the 6 month cliff");

        let total = result
            .iter()
            .map(|p| p.amount.amount.to_string().parse::<u128>().unwrap_or(0))
            .reduce(|r, p| r + p)
            .unwrap_or(0);

        assert_eq!(
            total, cmd.total_amount,
            "The total amount cumulated period should be equal to the total amount configured."
        )
    }
}
