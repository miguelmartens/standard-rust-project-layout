# `examples/`

Runnable examples. **Cargo-defined; do not rename.**

```console
$ cargo run --package app-core --example simple
```

Each `.rs` file here is a binary target with the file stem as its name. A
multi-file example is `examples/name/main.rs` plus sibling modules, and takes
the directory name.

## Why examples and not README snippets

`cargo build --examples` compiles everything in this directory, and CI runs
`--all-targets`. **An example that stops compiling breaks the build.** A snippet
in a README rots silently and is wrong by the time someone copies it.

Doc examples in `///` comments get the same guarantee — they are compiled _and
executed_ by `cargo test --doc` — and are better for anything short, because
they sit next to the item they document. Use `examples/` when the code is too
long for a doc comment or needs to be run interactively.

Note that `cargo test --all-targets` does **not** run doctests. That is why
[`../../../xtask`](../../../xtask) runs `cargo test --doc` as a second command,
and why CI has a separate step for it.

## What does not belong here

Scratch files. Everything in this directory is compiled on every CI run and is
part of what readers judge the crate by.
