# 0001. Use a Cargo workspace with flat sibling crates

- **Status:** Accepted
- **Date:** 2026-09-03
- **Deciders:** the rust-project-layout authors

## Context

This repository produces three artefacts: domain logic, a command line binary,
and a procedural macro. The proc macro cannot share a package with the other
two — a crate with `proc-macro = true` is compiled for the host, loaded into
`rustc`, and may export nothing but macros. So the repository contains at least
two crates whether we like it or not.

Once there is more than one crate, the versions of `serde`, `clap` and the rest
have to agree. Left to individual manifests they drift, and the first symptom is
usually two incompatible versions of the same type in one binary, reported as a
type error that names the same type twice.

We also want a reader arriving from `golang-standards/project-layout` to find a
structure they can navigate without reading Cargo's documentation first.

## Decision

We will use a single Cargo workspace with a **virtual manifest** at the
repository root, and place every crate as a flat sibling under `crates/`.

Shared metadata goes in `[workspace.package]`, dependency versions in
`[workspace.dependencies]` — including our own crates, with both `path` and
`version` — and lint configuration in `[workspace.lints]`. Members inherit with
`field.workspace = true` and `[lints] workspace = true`.

## Alternatives considered

**A single package with no workspace.** Simplest possible layout, and the right
answer for most projects. Rejected only because the proc macro forces a second
crate. Had `app-macros` not existed, this would have won and `crates/` would not
exist.

**Separate repositories per crate.** Genuine isolation and independent release
cadence. Rejected: an atomic change across `app-core` and `app-cli` becomes two
pull requests and a version bump, and no CI run ever tests the combination that
users will actually get.

**A workspace with nested crates** (`crates/app/core/`, `crates/app/macros/`).
Rejected. It reads as a hierarchy that Cargo does not implement — `app-core` has
no special relationship to `app` that the build system understands — and it
contradicts *M-CRATES-FLAT-FOLDER*. Name prefixes convey the grouping at no cost.

**A root package plus workspace members**, with `app-core` at the repository
root and the others under `crates/`. Rejected: it privileges one crate in the
directory layout, and every time a fourth crate is added someone has to decide
whether it is important enough to be promoted. A virtual manifest has no such
argument, because there is no privileged position to argue about.

**Path dependencies between siblings** (`app-core = { path = "../app-core" }`).
Rejected: it works locally and then produces a crate that cannot be published,
because a published crate needs a version requirement its consumers can resolve.

## Consequences

- One `Cargo.lock` and one `target/` directory for the whole repository.
  Contributors build everything once instead of once per crate.
- Adding a crate is three edits: the directory, `[workspace] members`, and
  `[workspace.dependencies]`. Forgetting the third is the common mistake and
  shows up as an unpublishable crate later, not as a build failure now.
- `cargo build --workspace` builds `xtask` as well. Accepted; the alternative is
  a second, excluded workspace with its own lockfile and lint configuration.
- Bumping a shared dependency is one line, and it is all-or-nothing. A crate
  that needs to lag behind has to say so explicitly in its own manifest, which
  is visible in review — the intended outcome.
