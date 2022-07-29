//! Main entry point for VestingGenerator

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use vesting_generator::application::APP;

/// Boot VestingGenerator
fn main() {
    abscissa_core::boot(&APP);
}
