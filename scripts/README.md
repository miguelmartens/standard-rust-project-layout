# `scripts/`

Shell scripts, and a warning about them.

**Prefer [`../xtask/`](../xtask/).** Automation written in Rust is cross
platform, type checked, needs nothing installed beyond `rustup`, and is the same
code CI runs. A shell script is none of those things. If you can express the
task in `xtask`, express it there.

## What still justifies a script

- **Bootstrapping**, which by definition runs before `cargo` is available.
- **Glue for a tool that is already a CLI** and needs three lines to invoke.
- **Environment-specific one-offs** that would be noise in `xtask`.
- **Git hooks** (`golang-standards/project-layout` gives these their own
  `githooks/` directory; there is no reason to).

## If you write one anyway

- `#!/usr/bin/env bash` and `set -euo pipefail`, on line 1 and line 2.
- Quote every expansion. `shellcheck` in CI, or do not bother.
- Say at the top of the file where it is meant to be run from, and make it work
  from anywhere:
  `cd "$(dirname "${BASH_SOURCE[0]}")/.."`.
- If it grows past about fifty lines, it has become a program. Move it to
  `xtask`.

Delete this directory if it is empty. A `scripts/` folder containing only a
README suggests there is tooling to find.
