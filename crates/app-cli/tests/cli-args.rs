//! Integration test for the `app` binary.
//!
//! # Why this test starts a process
//!
//! `app-cli` has no library target, so there is nothing here to `use`. The only
//! way in is `argv`, and the only way out is stdout, stderr, and an exit code.
//!
//! Cargo makes that bearable: it sets `CARGO_BIN_EXE_<name>` for every binary
//! target in the package, so `env!("CARGO_BIN_EXE_app")` is the path to the
//! freshly built executable. No hard-coded `target/debug/app`, and it works
//! under `--release` and cross-compilation.
//!
//! The tests below are still slower, coarser and harder to debug than the unit
//! tests in `app-core`. That is the cost of putting logic in a binary, and the
//! reason `main.rs` is four lines long.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_app"))
        .args(args)
        .output()
        .expect("failed to run the `app` binary")
}

#[test]
fn total_sums_the_lines() {
    let output = run(&["total", "--line", "widget:2:1500", "--line", "gadget:1:999"]);

    assert!(output.status.success(), "stderr: {:?}", output.stderr);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("2 lines"), "stdout was: {stdout}");
    assert!(stdout.contains("EUR 39.99"), "stdout was: {stdout}");
}

#[test]
fn config_prints_the_defaults() {
    let output = run(&["config"]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("max_order_lines: 100"),
        "stdout was: {stdout}"
    );
}

#[test]
fn a_malformed_line_is_rejected_with_context() {
    let output = run(&["total", "--line", "widget:two:1500"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    // `anyhow`'s context chain is what turns "invalid digit found in string"
    // into something a user can act on.
    assert!(
        stderr.contains("invalid --line value"),
        "stderr was: {stderr}"
    );
    assert!(stderr.contains("quantity must be"), "stderr was: {stderr}");
}

#[test]
fn a_domain_rule_violation_is_reported() {
    let output = run(&["total", "--line", "widget:0:1500"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    // The message comes from `app_core::Error::InvalidQuantity`. The CLI did
    // not reimplement the rule; it just let the error surface.
    assert!(
        stderr.contains("must be greater than zero"),
        "stderr was: {stderr}"
    );
}

#[test]
fn missing_arguments_are_a_usage_error() {
    let output = run(&["total"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("--line"), "stderr was: {stderr}");
}
