//! The `app` executable.
//!
//! # Why this file is four lines long
//!
//! Everything a binary crate contains is unreachable from anywhere else. There
//! is no library target, so no other crate can `use` it, no integration test
//! can call it, and no benchmark can measure it. Code put here is code that can
//! only ever be tested through a process boundary.
//!
//! So the binary does the two things only a binary can do — parse `argv` and
//! decide the exit code — and hands off immediately. The rules live in
//! `app-core`, where they can be unit tested in microseconds.
//!
//! The Go habit of a thin `cmd/foo/main.go` is exactly the same instinct. Rust
//! just expresses it as `src/bin/` or a dedicated `-cli` crate rather than a
//! `cmd/` directory.
//!
//! # Returning `Result` from `main`
//!
//! `fn main() -> anyhow::Result<()>` makes the runtime print the error chain to
//! stderr and exit non-zero. It costs one `?` instead of a `match` around every
//! call, and it is why this file has no error handling in it at all.
//!
//! Note the `Debug` formatting the runtime uses is what makes `anyhow`'s
//! context chain show up. That is a deliberate `anyhow` design, not an accident.

mod cli;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    cli::Cli::parse().run()
}
