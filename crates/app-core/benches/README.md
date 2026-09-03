# `benches/`

Benchmarks. **Cargo-defined; do not rename.**

```console
$ cargo bench --package app-core --bench order-total
```

## `harness = false`

The stock libtest bench harness needs `#[bench]`, which is still unstable, so a
stable-only project cannot use it. Every real benchmarking crate works around
this the same way — declare the target with `harness = false` and provide your
own `main`:

```toml
[[bench]]
name = "order-total"
harness = false
```

The two to choose between are [`criterion`](https://docs.rs/criterion) (mature,
statistical, HTML reports) and [`divan`](https://docs.rs/divan) (newer, much
less ceremony). Both go in `[dev-dependencies]`, so they never reach consumers.

[`order-total.rs`](order-total.rs) hand-rolls a timing loop because this
repository has no benchmarking dependency. **Do not copy that loop** — no
warm-up, no statistics, no outlier detection. Copy the wiring; put a real
harness behind it.

## Profiles

`[profile.bench]` in the workspace root sets `debug = true`. It inherits
everything else from `release`, so the code being measured is optimised, but a
profiler attached to the binary can still attribute samples to source lines.
Debug info costs nothing at runtime.
