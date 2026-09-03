# Architecture decision records

Short documents recording a decision, its context, and its consequences —
written when the decision is made, never edited afterwards.

The value is entirely in the _why_. Six months from now the code will show what
was decided; nothing except an ADR will show what the alternatives were and why
they lost. That is the question that actually blocks people.

## Conventions

- One file per decision, numbered and never renumbered: `NNNN-kebab-case-title.md`.
  `0000` is reserved for [the template](0000-template.md), so there is no ADR 0000.
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

[`0000-template.md`](0000-template.md). Copy it; do not edit it in place.

```console
$ cp docs/adr/0000-template.md docs/adr/0042-choose-a-database.md
```

It is kept as a file rather than pasted into this README so there is one copy to
maintain. Four sections, and the order is load-bearing — Context before Decision
so the reasoning reads forwards, Alternatives before Consequences so the trade
being made is on the page before its price:

| Section                 | Answers                                            |
| ----------------------- | -------------------------------------------------- |
| Context                 | What was true, and what forced a decision          |
| Decision                | What we will do                                    |
| Alternatives considered | What else was on the table, and why it lost        |
| Consequences            | What this costs, and what would make us revisit it |

**Alternatives is the section that justifies the whole practice.** "We chose X"
is recoverable from the code. "We tried Y and it could not do Z" is not
recoverable from anywhere.

See [`0001-use-a-cargo-workspace.md`](0001-use-a-cargo-workspace.md) for a
filled-in example.

## Where this comes from

[`architecture-decision-record/architecture-decision-record`][adr-index] is the
community index: Michael Nygard's original template, MADR, Tyree & Akerman,
arc42 and a dozen more, plus guidance on which decisions are worth recording. If
the four sections above are too thin for a decision you are making — MADR's
explicit pros-and-cons-per-option format is the usual next step up — take a
template from there rather than inventing one.

Two places where this directory deliberately differs from it, so that following
the link is not confusing:

**Numbered files, not verb phrases.** That index recommends file names built from
present-tense imperative verbs, such as `choose-database.md`. Numbers are used
here because "superseded by 0007" needs a handle that cannot drift, and a verb
phrase stops being unique the moment a second decision touches the same subject.

**Immutable, not amended.** That index is explicit that "in theory, immutability
is ideal. In practice, mutability has worked better for our teams", and suggests
adding date-stamped updates in place. This directory takes the stricter line: a
decision that changes gets a _new_ ADR that supersedes the old one. Both work,
and the mutable version is easier to live with. The strict version is chosen here
because editing a record quietly destroys the one thing it was written to
preserve — what its authors did not know yet.

[adr-index]: https://github.com/architecture-decision-record/architecture-decision-record
