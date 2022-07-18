//! VestingGenerator Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.

use serde::{Deserialize, Serialize};

/// VestingGenerator Configuration
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct VestingGeneratorConfig {
    /// Represent the configuration of the generate command
    pub generator: GeneratorSection,
}

/// Generate configuration section.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GeneratorSection {
    /// Set token denom registered into the json file.
    pub denom: String,
}

impl Default for GeneratorSection {
    fn default() -> Self {
        Self {
            denom: "uknow".to_owned(),
        }
    }
}
