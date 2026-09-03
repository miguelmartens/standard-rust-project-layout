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
types, and every rule lives in `app-core`.

The same instinct produces Go's thin `cmd/foo/main.go`. Rust expresses it as
`src/bin/` or a dedicated `-cli` crate rather than a `cmd/` directory.

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
