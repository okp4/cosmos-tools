//! Acceptance test: runs the application as a subprocess and asserts its
//! output for given argument combinations matches what is expected.
//!
//! Modify and/or delete these as you see fit to test the specific needs of
//! your application.
//!
//! For more information, see:
//! <https://docs.rs/abscissa_core/latest/abscissa_core/testing/index.html>

// Tip: Deny warnings with `RUSTFLAGS="-D warnings"` environment variable in CI

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    trivial_casts,
    unused_lifetimes,
    unused_qualifications
)]

use std::io::Read;
use abscissa_core::terminal::Streams;
use abscissa_core::testing::prelude::*;
use once_cell::sync::Lazy;

/// Executes your application binary via `cargo run`.
///
/// Storing this value as a [`Lazy`] static ensures that all instances of
/// the runner acquire a mutex when executing commands and inspecting
/// exit statuses, serializing what would otherwise be multithreaded
/// invocations as `cargo test` executes tests in parallel by default.
pub static RUNNER: Lazy<CmdRunner> = Lazy::new(CmdRunner::default);

/// Example of a test which matches a regular expression
#[test]
fn version_no_args() {
    let mut runner = RUNNER.clone();
    let mut cmd = runner.arg("--version").capture_stdout().run();
    cmd.stdout().expect_regex(r"\A\w+ [\d\.\-]+\z");
}

/// Use command-line argument value
#[test]
fn generate_with_args() {
    let mut runner = RUNNER.clone();
    let mut cmd = runner
        .args(&["generate", "40000", "--duration", "1000", "--interval", "1000"])
        .capture_stdout()
        .run();
    let output = read_until_end_stdout(cmd.stdout());
    assert_eq!(output, "[\n  {\n    \"length\": \"1000\",\n    \"amount\": {\n      \"denom\": \"uknow\",\n      \"amount\": \"40000\"\n    }\n  }\n]\n");
    cmd.wait().unwrap().expect_success();
}

fn read_until_end_stdout(stdout: &mut Stdout) -> String {
    let mut output = String::new();
    stdout.read_to_string(&mut output)
        .unwrap_or_else(|e| panic!("error reading line from {}: {}", stringify!($name), e));
    output
}
