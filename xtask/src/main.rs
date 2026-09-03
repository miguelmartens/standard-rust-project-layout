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
    ci      fmt, prettier, clippy, tests, doctests, rustdoc -- what CI runs
    fmt     format in place: rustfmt, then prettier if it is installed
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
    cargo(&["fmt", "--all"])?;
    prettier(Mode::Write)
}

fn fmt_check() -> Result<(), DynError> {
    cargo(&["fmt", "--all", "--check"])?;
    prettier(Mode::Check)
}

/// Whether a formatter should rewrite files or only report on them.
#[derive(Debug, Clone, Copy)]
enum Mode {
    Check,
    Write,
}

/// Runs Prettier over the files rustfmt does not touch: Markdown, YAML, JSON.
///
/// **Optional by design.** `rustup` is the only hard requirement for working on
/// this repository, so a missing Prettier is a printed note rather than a
/// failure. CI always has Node, so CI always runs it -- which is what stops
/// `.prettierrc` from becoming configuration that nothing enforces.
///
/// There is deliberately no `npx --yes prettier` fallback. A build tool that
/// silently downloads a package from the network on first use behaves
/// differently offline, and behaving differently is worse than not running.
fn prettier(mode: Mode) -> Result<(), DynError> {
    let flag = match mode {
        Mode::Check => "--check",
        Mode::Write => "--write",
    };
    let root = workspace_root()?;

    let local = root.join("node_modules").join(".bin").join("prettier");
    let Some(program) = which("prettier").or_else(|| local.is_file().then_some(local)) else {
        eprintln!("note: prettier not found, skipping `prettier {flag} .`");
        eprintln!("note: `npm install --global prettier` to run it here; CI runs it regardless.");
        return Ok(());
    };

    eprintln!("$ prettier {flag} .");
    let status = Command::new(program)
        .current_dir(&root)
        .args([flag, "."])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("`prettier {flag} .` failed: {status}").into())
    }
}

/// A minimal `which`: the first executable called `program` on `PATH`.
///
/// Hand-rolled because this crate has no dependencies and should keep none.
/// The `PATHEXT` handling is what makes it correct on Windows, where the
/// executable is `prettier.cmd` and not `prettier` -- precisely the class of
/// bug a shell script gets wrong, and the reason this file is Rust.
fn which(program: &str) -> Option<PathBuf> {
    let path = env::var_os("PATH")?;
    let extensions: Vec<String> = env::var("PATHEXT")
        .map(|value| value.split(';').map(str::to_owned).collect())
        .unwrap_or_default();

    env::split_paths(&path).find_map(|directory| {
        let bare = directory.join(program);
        if bare.is_file() {
            return Some(bare);
        }
        extensions
            .iter()
            .map(|extension| directory.join(format!("{program}{extension}")))
            .find(|candidate| candidate.is_file())
    })
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

    eprint!("$ ");
    for (key, value) in env {
        eprint!("{key}={value} ");
    }
    eprintln!("cargo {}", args.join(" "));

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
