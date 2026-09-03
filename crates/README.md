# `crates/`

Every crate this repository produces, as **flat siblings**. One directory per
crate, no nesting.

| Crate | Kind | Why it is separate |
|---|---|---|
| [`app-core`](app-core/) | library | Domain logic with no I/O. Reusable from something other than the CLI; testable without a process. |
| [`app-cli`](app-cli/) | binary | Parses `argv`, wires things together, picks an exit code. |
| [`app-macros`](app-macros/) | proc-macro | The language forces this one. A `proc-macro = true` crate can export nothing else. |

## The rules this directory follows

**Flat, always.** *M-CRATES-FLAT-FOLDER*: "All crates are siblings in one
folder." A crate nested inside another crate's directory — and especially inside
its `src/` — is never acceptable. Cargo will not find it, `cargo build` will not
build it, and readers will not expect it.

Add one level of grouping (`crates/server/`, `crates/client/`) only once the
flat list is genuinely unreadable, which the Microsoft guidelines put at
somewhere past one to two dozen crates. Below that, grouping costs more in
navigation than it saves in tidiness.

**Relationships are expressed in names, not in paths.** `app`, `app-core`,
`app-macros` sort together and read as a family. That is the whole mechanism.
There is no `app/core/` directory and there should not be.

**Every crate is a workspace member.** Listed in `[workspace] members`, and
listed again in `[workspace.dependencies]` with both `path` and `version`.
*M-CRATES-IN-WORKSPACE*. Siblings then depend on each other with
`app-core.workspace = true` — never `app-core = { path = "../app-core" }`, which
reintroduces exactly the version skew the workspace exists to prevent, and
produces a crate that cannot be published.

**Every crate inherits.** Metadata via `field.workspace = true`, lints via
`[lints] workspace = true`. A new crate's manifest should be almost entirely
inheritance; anything it states for itself is a claim that it is different, and
should be true.

## When to add a crate here

Good reasons, roughly in descending order of how often they are the real reason:

1. **Proc macros.** Not a choice — see `app-macros`.
2. **Separate publishing.** Different release cadence, or a subset you want on
   crates.io without the rest.
3. **A genuinely different dependency set.** Keeping a heavyweight dependency
   (a web framework, a database driver) out of a crate that does not need it.
4. **Compile-time isolation.** A stable core that rarely changes stops being
   rebuilt every time the volatile parts do.
5. **A hard architectural boundary you want the compiler to enforce.** Modules
   and `pub(crate)` cover most of this; a crate boundary is the version that
   also prevents a cyclic dependency, since Cargo forbids crate cycles outright.

Not a reason: "it feels tidier". A crate boundary costs a manifest, an entry in
two workspace tables, a public API you now have to keep stable, and a compile
unit. **A single-crate project does not need a `crates/` directory at all** —
`src/` at the repository root is the correct layout for it, and this repository
would be smaller and better if it had one crate rather than three.

## A note on features

The Microsoft guidelines suggest declaring workspace dependencies with
`default-features = false` and opting features back in per crate, so that one
crate's needs do not inflate everyone's build. That is good advice for a large
workspace.

This repository does not do it, because `clap` and `serde` with default features
off require a feature list long enough to obscure the point of the example.
If your workspace is big enough for the build time to matter, follow the
guideline; it is a real trade, not a stylistic one.
