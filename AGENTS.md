# AGENTS.md

Instructions for coding agents working in this repository.
[CONTRIBUTING.md](CONTRIBUTING.md) carries the same rules for humans, with the
reasoning; this file is the short, operational version. Where they disagree,
CONTRIBUTING.md is right and this file is stale — fix it.

## What this repository is

A **reference document about Rust project layout**, plus a working workspace that
demonstrates it and keeps its claims honest.

The consequence, and the thing most likely to be got wrong: **the README is the
deliverable.** The code exists to be an example. A change to the code that is not
reflected in the prose is half a change, and a claim in the prose that the code
does not demonstrate is a bug.

## Commands

```console
cargo xtask ci      # THE GATE: fmt, prettier, clippy, tests, doctests, rustdoc
cargo xtask fmt     # format in place
make ci             # identical; the Makefile only forwards
```

**Never report work as complete without a passing `cargo xtask ci`.** Individual
cargo commands are not sufficient — in particular `cargo test --all-targets` does
**not** run doctests, which is why `ci` runs `cargo test --doc` separately.

Also verify when relevant:

```console
cargo +1.85 check --workspace --all-targets --locked   # the declared MSRV
cargo deny check                                       # licences, advisories
```

Prettier and pre-commit are optional; `xtask` skips them with a notice when they
are absent. Do not add them as hard requirements. `rustup` is the only one.

## Cargo-defined. Do not restructure.

- `src/`, `src/bin/`, `tests/`, `benches/`, `examples/` are found by Cargo's
  target auto-discovery. Renaming any of them silently breaks the build — a
  renamed `tests/` makes `cargo test` run nothing **and report success**.
- Crates are flat siblings in `crates/`. Never nest a crate inside another crate,
  and never inside a `src/`.
- Sibling crates depend on each other with `dep.workspace = true`, never
  `path = "../dep"`. Every workspace crate is listed in
  `[workspace.dependencies]` with both `path` and `version`.

## House rules an agent is likely to break

- **Lints live in `[workspace.lints]`.** Never add `#![deny(...)]` or
  `#![allow(...)]` to a crate root. To silence a lint, allow it as narrowly as
  possible — usually an inner attribute on a `#[cfg(test)] mod tests` — and write
  the reason in a comment.
- **Never restate a default in a config file.** `rustfmt.toml` is one line
  because rustfmt's defaults are the Rust Style Guide. Before adding an option
  anywhere, verify it changes the output; if you keep it, say in a comment
  whether it overrides a default or deliberately restates one.
- **`unwrap` / `expect` are warn-level and belong only in tests.** In library
  code, return a `Result`. `thiserror` for libraries, `anyhow` for binaries.
- **`clippy::pedantic` is on.** Public functions returning `Result` need an
  `# Errors` section; public getters usually need `#[must_use]`.
- **`xtask` has zero dependencies.** Keep it that way; it is compiled before a
  contributor's first check.
- **The `Makefile` contains no automation.** Every recipe is one line forwarding
  to `cargo xtask` or `cargo`. If a recipe needs a second line, it belongs in
  `xtask`.
- **`Cargo.lock` is committed** and CI runs `--locked`. Stage lockfile changes in
  the same commit as the manifest change that caused them.
- **YAML files use `.yaml`.** The single exception is `.github/dependabot.yml`,
  because GitHub documents that exact name and a misnamed config fails silently.
- **Every directory that teaches something has a `README.md`.** Source trees do
  not — module docs do that job. Do not put a `README.md` inside `src/`.
- **Do not add a crate or a dependency** without a reason from
  [crates/README.md](crates/README.md). "It feels tidier" is not one.

## Writing prose here

- Comments and docs explain **why**, not what. The code already says what.
- **No comparisons to other languages' layouts.** The README credits
  `golang-standards/project-layout` in one line and says nothing else about it.
  Do not reintroduce translation tables or "coming from X" sections.
- British spelling: `organise`, `normalise`, `behaviour`, and `licence` for the
  noun — but `license` where it is a Cargo manifest key or an SPDX identifier.
  **Blockquotes from external sources are verbatim** and keep their original
  spelling; do not "correct" them.
- Markdown is hand-wrapped at roughly 80 columns. Prettier runs with
  `proseWrap: preserve` and will **not** rewrap, so a botched string replacement
  leaves a fused over-long line rather than being tidied away. After editing
  prose, check for lines much longer than the surrounding ones.
- Cite primary sources inline. Where an opinionated source disagrees with the
  Cargo Book, the Cargo Book wins and the text says so.

## Deliberate oddities — do not "fix" these

| Looks wrong                                                                          | Why it is that way                                                          |
| ------------------------------------------------------------------------------------ | --------------------------------------------------------------------------- |
| `benches/order-total.rs` hand-rolls a timing loop with `harness = false`             | The repo has no benchmarking dependency on purpose. Do not add criterion.   |
| `app-cli` has no lib target and its tests spawn the binary                           | That is the teaching point about binaries, not an oversight.                |
| `app-macros` uses raw `proc_macro`, no `syn`/`quote`                                 | Keeps the dependency tree honest. The docs say real macros should use them. |
| `tests/order-lifecycle.rs` contains commented-out assertions marked DOES NOT COMPILE | Deliberate: it demonstrates what an integration test cannot reach.          |
| `docs/adr/0000-template.md` is not a decision                                        | `0000` is reserved for the template. There is no ADR 0000.                  |
| ADRs are never edited                                                                | A changed decision gets a new ADR that supersedes the old one.              |
| `deny.toml` allows licences nothing uses                                             | An allow-list is a set of licences we accept, not an inventory.             |

## Before reporting done

1. `cargo xtask ci` passes.
2. If you changed a manifest, the declared MSRV still builds:
   `cargo +1.85 check --workspace --all-targets --locked`.
3. If you changed prose, relative links and README anchors still resolve, and the
   repository tree in the README still lists every root file.
4. If you changed behaviour, the README says what the code now does.
