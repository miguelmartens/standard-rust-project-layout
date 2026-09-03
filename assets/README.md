# `assets/`

Non-code files the project needs: images, fonts, icons, static web files, seed
data, test fixtures too large to inline, SQL migrations, `.proto` and OpenAPI
schemas.

**Rust has no convention here either.** `assets/`, `resources/`, `static/` and
`data/` are all common. What matters is that these files stay *outside* `src/`,
because Cargo treats `src/` as source and you do not want a stray `.rs` file in
a data directory becoming a compile target.

## Getting the files to your program

Two mechanisms, and the choice matters more than the directory name:

**Compiled in.** `include_str!` / `include_bytes!` take a path relative to the
*source file*, so the data becomes part of the binary. Deployment is one file;
changing an asset requires a rebuild. Good for small, rarely-changing things:
a default configuration, a SQL schema, a shader.

**Read at runtime.** The path has to be resolved somehow, and `CARGO_MANIFEST_DIR`
is only correct during `cargo test` and `cargo run` — it does not exist in a
deployed binary. Production code should take the path from configuration or a
platform directory crate rather than guessing relative to the executable.

`build.rs` is the third option, for assets that need generating or transforming
at compile time. Write output to `OUT_DIR` and `include!` it; never write into
`src/`.

## Where the Go layout's directories land

`web/`, `configs/`, `init/` and `api/` from `golang-standards/project-layout`
all fall in here or next to it. None of them has a Rust equivalent, because none
of them is a language concern.

Delete this directory if the project has no assets.
