# Architecture decision records

Short documents recording a decision, its context, and its consequences —
written when the decision is made, never edited afterwards.

The value is entirely in the _why_. Six months from now the code will show what
was decided; nothing except an ADR will show what the alternatives were and why
they lost. That is the question that actually blocks people.

## Conventions

- One file per decision, numbered and never renumbered: `NNNN-kebab-case-title.md`.
- **ADRs are immutable.** A decision that changes gets a _new_ ADR that
  supersedes the old one, and the old one gets a line at the top pointing at it.
  Editing history to look consistent destroys the only thing this directory is
  for.
- Status is one of `Proposed`, `Accepted`, `Superseded by NNNN`, `Deprecated`.
- Keep it to a page. If it needs more, the extra material is a design document
  in [`../`](../) that the ADR links to.

## When to write one

When the decision is expensive to reverse, or when someone will predictably ask
"why on earth is it done that way?" — choice of database, error-handling
strategy, a crate split, a dependency with an unusual licence, a deliberate
deviation from a convention this repository otherwise follows.

Not for reversible, local choices. An ADR per function is an ADR directory
nobody reads.

## Template

```markdown
# NNNN. Title stated as a decision

- **Status:** Proposed | Accepted | Superseded by NNNN | Deprecated
- **Date:** YYYY-MM-DD
- **Deciders:** names or team

## Context

The forces at play: the constraint, the problem, what is true today. No
solutions here.

## Decision

What was decided, in the active voice: "We will ...".

## Alternatives considered

Each option that was genuinely on the table, and the specific reason it lost.
This is the section people come back for.

## Consequences

What becomes easier, what becomes harder, and what has to be revisited if an
assumption changes.
```

See [`0001-use-a-cargo-workspace.md`](0001-use-a-cargo-workspace.md) for a
filled-in example.
