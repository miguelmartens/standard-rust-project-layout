# `tests/`

The only place this crate's macros can be tested for real.

A proc-macro crate cannot invoke its own macros from `src/`: the macro has to be
compiled and loaded into `rustc` before it can run. Files here are separate
crates that depend on `app-macros` as an ordinary dependency, so `shout!` behaves
exactly as it does for a downstream user.

That is the same reasoning as any other integration test — see
[`../../app-core/tests/README.md`](../../app-core/tests/README.md) — with a
harder constraint behind it. Here it is not a design choice about what to test
through the public API; it is the only option.

The unit tests in [`../src/lib.rs`](../src/lib.rs) cover the pure helpers and
nothing else, because `proc_macro::TokenStream` panics when constructed outside a
real compilation. Real macro crates avoid that by parsing into `proc_macro2`
types, which work anywhere and make the interesting logic unit-testable.

For error paths, a doctest fenced as `compile_fail` covers the simple cases, and
[`trybuild`](https://docs.rs/trybuild) covers them properly by asserting on the
exact diagnostic a user would see.
