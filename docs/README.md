# `docs/`

Prose that does not belong in a doc comment.

`cargo doc` output is the reference: what a type is, what a function returns,
what errors it can produce. It is generated from the code, so it cannot go stale
without the compiler noticing. **Anything that can live in a `///` comment
should.**

What is left over, and belongs here:

- **Architecture decision records** — [`adr/`](adr/). Why the code is the way it
  is, written when the decision was made.
- **Design documents and diagrams** for things spanning several crates, which no
  single crate's docs can own.
- **Runbooks and operational notes** — what to do at 03:00.
- **Tutorials and guides** longer than a doc example.

There is no Cargo convention for this directory. `docs/` is simply what most
repositories use, and GitHub Pages knows how to serve it.

## What does not belong here

- **API reference.** That is `cargo doc`. A hand-maintained copy is a
  hand-maintained lie.
- **Code examples.** Those go in `examples/`, where CI compiles them.
- **`README.md` content.** If a reader needs it to get started, it goes in the
  top-level `README.md`, not one directory down.

## If you use mdBook

Point `book.toml` at `docs/` as its source and let it publish from CI. Nothing
in this layout is in the way of that; the requirement is only that it stays
outside `src/`, where Cargo would try to compile it.
