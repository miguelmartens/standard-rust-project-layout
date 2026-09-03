# `tests/`

Integration tests for the `app` binary.

A binary crate has no library target, so there is nothing here to `use`. These
tests start the real executable and inspect what comes back out.

`env!("CARGO_BIN_EXE_app")` is the path to the freshly built binary — Cargo sets
`CARGO_BIN_EXE_<name>` for every binary target in the package. Use it instead of
hard-coding `target/debug/app`, which is wrong under `--release`, wrong when
cross-compiling, and wrong when `CARGO_TARGET_DIR` is set.

The name is the **binary target** name (`app`), not the package name (`app-cli`).

## The point being made

These tests are slower, coarser and harder to debug than the unit tests in
`app-core`. Reading them is the argument for keeping `main.rs` four lines long.

For anything more elaborate, [`assert_cmd`](https://docs.rs/assert_cmd) and
[`insta`](https://docs.rs/insta) do this properly — process assertions and
snapshot testing of output. Both are `[dev-dependencies]`.
