//! VestingGenerator Config

use serde::{Deserialize, Serialize};

/// VestingGenerator Configuration
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct CosmosToolsConfig {
    /// Represent the configuration of the vesting subcommands
    pub vesting: VestingSection,
}

/// Generate configuration section.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VestingSection {
    /// Set token denom registered into the json file.
    pub denom: String,
}

impl Default for VestingSection {
    fn default() -> Self {
        Self {
            denom: "uknow".to_owned(),
        }
    }
}
