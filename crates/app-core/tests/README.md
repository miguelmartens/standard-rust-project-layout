# `tests/`

Integration tests. **Cargo-defined; do not rename.**

Each `.rs` file directly in this directory is compiled as its own crate, links
against `app-core` as an ordinary dependency, and can therefore only reach what
`lib.rs` makes public. That restriction is the point: these tests verify that
the API is usable, not that the implementation is correct.

Unit tests do not live here. They live in `#[cfg(test)] mod tests` beside the
code they test, where they can see private state. See
[`../src/domain/order.rs`](../src/domain/order.rs).

There is no mirrored `test/` tree in Rust. Go's `/test` directory has no
equivalent, because Cargo already owns both halves of the problem.

## File naming

Target names are kebab-case: `order-lifecycle.rs` gives
`cargo test --test order-lifecycle`. Rust _modules_ are `snake_case` (RFC 430),
but these files are crate roots, not modules.

## Shared helper code

Put it in `tests/common/mod.rs`, not `tests/common.rs`. "Files in subdirectories
of the `tests` directory don't get compiled as separate crates" — so
`tests/common.rs` would run as an empty test suite of its own and show up in the
output. Then `mod common;` in each test file that needs it.

A multi-file integration test is `tests/name/main.rs` plus sibling modules; the
directory name becomes the target name.
