# `app-cli`

The `app` executable. Parses arguments, wires things together, picks an exit
code. Nothing else.

```console
$ cargo run --package app-cli -- total --line widget:2:1500 --line gadget:1:999
2 lines, total EUR 39.99

$ cargo run --package app-cli -- config
```

## The package is `app-cli`; the binary is `app`

```toml
[[bin]]
name = "app"
path = "src/main.rs"
```

Cargo would otherwise name the binary after the package. Separating them is
common — `ripgrep` ships `rg` — and worth knowing before you contort a package
name to get the executable name you wanted. `path` is required here, because
Cargo only infers `src/main.rs` for a `[[bin]]` whose name matches the package.

## Why it is so thin

A binary crate has no library target. Nothing can `use` it: not another crate,
not an integration test, not a benchmark. Code placed here is code that can only
ever be tested through a process boundary.

[`tests/cli-args.rs`](tests/cli-args.rs) shows what that costs. It runs the real
executable via `env!("CARGO_BIN_EXE_app")` — the only way in is `argv`, the only
way out is stdout, stderr and an exit code. Compare that to the unit tests in
`app-core`, which run in microseconds and can assert on private state.

So: `main.rs` is four lines, [`cli.rs`](src/cli.rs) turns strings into domain
types, [`cli/config.rs`](src/cli/config.rs) turns the environment into a
`Config`, and every rule lives in `app-core`.

## Configuration is read here, once

`app_core::Config` is the shape; reading it is this crate's job, because a
library that reads `std::env` has an input its caller cannot see or override.
[`src/cli/config.rs`](src/cli/config.rs) reads `APP_CURRENCY` and
`APP_MAX_ORDER_LINES`, and `Cli::run` passes the resulting `Config` down.
[`.env.example`](../../.env.example) is the documented list of variables —
nothing loads a `.env` for you, which the root README explains under
"Environment variables".

Note the loader takes its lookup as an argument instead of calling
`std::env::var` inline. `std::env::set_var` is `unsafe` in edition 2024, so a
unit test cannot arrange an environment in-process at all; passing the lookup in
makes the parsing and the error messages a pure function, and
[`tests/cli-args.rs`](tests/cli-args.rs) still covers the real thing by setting
variables on the child process.

## `anyhow`, not `thiserror`

Nothing downstream will ever match on an error out of this crate; the only
consumer is a human reading stderr. That makes attaching context far more
valuable than enumerating variants, which is exactly the trade `anyhow` makes.

`fn main() -> anyhow::Result<()>` gets the error chain printed and a non-zero
exit code for free.

## When you would use `src/bin/` instead

A second small binary in an existing package — `src/bin/app-admin.rs` — is
lighter than a whole crate: no manifest, no workspace entries, no published API.
Reach for a separate `-cli` crate when the binary has its own dependency set
(`clap`, a terminal library, a progress bar) that the library crate should not
inherit. That is the case here.
