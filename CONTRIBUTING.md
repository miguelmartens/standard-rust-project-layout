# Contributing

## Setup

```console
$ git clone https://github.com/example/rust-project-layout
$ cd rust-project-layout
$ cargo xtask ci
```

That is the whole setup. [`rust-toolchain.toml`](rust-toolchain.toml) pins the
channel and pulls in `rustfmt` and `clippy`, and rustup honours it on the first
`cargo` invocation — no `rustup override`, no version to install by hand.

You need `rustup`. You do not need `make`, `just`, or a shell that understands
`set -euo pipefail`.

Two optional extras, neither of which anything in CI depends on:

```console
$ cargo xtask hooks   # install the git hooks (needs pre-commit)
```

[`.pre-commit-config.yaml`](.pre-commit-config.yaml) formats and sanity-checks
staged files on commit, and runs clippy and the tests on push. It exists purely
to save you a CI round trip; every expensive hook in it runs a command CI runs
anyway, so skipping it costs you latency and nothing else. Install `pre-commit`
with `uv tool install pre-commit` or `brew install pre-commit`.

Node is **optional**. Prettier formats the Markdown, YAML and JSON that rustfmt
does not touch; `cargo xtask fmt` runs it when it is on `PATH` and prints a skip
notice when it is not, so you are never blocked on it. CI always runs it, so a
Markdown table you did not align gets caught there rather than never. Install it
with `npm install --global prettier` if you would rather see it locally.

## Before you open a pull request

```console
$ cargo xtask ci
```

This runs exactly what CI runs, in the same order:

1. `cargo fmt --all --check`
2. `prettier --check .` — skipped locally if Prettier is not installed
3. `cargo clippy --workspace --all-targets --all-features -- -D warnings`
4. `cargo test --workspace --all-targets`
5. `cargo test --workspace --doc` — separately, because step 4 skips doctests
6. `cargo doc --workspace --no-deps --all-features`

Cheap checks fail first, so a misformatted file does not cost a full test run to
discover.

`make ci` does the same thing if that is what your fingers type. The
[`Makefile`](Makefile) is aliases only — one line per recipe, forwarding to
`cargo xtask`. New automation goes in [`xtask/`](xtask/), never into a recipe.

`cargo xtask fmt` fixes formatting in place, for both formatters.
`cargo clippy --fix` fixes many lints in place.

## House rules

**Do not argue with `rustfmt`, or with Prettier.** [`rustfmt.toml`](rustfmt.toml)
is two lines and should stay that way; [`.prettierrc`](.prettierrc) sets four
options and one override. Its defaults are the Rust style guide; every option added
is a permanent tax on every reader who has internalised the default output. If a
line formats badly, the line is usually the problem.

**Lints are configured in one place.** `[workspace.lints]` in the root
[`Cargo.toml`](Cargo.toml). Do not add `#![deny(...)]` to a crate root — it
applies to one crate, drifts from the others, and is invisible from the place
people look.

To allow a lint, allow it as narrowly as possible and say why:

```rust
// Test code: a panic *is* the failure report.
#![allow(clippy::unwrap_used)]
```

A workspace-wide allow needs to be justified in the pull request, because it is
a decision about every crate, forever.

**`unwrap()` and `expect()` are warnings, not errors, and that is deliberate.**
They are correct in tests, in build scripts, and in `main` where the process is
about to exit anyway. In library code they are a promise that this can never
fail — if you can prove it, write the proof in a comment next to the allow. If
you cannot, return a `Result`.

**Do not add a crate without a reason from
[`crates/README.md`](crates/README.md).** "It feels tidier" is not one. A crate
boundary costs a manifest, two workspace table entries, a public API you now
have to keep stable, and a compile unit.

**Do not add a dependency to `xtask`.** Every one of them is compile time paid
by every contributor before their first check.

**`Cargo.lock` is committed.** Include it in your commit when it changes. CI runs
with `--locked`, so a stale lockfile fails the build rather than being silently
regenerated.

## Commits

Keep them logically separate: a formatting change, a refactor and a behaviour
change in one commit is three things nobody can review or revert independently.

[Conventional Commits](https://www.conventionalcommits.org/) is not enforced
here, but it is what the changelog tooling in the Rust ecosystem
(`release-plz`, `cargo-release`) understands, so following it costs nothing and
may save work later.

## Tests

Put the test where it can see what it needs to test.

- **Unit tests** go in `#[cfg(test)] mod tests` in the same file as the code.
  They can reach private state. Most tests are these.
- **Integration tests** go in `tests/`. Each file is a separate crate and sees
  only the public API. Use them to check that the API is usable, not that the
  implementation is correct.
- **Doc examples** in `///` comments are compiled _and run_ by
  `cargo test --doc`. They are the best documentation you can write, because
  they cannot go stale.

New behaviour needs a test. A bug fix needs a test that fails without the fix —
if writing that test is hard, that is usually information about the design.

## Documentation

Public items need doc comments; `clippy::pedantic` will ask for a `# Errors`
section on anything returning `Result`, and that is a reasonable thing to be
asked for.

Comments in this repository explain **why**, not what. The code already says
what it does. Comments that restate it go stale and mislead.

## Bumping the MSRV

Change `rust-version` in `[workspace.package]`. That is the only place it
appears — CI reads it from there, and clippy reads it from there.

Raising the MSRV is a **minor** version bump, not a major one (_M-MSRV_). Keep
it a few releases behind current stable, and note it in
[`CHANGELOG.md`](CHANGELOG.md): it is the most common reason a downstream build
breaks after an upgrade.

## Licence

Unless you state otherwise, any contribution you intentionally submit for
inclusion in this work, as defined in the Apache-2.0 licence, shall be dual
licensed as `MIT OR Apache-2.0`, without any additional terms or conditions.

(That paragraph is the ecosystem-standard notice. See [`LICENSE`](LICENSE) for
why Rust projects dual-license.)
