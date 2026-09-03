# `app-macros`

Procedural macros. **The one crate split Rust forces on you.**

```toml
[lib]
proc-macro = true
```

That line changes how the crate is compiled: it is built for the *host* and
dynamically loaded into `rustc`, and it may export nothing but macros. Domain
types cannot live alongside them. There is no arrangement of modules that avoids
this, and no judgement call to make.

Worth stating plainly, because it is the exception. Every other crate boundary
in this repository is a decision that could reasonably have gone the other way.

## Hiding the split from users

The convention is for the library crate to re-export the macro, so downstream
code depends on one crate and never learns there are two:

```rust
// in app-core/src/lib.rs
pub use app_macros::shout;
```

This is what `serde` does with `serde_derive` behind its `derive` feature, and
what `thiserror` does with `thiserror-impl`. This repository skips it only so
that `app-cli`'s dependency on `app-macros` stays visible in the manifest.

## Why there is no `syn` or `quote`

Real macros use [`syn`](https://docs.rs/syn) to parse, [`quote`](https://docs.rs/quote)
to build output, and [`proc-macro2`](https://docs.rs/proc-macro2) to make both
usable outside a compilation. **Use them.** They are the ecosystem default for
good reason.

`shout!` parses one string literal, which is roughly the limit of what the raw
`proc_macro` API can do without becoming painful. Keeping this crate
dependency-free keeps the repository's dependency tree honest — the subject here
is layout, not macros.

## Testing a proc-macro crate

The macro cannot be invoked from `src/`; it has to be compiled and loaded before
it can run. So:

- **`#[cfg(test)] mod tests` in `src/lib.rs`** covers the pure helpers only.
  `proc_macro::TokenStream` panics outside a real compilation, so the macro body
  itself is unreachable from there. This is the practical reason real macro
  crates parse into `proc_macro2` types: those work anywhere, which makes the
  interesting logic unit-testable.
- **[`tests/shout.rs`](tests/shout.rs)** is a separate crate that depends on this
  one, so it invokes the macro exactly as a user would.
- **`#[doc = "```compile_fail"]` examples** cover the error paths, and
  [`trybuild`](https://docs.rs/trybuild) does it properly for anything larger.
