# `xtask/`

Repository automation, written in Rust.

```console
$ cargo xtask ci      # everything CI runs, in the order CI runs it
$ cargo xtask fmt     # format in place
$ cargo xtask lint    # clippy over every target, warnings denied
$ cargo xtask test    # tests, then doctests
$ cargo xtask help
```

## How it works

The whole mechanism is one alias in [`../.cargo/config.toml`](../.cargo/config.toml):

```toml
[alias]
xtask = "run --package xtask --"
```

`cargo xtask ci` expands to `cargo run --package xtask -- ci`. There is no
plugin, nothing to install, and no magic. The pattern is
[matklad's `cargo-xtask`](https://github.com/matklad/cargo-xtask):
"a way to extend stock, stable cargo with custom commands (xtasks), written in
Rust."

## Why not a Makefile

| | `xtask` | `make` | `just` | `scripts/*.sh` |
|---|---|---|---|---|
| Runs on Windows | yes | no | yes | no |
| Extra install | none | `make` | `just` | shell |
| Typos caught at | compile time | run time | run time | run time |
| CI runs the same thing | yes | yes | yes | yes |
| Can call libraries | yes | no | no | no |

`just` is a real alternative and a good tool. The argument for `xtask` over it
is one fewer thing to install and pin, and that "automation" and "the project"
are then the same language, the same lints, and the same review standard.

The honest cost: the first `cargo xtask` in a clean checkout compiles this
crate. **Which is why it has no dependencies, and should keep none.** Adding
`clap` here would make every contributor pay for argument parsing before their
first check — `std::env::args().nth(1)` is enough. matklad's own advice: "It is
advisable to minimize the compile time of xtasks."

## Workspace membership

`xtask` is a member of the main workspace, so it is covered by the same
`Cargo.lock`, the same `[workspace.lints]`, and `cargo fmt --all`. The cost is
that `cargo build --workspace` builds it.

The alternative is a second, excluded workspace rooted at `xtask/`, which keeps
it out of ordinary builds at the price of a second lockfile and a second lint
configuration. Both are defensible. The choice is recorded here so nobody has to
re-derive it.

## What does not belong here

Anything that needs to run *during* a build. `xtask` is invoked explicitly and
does not hook into the Cargo lifecycle — "xtasks do not integrate with Cargo
lifecycle". Compile-time code generation is [`build.rs`](https://doc.rust-lang.org/cargo/reference/build-scripts.html),
which is a different mechanism with a different cost.
