# `xtask/`

Repository automation, written in Rust.

```console
$ cargo xtask ci      # everything CI runs, in the order CI runs it
$ cargo xtask fmt     # format in place: rustfmt, then prettier if installed
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

|                        | `xtask`      | `make`   | `just`   | `scripts/*.sh` |
| ---------------------- | ------------ | -------- | -------- | -------------- |
| Runs on Windows        | yes          | no       | yes      | no             |
| Extra install          | none         | `make`   | `just`   | shell          |
| Typos caught at        | compile time | run time | run time | run time       |
| CI runs the same thing | yes          | yes      | yes      | yes            |
| Can call libraries     | yes          | no       | no       | no             |

`just` is a real alternative and a good tool. The argument for `xtask` over it
is one fewer thing to install and pin, and that "automation" and "the project"
are then the same language, the same lints, and the same review standard.

None of which is an argument against a Makefile that only _forwards_ here, and
[`../Makefile`](../Makefile) is exactly that: `make ci` runs `cargo xtask ci`,
one line per recipe. The table above compares places to _keep_ automation. A
Makefile with no automation in it does not appear in that comparison, because it
is not doing the job — it is a keyboard shortcut for people whose fingers type
`make` before they finish reading the README.

The honest cost: the first `cargo xtask` in a clean checkout compiles this
crate. **Which is why it has no dependencies, and should keep none.** Adding
`clap` here would make every contributor pay for argument parsing before their
first check — `std::env::args().nth(1)` is enough. matklad's own advice: "It is
advisable to minimize the compile time of xtasks."

## The one optional dependency: Prettier

`fmt` and `ci` also run Prettier over the Markdown, YAML and JSON, because
`cargo fmt` covers `.rs` and nothing else.

It is **optional locally and mandatory in CI**. `xtask` looks for `prettier` on
`PATH` and in `node_modules/.bin`, runs it if found, and prints a skip notice if
not — so `rustup` remains the only hard requirement for contributing, while
[`.prettierrc`](../.prettierrc) still cannot drift, because the CI runner always
has Node.

There is deliberately no `npx --yes prettier` fallback. A build tool that
silently fetches a package from the network on first use behaves differently
offline, and behaving differently is worse than not running.

The `which` helper this needs is fifteen lines of `std`, and it reads `PATHEXT`
so it finds `prettier.cmd` on Windows. That is the argument for this whole
pattern in miniature: the equivalent shell script would have got it wrong, and
nobody would have noticed until a Windows contributor filed an issue.

## Workspace membership

`xtask` is a member of the main workspace, so it is covered by the same
`Cargo.lock`, the same `[workspace.lints]`, and `cargo fmt --all`. The cost is
that `cargo build --workspace` builds it.

The alternative is a second, excluded workspace rooted at `xtask/`, which keeps
it out of ordinary builds at the price of a second lockfile and a second lint
configuration. Both are defensible. The choice is recorded here so nobody has to
re-derive it.

## What does not belong here

Anything that needs to run _during_ a build. `xtask` is invoked explicitly and
does not hook into the Cargo lifecycle — "xtasks do not integrate with Cargo
lifecycle". Compile-time code generation is [`build.rs`](https://doc.rust-lang.org/cargo/reference/build-scripts.html),
which is a different mechanism with a different cost.
