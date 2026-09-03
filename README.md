# Standard Rust Project Layout

A reference layout for Rust projects, and a working workspace that demonstrates
it. `cargo build --workspace` succeeds; `cargo xtask ci` passes.

If you arrived here from [`golang-standards/project-layout`][go-layout], start
with [Coming from Go](#coming-from-go). If you are about to lay out your first
Rust project, read [Part 1](#part-1--what-cargo-defines) and then stop; Part 2 is
for when you have more than one crate.

## Contents

- [Read this first: Rust is not Go](#read-this-first-rust-is-not-go)
- [Part 1 — What Cargo defines](#part-1--what-cargo-defines)
  - [The canonical package layout](#the-canonical-package-layout)
  - [Target auto-discovery](#target-auto-discovery)
  - [Naming: kebab-case targets, snake_case modules](#naming-kebab-case-targets-snake_case-modules)
  - [Module layout inside `src/`](#module-layout-inside-src)
  - [Unit tests and integration tests are different things](#unit-tests-and-integration-tests-are-different-things)
  - [Privacy is a language feature](#privacy-is-a-language-feature)
  - [Commit `Cargo.lock`](#commit-cargolock)
- [Part 2 — What Cargo leaves open](#part-2--what-cargo-leaves-open)
  - [One workspace, crates as flat siblings](#one-workspace-crates-as-flat-siblings)
  - [Inherit everything from the workspace](#inherit-everything-from-the-workspace)
  - [Lints belong in `[workspace.lints]`](#lints-belong-in-workspacelints)
  - [Edition and MSRV](#edition-and-msrv)
  - [Errors: `thiserror` for libraries, `anyhow` for binaries](#errors-thiserror-for-libraries-anyhow-for-binaries)
  - [When *not* to split into crates](#when-not-to-split-into-crates)
  - [Automation: `xtask`, not `make`](#automation-xtask-not-make)
  - [Directories the ecosystem has no convention for](#directories-the-ecosystem-has-no-convention-for)
- [Coming from Go](#coming-from-go)
- [Anti-patterns](#anti-patterns)
- [This repository](#this-repository)
- [References](#references)

---

## Read this first: Rust is not Go

**Go has no official project layout.** That vacuum is why
[`golang-standards/project-layout`][go-layout] exists. It is also why the repo is
widely criticised: it is one person's collection of patterns presented under a
name — `golang-standards` — that implies an authority it does not have. The repo
itself says so, at the top: *"This is **NOT an official standard defined by the
core Go dev team**. This is a set of common historical and emerging project
layout patterns in the Go ecosystem."*

**Rust does have an official layout.** [Cargo defines it][cargo-layout], enforces
it through [target auto-discovery][cargo-targets], and every Rust developer
already expects it. It is not advice. `src/lib.rs` is the library target because
Cargo looks there; move it and there is no library. Put integration tests in
`test/` instead of `tests/` and `cargo test` will not find them.

So this repository is not the Rust translation of the Go one. Inventing a
competing directory scheme in Rust does not produce a debatable convention, it
produces a broken build. What is left over is real, but it is a much smaller
territory than the Go repository covers.

Hence two halves, deliberately separated:

| | |
|---|---|
| **[Part 1](#part-1--what-cargo-defines)** | **What Cargo defines.** Quoted from the Cargo Book. Non-negotiable. Deviating breaks tooling, not taste. |
| **[Part 2](#part-2--what-cargo-leaves-open)** | **What Cargo leaves open.** Workspace organisation, docs, CI, deployment, tooling config. Recommendations, with reasons — argue with them. |

Where an opinionated source disagrees with the Cargo Book, the Cargo Book wins,
and this document says so.

---

# Part 1 — What Cargo defines

**Everything in this section is fixed.** Not by convention, by the build system.

## The canonical package layout

Quoted verbatim from [the Cargo Book][cargo-layout]:

```text
.
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs
```

> - `Cargo.toml` and `Cargo.lock` are stored in the root of your package
>   (*package root*).
> - Source code goes in the `src` directory.
> - The default library file is `src/lib.rs`.
> - The default executable file is `src/main.rs`.
>   - Other executables can be placed in `src/bin/`.
> - Benchmarks go in the `benches` directory.
> - Examples go in the `examples` directory.
> - Integration tests go in the `tests` directory.

And for targets that outgrow one file:

> If a binary, example, bench, or integration test consists of multiple source
> files, place a `main.rs` file along with the extra *modules* within a
> subdirectory of the `src/bin`, `examples`, `benches`, or `tests` directory.
> The name of the executable will be the directory name.

That is the entire mandatory layout. It is smaller than Go's, and it is
mandatory rather than suggested.

**A single-crate project should look exactly like that and nothing more.** No
`crates/`, no workspace, no `xtask`. Everything in Part 2 is what you reach for
when a project outgrows this, and most projects never do.

## Target auto-discovery

The directory names above are not decoration — Cargo scans for them.

| Target | Discovered at | Target name |
|---|---|---|
| library | `src/lib.rs` | package name, dashes → underscores |
| default binary | `src/main.rs` | package name, dashes kept |
| extra binaries | `src/bin/*.rs`, `src/bin/*/main.rs` | file stem, or directory name |
| examples | `examples/*.rs`, `examples/*/main.rs` | file stem, or directory name |
| integration tests | `tests/*.rs`, `tests/*/main.rs` | file stem, or directory name |
| benchmarks | `benches/*.rs`, `benches/*/main.rs` | file stem, or directory name |

> For [auto discovered] targets, it defaults to the directory or file name.
>
> — [Cargo Targets][cargo-targets]

This is the mechanical reason a custom directory scheme is not merely
unconventional in Rust. Rename `tests/` to `test/` and `cargo test` runs nothing
and reports success.

Auto-discovery can be switched off per target type with `autolib`, `autobins`,
`autoexamples`, `autotests` and `autobenches`, and individual targets can be
declared explicitly with a `path`:

```toml
[[bin]]
name = "app"          # the binary users type
path = "src/main.rs"  # required: Cargo only infers this when name == package name
```

Both are escape hatches for real needs — a binary whose name differs from its
package, a generated target — not a licence to reorganise. See
[`crates/app-cli/Cargo.toml`](crates/app-cli/Cargo.toml) for the one case in
this repository.

## Naming: kebab-case targets, snake_case modules

Two different rules, and mixing them up is the most common naming mistake.

**Modules and crates are `snake_case`**, per [RFC 430][rfc430]:

| Item | Convention |
|---|---|
| Crates | `snake_case` (but prefer a single word) |
| Modules | `snake_case` |
| Types, traits, enum variants | `UpperCamelCase` |
| Functions, methods, local variables | `snake_case` |
| Statics and constants | `SCREAMING_SNAKE_CASE` |
| Type parameters | concise `UpperCamelCase`, usually `T` |
| Lifetimes | short and lowercase, `'a` |

Acronyms count as one word: `Uuid`, not `UUID`; `is_xid_start`, not `is_XID_start`.

**Target names are kebab-case.** Package names on crates.io are conventionally
hyphenated (`app-core`, `serde-json`… ) and so are the file names in `tests/`,
`examples/`, `benches/` and `src/bin/`, because the file name *is* the target
name:

```text
tests/order-lifecycle.rs   →  cargo test --test order-lifecycle
examples/simple.rs         →  cargo run --example simple
src/bin/app-admin.rs       →  cargo run --bin app-admin
```

These files are crate roots, not modules, which is why the module rule does not
apply to them. Cargo replaces hyphens with underscores when the package name
becomes a library's crate name: package `app-core` is `use app_core::…`.

One more from the [API guidelines][api-naming]: *"Crate names should not use
`-rs` or `-rust` as a suffix or prefix. Every crate is Rust! It serves no
purpose to remind users of this constantly."*

## Module layout inside `src/`

Cargo owns the directory names; the file layout *inside* `src/` is the module
system's business, and there the language gives you two forms.

**Prefer `foo.rs` + `foo/` over `foo/mod.rs`.** From the [edition guide][paths]:

> In Rust 2018 the restriction that a module with submodules must be named
> `mod.rs` is lifted. `foo.rs` can just be `foo.rs`, and the submodule is still
> `foo/bar.rs`. This eliminates the special name, and if you have a bunch of
> files open in your editor, you can clearly see their names, instead of having
> a bunch of tabs named `mod.rs`.

```text
src/                          src/
├── lib.rs                    ├── lib.rs
├── domain.rs        ✅        └── domain/           ❌ (legacy)
└── domain/                       ├── mod.rs
    ├── customer.rs               ├── customer.rs
    └── order.rs                  └── order.rs
```

Both still compile. The tab-title argument is the whole argument, and it is
enough. **Mixing the two forms in one project is the only genuinely bad option.**

**Use `lib.rs` as a façade.** Declare modules privately, re-export the public
API:

```rust
// crates/app-core/src/lib.rs
mod config;   // private
mod domain;   // private
mod error;    // private

pub use crate::config::Config;
pub use crate::domain::{Customer, CustomerId, Order, OrderId, OrderLine, OrderStatus};
pub use crate::error::{Error, Result};
```

Callers write `app_core::Order`. Nobody writes — or can write —
`app_core::domain::order::Order`. The internal module tree is therefore not part
of the public API, and splitting `domain::order` into three modules next month is
a refactor rather than a breaking change.

This is the single highest-leverage layout decision in the whole document, and
it costs six lines.

## Unit tests and integration tests are different things

They are not two styles for the same job. They see different code and break for
different reasons, and a project wants both.

| | Unit test | Integration test |
|---|---|---|
| Lives in | `#[cfg(test)] mod tests`, same file as the code | `tests/*.rs` |
| Compiled as | part of the crate | **a separate crate** |
| Can see | private fields and private functions | only what `lib.rs` re-exports |
| Answers | "is the implementation right?" | "is the API usable?" |
| Breaks when | internals change | the public API changes |

From [the Book][book-tests]:

> You'll put unit tests in the *src* directory in each file with the code that
> they're testing. The convention is to create a module named `tests` in each
> file to contain the test functions and to annotate the module with `cfg(test)`.

and:

> Each file in the *tests* directory is a separate crate, so we need to bring our
> library into each test crate's scope. […] They use your library in the same way
> any other code would, which means they can only call functions that are part of
> your library's public API.

This repository makes the contrast concrete on purpose. Compare the unit tests
at the bottom of [`crates/app-core/src/domain/order.rs`](crates/app-core/src/domain/order.rs),
which assert on the private `status` and `lines` fields, with
[`crates/app-core/tests/order-lifecycle.rs`](crates/app-core/tests/order-lifecycle.rs),
which contains those same assertions commented out and marked *DOES NOT COMPILE*.

**There is no mirrored `test/` tree.** Rust does not have one and does not want
one. Tests either sit beside the code or sit at the API boundary; there is no
third place for them.

Two practical notes:

- Shared helpers for integration tests go in **`tests/common/mod.rs`**, not
  `tests/common.rs`. Files in subdirectories of `tests/` are not compiled as
  separate crates, so `tests/common.rs` would run as its own empty test suite
  and appear in the output.
- **`cargo test --all-targets` does not run doctests.** This surprises everyone
  once. Run `cargo test --doc` as a second command — see
  [`xtask/src/main.rs`](xtask/src/main.rs) and
  [`.github/workflows/ci.yaml`](.github/workflows/ci.yaml), which both do.

## Privacy is a language feature

Rust has no `internal/` directory, needs none, and would be worse for having one.

```rust
mod domain;                    // invisible outside the crate
pub(crate) fn helper() {}      // this crate only
pub(super) fn parent_only() {} // parent module only
pub fn api() {}                // actually public
```

The compiler enforces every one of those. A directory name cannot be enforced by
anything except a linter and a code review, which is precisely why Go needed the
compiler to special-case the name `internal`.

If you want a boundary stronger than a module — one the compiler will refuse to
let anyone cross even inside the same project — the tool for that is a separate
crate, not a directory. Cargo also forbids cyclic crate dependencies, so a crate
boundary is the only way to make "A may use B, but B may never use A"
structurally true.

## Commit `Cargo.lock`

**Commit it. For binaries and for libraries.** `cargo new` does this by default,
and the reasons in the [Cargo FAQ][faq-lock] apply to both:

> Deterministic builds help with
>
> - Running `git bisect` to find the root cause of a bug
> - Ensuring CI only fails due to new commits and not external factors
> - Reducing confusion when contributors see different behavior as compared to
>   other contributors or CI

The old advice — *binaries commit it, libraries do not* — came from a real
observation and drew the wrong conclusion. The observation is that
`Cargo.lock` "does not affect the consumers of your package, only `Cargo.toml`
does that"; a library's lockfile is ignored by everyone downstream. The wrong
conclusion is that it is therefore useless. It is not useless to *you*: it makes
your own CI reproducible, which is where you spend your time.

What you lose by committing it is coverage of newer dependency versions. Get
that back with a **scheduled job that runs `cargo update` first** —
["Verifying Latest Dependencies"][ci-latest] in the Cargo Book — rather than by
leaving every run of every CI job non-reproducible. Run CI with `--locked` so a
stale lockfile fails loudly instead of being silently regenerated.

The lockfile is also what makes an MSRV job meaningful. With `resolver = "3"`
Cargo picks dependency versions compatible with your `rust-version` when it
writes the lockfile, so `cargo check --locked` on the old toolchain tests the
versions your users will actually get.

---

# Part 2 — What Cargo leaves open

**Everything below is a recommendation, not a rule.** Cargo has no opinion here,
so the opinions are mine and the sources are cited. Disagree where you have a
reason; the reason is the part that matters.

Most of this section only applies once a project has more than one crate. If
yours has one, [Part 1](#part-1--what-cargo-defines) was the whole document.

## One workspace, crates as flat siblings

Two or more related crates means one workspace. From the
[Microsoft Rust guidelines][ms-project]:

- ***M-CARGO-WORKSPACE*** — "Common settings come from the workspace
  `Cargo.toml`."
- ***M-CRATES-IN-WORKSPACE*** — "The workspace lists and versions all crates."
- ***M-CRATES-FLAT-FOLDER*** — "All crates are siblings in one folder."

```text
✅  crates/app-core/            ❌  crates/app/core/
    crates/app-cli/                crates/app-cli/src/macros/
    crates/app-macros/             app-core/crates/helper/
```

**Flat.** One directory, one level, every crate a sibling. Nesting a crate inside
another crate — and especially inside its `src/` — is never acceptable: Cargo
will not find it, `cargo build` will not build it, and no reader expects it. Add
one level of grouping (`crates/server/`, `crates/client/`) only past roughly one
to two dozen crates, where the flat list genuinely stops being readable.

**Relationships live in names, not paths.** `app`, `app-core`, `app-macros` sort
together and read as a family. That is the entire mechanism, and it is enough,
because there is no relationship between them that the build system understands
anyway.

**Use a virtual manifest at the root.** A `Cargo.toml` with `[workspace]` and no
`[package]`:

> Alternatively, a `Cargo.toml` file can be created with a `[workspace]` section
> but without a `[package]` section. This is called a *virtual manifest*.
>
> — [Cargo Workspaces][cargo-workspaces]

The alternative — promoting one crate to the repository root — privileges it in
the directory layout and starts an argument every time a crate is added. A
virtual manifest has no privileged position to argue about.

`resolver` must be set explicitly in a virtual workspace, because there is no
`package.edition` to infer it from. Use `resolver = "3"`: it is MSRV-aware and
will not lock you to a dependency version that needs a newer compiler than you
claim to support.

## Inherit everything from the workspace

Three tables in the root manifest, one line per crate to opt in.

```toml
# root Cargo.toml
[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.85"
license = "MIT OR Apache-2.0"
repository = "https://github.com/example/rust-project-layout"

[workspace.dependencies]
app-core = { path = "crates/app-core", version = "0.1.0" }   # both, always
anyhow = "1.0.104"
serde = { version = "1.0.229", features = ["derive"] }
```

```toml
# crates/app-cli/Cargo.toml
[package]
name = "app-cli"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[lints]
workspace = true

[dependencies]
app-core.workspace = true    # not { path = "../app-core" }
anyhow.workspace = true
```

**Sibling crates get `path` *and* `version`.** *M-CRATES-IN-WORKSPACE*: instead
of `sibling.path = "../sibling"`, intra-workspace dependencies resolve via
`sibling.workspace = true` with the canonical version declared centrally. `path`
is what a local build uses; `version` is what a published crate records for its
consumers. Omit `version` and the crate cannot be published — a mistake that
surfaces months later, at release time, and never as a build failure.

**A new crate's manifest should be almost entirely inheritance.** Anything it
states for itself is a claim that it differs from the rest of the workspace, and
that claim should be true and visible in review.

Adding a crate is three edits: the directory, `[workspace] members`, and
`[workspace.dependencies]`. Forgetting the third is the common mistake.

## Lints belong in `[workspace.lints]`

Not scattered as `#![deny(...)]` across crate roots, where they apply to one
crate, drift out of sync with the others, and are invisible from the place people
look.

```toml
[workspace.lints.rust]
unsafe_code = "forbid"
missing_debug_implementations = "warn"
unreachable_pub = "warn"

[workspace.lints.clippy]
all = { level = "warn", priority = -1 }
pedantic = { level = "warn", priority = -1 }
unwrap_used = "warn"
expect_used = "warn"
todo = "warn"
```

The `priority = -1` is load-bearing. From the [manifest reference][manifest]:
*"lower (particularly negative) numbers have lower priority, being overridden by
higher numbers"*. Groups go below the individual lints so that a specific lint
can override a group it belongs to. Without it, `clippy::pedantic` and
`clippy::unwrap_used` fight and Cargo reports an ambiguity error.

`forbid` rather than `deny` for `unsafe_code`: `forbid` cannot be lifted by a
local `#[allow]`, so a crate that genuinely needs `unsafe` has to override it in
its own manifest — visibly, in review.

`unwrap_used` and `expect_used` are `warn`, not `deny`, on purpose. They are
correct in tests, build scripts, and `main`. Allow them narrowly and say why:

```rust
#[cfg(test)]
mod tests {
    // Test code: a panic *is* the failure report.
    #![allow(clippy::unwrap_used)]
```

Keep `clippy.toml` for thresholds only — it tunes lints, it cannot enable them.
Which lints are on is a Cargo concern, and belongs where every crate inherits it.

## Edition and MSRV

**Latest edition, always.** ***M-LATEST-EDITION***: new crates set `edition` to
the latest stable release, currently `2024` at minimum. Older editions provide no
downstream compatibility advantage — a 2015-edition crate can depend on a
2024-edition crate without friction, and vice versa. An old edition on a new
crate buys nothing and costs you the modern syntax.

**Set an MSRV on day one, and keep it behind stable.** ***M-MSRV***: libraries
declare a minimum supported Rust version at creation and update it as new
compiler features become necessary, staying a few versions behind current
release.

```toml
[workspace.package]
edition = "2024"
rust-version = "1.85"   # the floor for edition 2024
```

**Bumping the MSRV is a *minor* version bump, not a major one.** This surprises
people. The reasoning in *M-MSRV* is that ecosystem projects already depend on
reasonably modern compilers through their transitive dependencies, so treating
every MSRV bump as a breaking change produces major-version churn that helps
nobody. Note it in the changelog, because it is the most common reason a
downstream build breaks after an upgrade.

Declare it in exactly one place. Here, CI reads it back out of `Cargo.toml`
rather than restating it — see the `msrv` job in
[`.github/workflows/ci.yaml`](.github/workflows/ci.yaml). Clippy reads it from
there too, which is why [`clippy.toml`](clippy.toml) has no `msrv` key.

Same rule for the toolchain: [`rust-toolchain.toml`](rust-toolchain.toml) pins
the channel, and the CI workflow does not mention a version at all. Pinning it
twice is how the two drift apart.

## Errors: `thiserror` for libraries, `anyhow` for binaries

Not a matter of taste. It follows from who the caller is.

**Libraries use [`thiserror`][thiserror].** Callers need to *match* on failures to
decide what to do, so the error has to be a real type with real variants.
`thiserror` generates the `Display` and `Error` impls and then disappears — it
does not show up in your public API.

Make the enum `#[non_exhaustive]`. Adding a variant to a public enum is normally
a breaking change; `#[non_exhaustive]` forces external callers to write a
catch-all arm up front, which makes future variants a minor bump. Error enums
grow by nature, so this is nearly always the right trade. See
[`crates/app-core/src/error.rs`](crates/app-core/src/error.rs).

**Binaries use [`anyhow`][anyhow].** Nothing downstream will match on the error;
the only consumer is a human reading stderr. That makes attaching context worth
far more than enumerating variants. `fn main() -> anyhow::Result<()>` gets you
the printed error chain and a non-zero exit code for free. See
[`crates/app-cli/src/cli.rs`](crates/app-cli/src/cli.rs).

**`unwrap()` in library code is the anti-pattern**, and the reason
`clippy::unwrap_used` is on in this workspace. It is a promise that this can
never fail, made to someone who cannot check it and will be the one to see the
panic. If you can prove it, write the proof in a comment next to the allow. If
you cannot, return a `Result`.

## When *not* to split into crates

**A single-crate project does not need a `crates/` directory.** `src/` at the
repository root is the correct layout, and this repository would be smaller and
better if it had one crate rather than three.

Legitimate reasons to add one:

1. **Proc macros.** Not a choice: a `proc-macro = true` crate is compiled for the
   host, loaded into `rustc`, and may export nothing else. This is the one split
   the language forces. See [`crates/app-macros/`](crates/app-macros/).
2. **Separate publishing.** A different release cadence, or a subset you want on
   crates.io without the rest.
3. **A genuinely different dependency set.** Keeping a web framework or a
   database driver out of a crate that does not need it.
4. **Compile-time isolation.** A stable core stops being rebuilt every time the
   volatile parts change.
5. **A boundary you want the compiler to enforce absolutely**, including
   acyclicity — Cargo forbids crate cycles, modules do not.

**"It feels tidier" is not one.** A crate boundary costs a manifest, two entries
in workspace tables, a public API you now have to keep stable, and a compile
unit. Modules are free, and `pub(crate)` covers most of what people reach for a
crate to express.

## Automation: `xtask`, not `make`

Put repository automation in a Rust crate and alias it:

```toml
# .cargo/config.toml
[alias]
xtask = "run --package xtask --"
```

`cargo xtask ci` then expands to `cargo run --package xtask -- ci`. That is the
entire [`cargo-xtask`][xtask] mechanism — "a way to extend stock, stable cargo
with custom commands (xtasks), written in Rust".

Why it beats a Makefile:

- **Cross-platform.** It "can more easily be cross platform, as it doesn't use
  the shell". No `sh`-isms, no parallel `.ps1` that drifts.
- **Type checked.** A typo in a Makefile recipe is found by whoever hits it. A
  typo here does not compile.
- **Nothing to install.** A contributor with `rustup` has everything. `make` is
  absent on a stock Windows box; `just` is one more thing to install and pin.
- **CI runs the same code.** When the CI job is `cargo xtask ci`, "works locally,
  fails in CI" stops being about the commands differing.

The cost is that the first `cargo xtask` in a clean checkout compiles the crate,
which is why [`xtask/`](xtask/) has **zero dependencies** and should keep none.
matklad's own advice: *"It is advisable to minimize the compile time of xtasks."*
`std::env::args().nth(1)` is enough; adding `clap` here makes every contributor
pay for argument parsing before their first check.

`xtask` runs when you ask it to — *"xtasks do not integrate with Cargo
lifecycle"*. Compile-time code generation is `build.rs`, a different mechanism
with a different cost.

## Directories the ecosystem has no convention for

Everything below this line: **Rust has no convention, and neither does this
document beyond "pick one and be consistent".** Cargo does not know these
directories exist. Common choices, with a defensible default in bold:

| Purpose | Common names | Notes |
|---|---|---|
| Long-form docs | **`docs/`**, `doc/`, `book/` | `cargo doc` is the API reference; this is what does not fit in a `///` comment. |
| Decision records | **`docs/adr/`**, `docs/decisions/`, `rfcs/` | See [`docs/adr/README.md`](docs/adr/README.md) for a template. |
| Deployment | **`deploy/`**, `deployments/`, `infra/`, `ops/`, `k8s/` | Dockerfiles, manifests, Terraform. |
| Static files | **`assets/`**, `resources/`, `static/`, `data/` | Must stay *outside* `src/`, which Cargo compiles. |
| Shell scripts | **`scripts/`** | Prefer `xtask`. See [`scripts/README.md`](scripts/README.md). |
| Schemas | **`proto/`**, `openapi/`, `schemas/` | Whatever the code generator expects. |
| Migrations | **`migrations/`** | Set by your ORM (`sqlx`, `diesel`), not by you. |

**Delete the ones you do not use.** An empty `deploy/` containing only a README
is worse than no `deploy/`: it implies a deployment story exists and sends
readers looking for one. The directories in this repository are a menu, not a
checklist — take `crates/` and `xtask/` if you need them, take nothing else
unless you have something to put in it.

---

## Coming from Go

If you are porting a Go layout, this table is the short version. **The biggest
mistake is bringing the directory structure across wholesale.** Most of Go's
top-level directories are answers to questions Rust answers elsewhere — in the
language, in Cargo, or not at all.

| [`golang-standards/project-layout`][go-layout] | Rust equivalent |
|---|---|
| `cmd/` | `src/bin/`, or a dedicated `crates/<name>-cli` crate |
| `internal/` | **Just don't write `pub`.** Visibility is compiler-enforced |
| `pkg/` | `src/` for one crate, `crates/` for several |
| `api/` | `proto/`, `openapi/` — not language-defined |
| `vendor/` | `cargo vendor` if truly needed; `Cargo.lock` usually suffices |
| `test/` | `tests/` for integration; `#[cfg(test)] mod tests` for unit |
| `configs/` | `config/` or `assets/` — no Rust convention |
| `build/`, `Makefile` | `xtask/`, or `just` / `cargo-make` |
| `third_party/` | `Cargo.toml` dependencies; forks via `[patch]` |
| `scripts/` | `xtask/` first; `scripts/` for bootstrapping only |
| `docs/` | `cargo doc` for the API; `docs/` for everything else |
| `examples/` | `examples/` — same name, but Cargo compiles it |
| `web/`, `assets/` | `assets/` — no convention, keep it outside `src/` |
| `init/`, `deployments/` | `deploy/` — no convention |
| `githooks/` | `.githooks/` or `scripts/` — no convention, and no need for a top-level directory |

### `internal/` in particular

**`internal/` solves a problem Rust does not have.** Go needed the compiler to
special-case a *directory name* because Go has no visibility modifier beyond
capitalisation, and capitalisation is per-identifier with no notion of "public
within this module but not outside it".

Rust has `pub`, `pub(crate)`, `pub(super)` and `pub(in path)`, all enforced by
the compiler, all local to the item rather than to a directory. Creating an
`internal/` module in Rust adds a naming convention on top of a language feature
that already does the job better — and the private-module-plus-`pub use` façade
in [Part 1](#module-layout-inside-src) gives you the same encapsulation with a
*better* public API, because the internal path never appears in a `use`
statement at all.

### `pkg/` and `cmd/`

`pkg/` is a Go response to `internal/` — if `internal/` is private, something has
to be public. Rust has no such split: `src/` is the crate, and what is public is
what says `pub`.

`cmd/` maps to `src/bin/` for a small extra binary, or a `-cli` crate when the
binary has its own dependency set. The Go instinct of a thin `cmd/foo/main.go` is
right and worth keeping — see [`crates/app-cli/src/main.rs`](crates/app-cli/src/main.rs),
which is four lines and explains why.

### What actually carries over

The instincts, not the directories: keep binaries thin, keep the domain free of
I/O, put deployment configuration somewhere obvious, write down decisions. Those
were always the valuable part of the Go layout. The directory names were the
part that was never a standard.

---

## Anti-patterns

Things that will get flagged in review, and what to do instead.

**Crates nested inside other crates.** `crates/app/core/`, or worse
`crates/app-cli/src/macros/` with its own `Cargo.toml`. Cargo does not find them,
`cargo build` does not build them, and no reader expects them. Flat siblings in
one directory; express relationships with name prefixes. *M-CRATES-FLAT-FOLDER*.

**`path` dependencies between siblings.** `app-core = { path = "../app-core" }`
works locally and then produces a crate that cannot be published, because there
is no version requirement for consumers to resolve. Use
`app-core.workspace = true` with `path` *and* `version` declared once in
`[workspace.dependencies]`. *M-CRATES-IN-WORKSPACE*.

**`mod.rs` everywhere.** Legal, but it fills your editor with identical tab
titles for no benefit. `foo.rs` + `foo/`. And whichever you pick, do not mix
them.

**An `internal/` or `pkg/` directory.** Go habits. Rust has `pub(crate)` and
private modules, enforced by the compiler. See [above](#internal-in-particular).

**A `utils` / `helpers` / `common` junk-drawer module.** These names mean "I did
not want to decide where this goes", and they grow monotonically because nothing
ever gets removed from a module with no defining idea. Name modules after what
they are about. If a function genuinely belongs nowhere, it usually belongs next
to the type it operates on.

**Deep hierarchies built in anticipation.** `src/domain/models/entities/order/`
on the theory that it will be needed later. It will not, and until then every
reader pays for it. Start flat — `src/order.rs` — and split when a file gets hard
to navigate, not before. The `foo.rs` + `foo/` form makes that split mechanical
when it comes.

**`unwrap()` in library code.** A promise that this can never fail, made to
someone who cannot verify it and will be the one to see the panic. `thiserror`
for libraries, `anyhow` for binaries. If an `unwrap` is genuinely provable, allow
the lint narrowly and write the proof in the comment.

**Uncommitted `Cargo.lock`.** Non-reproducible CI, `git bisect` that does not
bisect, and "works on my machine" that is technically accurate. Commit it, run CI
with `--locked`, and get fresh-dependency coverage from a scheduled
`cargo update` job instead.

**Deviating from `src/` / `tests/` / `benches/` / `examples/`.** This is the one
that is not a matter of opinion. Those names are how Cargo finds your targets. A
`test/` directory does not fail loudly — `cargo test` reports success having run
nothing at all.

**Lints scattered as `#![deny(...)]` in crate roots.** One crate's worth of
coverage, invisible from the root, guaranteed to drift. `[workspace.lints]`.

**Restating the toolchain version in CI.** `rust-toolchain.toml` already pins it
and rustup already honours it. Two sources of truth is one source of truth and
one source of confusion.

---

## This repository

```text
.
├── Cargo.toml                     # virtual manifest: [workspace], no [package]
├── Cargo.lock                     # committed, deliberately
├── rust-toolchain.toml            # channel + rustfmt, clippy — the only version pin
├── rustfmt.toml                   # two lines, and it should stay that way
├── clippy.toml                    # thresholds only; lint selection is in Cargo.toml
├── deny.toml                      # cargo-deny: advisories, licences, bans, sources
├── .cargo/config.toml             # [alias] xtask = "run --package xtask --"
├── .editorconfig
├── .gitignore                     # note what is NOT ignored
├── README.md                      # you are here
├── CONTRIBUTING.md
├── CHANGELOG.md
├── LICENSE                        # placeholder: the MIT OR Apache-2.0 convention
├── .github/
│   ├── workflows/ci.yaml          # check · msrv · deny
│   └── dependabot.yml
├── crates/                        # every crate, flat siblings
│   ├── app-core/                  #   library: domain logic, no I/O
│   │   ├── src/{lib,error,config,domain}.rs
│   │   ├── src/domain/{order,customer}.rs
│   │   ├── tests/order-lifecycle.rs
│   │   ├── examples/simple.rs
│   │   └── benches/order-total.rs
│   ├── app-cli/                   #   binary `app`: parsing and wiring only
│   │   ├── src/{main,cli}.rs
│   │   └── tests/cli-args.rs
│   └── app-macros/                #   proc-macro: the split the language forces
├── xtask/                         # automation in Rust, not Make
├── docs/
│   └── adr/                       # template + one filled-in decision record
├── deploy/                        # no Rust convention — delete if unused
├── assets/                        # no Rust convention — delete if unused
└── scripts/                       # prefer xtask — delete if unused
```

Every directory has a `README.md` saying what belongs in it and, where it
matters, what does not. Every non-obvious file has a comment explaining why it is
where it is rather than what the code does.

### Running it

```console
$ cargo xtask ci        # fmt, clippy, tests, doctests, rustdoc — what CI runs
$ cargo xtask fmt       # format in place
$ cargo xtask lint      # clippy, --all-targets, warnings denied
$ cargo xtask test      # tests, then doctests

$ cargo run -p app-cli -- total --line widget:2:1500 --line gadget:1:999
2 lines, total EUR 39.99

$ cargo run -p app-core --example simple
$ cargo bench -p app-core --bench order-total
$ cargo doc --workspace --no-deps --open
```

### Using it as a template

1. Copy `Cargo.toml`, `rust-toolchain.toml`, `rustfmt.toml`, `clippy.toml`,
   `deny.toml`, `.cargo/config.toml`, `xtask/` and the CI workflow.
2. **If you have one crate, stop there.** Put `src/` at the repository root, drop
   the workspace, and re-read [Part 1](#part-1--what-cargo-defines).
3. If you have several, keep `crates/` and delete the example crates.
4. Delete every directory you have nothing to put in.
5. Replace `LICENSE` with real licence text and fix the `repository` and
   `authors` fields.

---

## References

### Primary — authoritative

- [The Cargo Book: Package Layout][cargo-layout] — the canonical layout
- [The Cargo Book: Cargo Targets][cargo-targets] — target auto-discovery
- [The Cargo Book: Workspaces][cargo-workspaces] — virtual manifests, inheritance
- [The Cargo Book: The Manifest Format][manifest] — `[lints]`, `priority`, `rust-version`
- [The Cargo Book: Profiles][profiles] — `lto`, `codegen-units`, `strip`, `debug`
- [The Cargo Book: FAQ — why have `Cargo.lock` in version control][faq-lock]
- [The Cargo Book: Continuous Integration — verifying latest dependencies][ci-latest]
- [The Book, ch. 7: Managing Growing Projects][book-modules] — packages, crates, modules
- [The Book, ch. 11.3: Test Organization][book-tests] — unit vs integration tests
- [Edition Guide: Path clarity][paths] — `foo.rs` + `foo/` instead of `mod.rs`
- [RFC 430: Finalizing naming conventions][rfc430] — casing rules
- [Rust API Guidelines][api-guidelines] — naming, interoperability, documentation

### Opinionated, well-reasoned — the source of most of Part 2

- [Microsoft: Rust Guidelines — Project][ms-project] — *M-CARGO-WORKSPACE*,
  *M-CRATES-IN-WORKSPACE*, *M-CRATES-FLAT-FOLDER*, *M-LATEST-EDITION*, *M-MSRV*
- [matklad: `cargo-xtask`][xtask] — automation in Rust rather than Make
- [`thiserror`][thiserror] and [`anyhow`][anyhow] — the library/binary error split
- [`cargo-deny`][cargo-deny] — advisories, licences, bans, sources

### Prior art — compared against, not copied

- [`golang-standards/project-layout`][go-layout] — the repository this one answers
- [Rust-Trends/example_project_structure](https://github.com/Rust-Trends/example_project_structure)
- [binnev/rust-template](https://github.com/binnev/rust-template/)
- [Stack Overflow: recommended directory structure for a Rust project](https://stackoverflow.com/questions/38276960/what-is-the-recommended-directory-structure-for-a-rust-project)
- [Djamware: Rust project structure and best practices](https://www.djamware.com/post/rust-project-structure-and-best-practices-for-clean-scalable-code)

[go-layout]: https://github.com/golang-standards/project-layout
[cargo-layout]: https://doc.rust-lang.org/cargo/guide/project-layout.html
[cargo-targets]: https://doc.rust-lang.org/cargo/reference/cargo-targets.html
[cargo-workspaces]: https://doc.rust-lang.org/cargo/reference/workspaces.html
[manifest]: https://doc.rust-lang.org/cargo/reference/manifest.html
[profiles]: https://doc.rust-lang.org/cargo/reference/profiles.html
[faq-lock]: https://doc.rust-lang.org/cargo/faq.html#why-have-cargolock-in-version-control
[ci-latest]: https://doc.rust-lang.org/cargo/guide/continuous-integration.html#verifying-latest-dependencies
[book-modules]: https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
[book-tests]: https://doc.rust-lang.org/book/ch11-03-test-organization.html
[paths]: https://doc.rust-lang.org/edition-guide/rust-2018/path-changes.html
[rfc430]: https://rust-lang.github.io/rfcs/0430-finalizing-naming-conventions.html
[api-guidelines]: https://rust-lang.github.io/api-guidelines/
[api-naming]: https://rust-lang.github.io/api-guidelines/naming.html
[ms-project]: https://microsoft.github.io/rust-guidelines/guidelines/project/
[xtask]: https://github.com/matklad/cargo-xtask
[thiserror]: https://docs.rs/thiserror
[anyhow]: https://docs.rs/anyhow
[cargo-deny]: https://embarkstudios.github.io/cargo-deny/

---

## Licence

This repository is a reference layout, licensed `MIT OR Apache-2.0` like the rest
of the Rust ecosystem. See [`LICENSE`](LICENSE) — which is a placeholder
explaining the convention, and which you should replace before publishing
anything.
