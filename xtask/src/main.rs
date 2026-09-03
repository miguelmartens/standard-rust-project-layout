//! Repository automation.
//!
//! # Why this exists instead of a Makefile
//!
//! The [cargo-xtask] pattern: "a way to extend stock, stable cargo with custom
//! commands (xtasks), written in Rust." The whole mechanism is the alias in
//! `.cargo/config.toml`:
//!
//! ```toml
//! [alias]
//! xtask = "run --package xtask --"
//! ```
//!
//! so `cargo xtask ci` becomes `cargo run --package xtask -- ci`.
//!
//! Four concrete reasons to prefer it over `make` / `just` / a `scripts/` folder:
//!
//! 1. **It runs on Windows.** No shell, no `sh`-isms, no separate `.ps1` copy
//!    that drifts. matklad again: it "can more easily be cross platform, as it
//!    doesn't use the shell."
//! 2. **It is type checked.** A typo in a Makefile recipe is discovered by the
//!    contributor who hits it. A typo here does not compile.
//! 3. **It needs nothing installed.** A contributor with `rustup` has
//!    everything. `make` is absent on a stock Windows box and `just` is one
//!    more thing to install and pin.
//! 4. **CI runs the same code you do.** When the CI job is `cargo xtask ci`,
//!    "works locally, fails in CI" stops being about the commands differing.
//!
//! The honest cost: the first `cargo xtask` in a clean checkout has to compile
//! this crate. That is why it has no dependencies — see `Cargo.toml`.
//!
//! # A note on workspace membership
//!
//! `xtask` is a member of the main workspace here, which keeps it under the
//! same lints, the same `Cargo.lock`, and the same `cargo fmt --all`. The
//! trade-off is that `cargo build --workspace` builds it too.
//!
//! The alternative — a second, excluded workspace under `xtask/` — keeps it out
//! of ordinary builds at the cost of a second lockfile and a second lint
//! configuration. Both are defensible. Pick one and write down which.
//!
//! [cargo-xtask]: https://github.com/matklad/cargo-xtask

use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

/// std-only error handling: this crate has no `anyhow`, and does not need one.
type DynError = Box<dyn std::error::Error>;

const HELP: &str = "\
cargo xtask <TASK>

TASKS:
    ci      fmt --check, clippy, tests, doctests, and rustdoc -- what CI runs
    fmt     format the workspace in place
    lint    clippy over every target, warnings denied
    test    tests and doctests
    help    print this message
";

fn main() -> ExitCode {
    let task = env::args().nth(1);

    let result = match task.as_deref() {
        Some("ci") => ci(),
        Some("fmt") => fmt(),
        Some("lint") => lint(),
        Some("test") => test(),
        Some("help" | "--help" | "-h") | None => {
            print!("{HELP}");
            return ExitCode::SUCCESS;
        }
        Some(unknown) => Err(format!("unknown task `{unknown}`\n\n{HELP}").into()),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

/// Everything CI runs, in the order CI runs it.
///
/// Ordering is deliberate: the cheap, high-signal checks fail first, so a
/// misformatted file does not cost a full test run to discover.
fn ci() -> Result<(), DynError> {
    fmt_check()?;
    lint()?;
    test()?;
    doc()
}

fn fmt() -> Result<(), DynError> {
    cargo(&["fmt", "--all"])
}

fn fmt_check() -> Result<(), DynError> {
    cargo(&["fmt", "--all", "--check"])
}

/// `--all-targets` covers tests, examples and benches, not just `src/`. Lints
/// that only fire in a test file are still lints.
fn lint() -> Result<(), DynError> {
    cargo(&[
        "clippy",
        "--workspace",
        "--all-targets",
        "--all-features",
        "--",
        "-D",
        "warnings",
    ])
}

/// Two invocations, because one does not do the job.
///
/// `--all-targets` deliberately *excludes* doctests. Running only
/// `cargo test --all-targets` silently skips every example in every doc
/// comment, which is usually where a library's most-read code lives.
fn test() -> Result<(), DynError> {
    cargo(&["test", "--workspace", "--all-targets"])?;
    cargo(&["test", "--workspace", "--doc"])
}

/// Documentation that does not build is a broken link away from being wrong.
/// `--no-deps` keeps the check about this workspace.
///
/// `RUSTDOCFLAGS=-D warnings` is set here rather than left to CI, because a
/// broken intra-doc link should fail on the machine that wrote it.
fn doc() -> Result<(), DynError> {
    cargo_with_env(
        &["doc", "--workspace", "--no-deps", "--all-features"],
        &[("RUSTDOCFLAGS", "-D warnings")],
    )
}

/// Runs `cargo` with the given arguments from the workspace root.
///
/// Uses `$CARGO` when set, so the task runs under the same toolchain that
/// invoked it — `cargo +nightly xtask ci` stays on nightly all the way down.
fn cargo(args: &[&str]) -> Result<(), DynError> {
    cargo_with_env(args, &[])
}

/// As [`cargo`], with extra environment variables for the child process.
fn cargo_with_env(args: &[&str], env: &[(&str, &str)]) -> Result<(), DynError> {
    let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));

    for (key, value) in env {
        eprint!("$ {key}={value} ");
    }
    eprintln!("$ cargo {}", args.join(" "));

    let status = Command::new(cargo)
        .current_dir(workspace_root()?)
        .args(args)
        .envs(env.iter().copied())
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("`cargo {}` failed: {status}", args.join(" ")).into())
    }
}

/// The directory holding the workspace `Cargo.toml`.
///
/// `CARGO_MANIFEST_DIR` is `<root>/xtask` at compile time, so the parent is the
/// root. Resolving it this way means `cargo xtask ci` behaves the same from any
/// subdirectory of the repository.
fn workspace_root() -> Result<PathBuf, DynError> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| format!("`{}` has no parent directory", manifest_dir.display()).into())
}
