# `app-core`

The domain: order and customer types, and the rules about how they change.
No I/O, no CLI, no framework.

## Layout

```text
app-core/
├── Cargo.toml
├── src/
│   ├── lib.rs             # façade: declares modules, re-exports the API
│   ├── config.rs          # configuration shape (not loading)
│   ├── error.rs           # the crate's error type
│   ├── domain.rs          # declares the submodules below, re-exports them
│   └── domain/
│       ├── customer.rs
│       └── order.rs       # + its unit tests, at the bottom of the file
├── tests/
│   └── order-lifecycle.rs # integration test: public API only
├── examples/
│   └── simple.rs
└── benches/
    └── order-total.rs
```

Every one of those directories is defined by Cargo. Renaming any of them loses
target auto-discovery.

## The two things this crate is here to demonstrate

**`lib.rs` as a façade.** Every `mod` in `lib.rs` is private; the public API is
the list of `pub use` statements below them. Callers write `app_core::Order`,
never `app_core::domain::order::Order`. The internal module tree is therefore
not part of the API, and can be reorganised without a major version bump.

It is also why no directory here means "private": privacy is a language feature
the compiler enforces, not a directory name a linter enforces.

**Unit tests and integration tests are not the same test.**

|             | `#[cfg(test)] mod tests` in `src/domain/order.rs` | `tests/order-lifecycle.rs`    |
| ----------- | ------------------------------------------------- | ----------------------------- |
| Compiled as | part of this crate                                | a separate crate              |
| Can see     | private fields, private functions                 | only what `lib.rs` re-exports |
| Tests       | that the implementation is right                  | that the API is usable        |
| Breaks when | internals change                                  | the public API changes        |

Both files say so in their own comments, and the integration test has a
commented-out line that does not compile, to make the boundary concrete.

## Dependencies

`serde` for the configuration shape, `thiserror` for the error type. That is
all, on purpose: this is the crate everything else depends on, so every
dependency added here is added to everything.

Note what is _not_ here. There is no `toml` or `serde_json`, because choosing a
configuration file format is an application decision. There is no `anyhow`,
because a library's callers need to match on errors.
