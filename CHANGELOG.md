# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] — 2026-09-03

### Added

- Workspace skeleton: virtual manifest, `resolver = "3"`, edition 2024,
  MSRV 1.85, shared metadata, dependencies and lints.
- `app-core`: the order and customer domain, `thiserror` error type, `serde`
  configuration shape.
- `app-cli`: the `app` binary, `clap` derive, `anyhow` error handling.
- `app-macros`: a dependency-free procedural macro crate.
- `xtask`: `ci`, `fmt`, `lint` and `test` tasks, no dependencies.
- CI: `check`, `msrv` and `deny` jobs; weekly schedule.
- Documentation: the layout README, an ADR template and one filled-in ADR, and
  a README in every directory.

[Unreleased]: https://github.com/example/rust-project-layout/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/example/rust-project-layout/releases/tag/v0.1.0

---

## Why this file exists, and what Rust adds to the usual advice

**`cargo` does not generate release notes.** There is no `cargo changelog`, and
`git log` is not one: commit messages are written for reviewers, a changelog is
written for the people who have to decide whether to upgrade.

**Version numbers here are Cargo's version numbers.** With
`[workspace.package] version`, every crate in this workspace releases together
under one number, so there is one changelog. Workspaces that version crates
independently need a changelog per crate — decide which you are before the first
release, because switching later means rewriting history.

**Bumping the MSRV is a `minor` bump, not a `major` one.** _M-MSRV_: raising
the minimum supported Rust version "warrants only minor version increments",
because the ecosystem already depends on reasonably modern compilers through
transitive dependencies. Still record it — it is the single most common reason a
downstream build breaks after an upgrade.

**Adding a variant to a `#[non_exhaustive]` enum is a minor change**; adding one
to an ordinary public enum is a major change. Both are easy to do by accident.
[`cargo-semver-checks`](https://github.com/obi1kenobi/cargo-semver-checks) will
catch most of it mechanically, and belongs in CI once the crate is published.

**A new default Cargo feature is additive; removing or renaming one is
breaking.** Features are part of the public API, and are the part most often
changed without anyone thinking of it as an API change.

If maintaining this by hand is not going to happen,
[`release-plz`](https://release-plz.dev/) and
[`cargo-release`](https://github.com/crate-ci/cargo-release) automate it from
conventional commits. An automated changelog that exists beats a hand-written
one that stopped in 2024.
