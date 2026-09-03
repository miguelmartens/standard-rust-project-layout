# `.cargo/`

Cargo configuration that applies to commands run inside this repository.
Committed, so every contributor and every CI run gets it.

[`config.toml`](config.toml) contains one thing:

```toml
[alias]
xtask = "run --package xtask --"
```

That single alias is the entire [`cargo-xtask`](https://github.com/matklad/cargo-xtask)
mechanism — `cargo xtask ci` becomes `cargo run --package xtask -- ci`. See
[`../xtask/README.md`](../xtask/README.md).

## What else can go here, and the one thing that must not

Legitimate: `[build]` settings such as `target-dir` or `rustflags`, `[target.*]`
linker and runner configuration for cross-compilation, `[net]` settings, more
aliases.

**Never credentials.** `~/.cargo/credentials.toml` is the file for registry
tokens, and it is deliberately outside the repository. A token committed to
`.cargo/config.toml` is a token published to everyone who clones.

Be careful with `[build] rustflags` in particular: it applies to dependencies as
well as your own code, and it silently invalidates the build cache for anything
else on the machine sharing that target directory. Per-crate lints belong in
`[workspace.lints]`, not here.

## The file name

`config.toml`, not `config`. The extensionless form still works for
compatibility and is deprecated; new repositories should use `config.toml`.
