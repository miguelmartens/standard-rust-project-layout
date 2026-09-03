# 0000. Template

<!--
HOW TO USE THIS FILE

    cp docs/adr/0000-template.md docs/adr/0042-choose-a-database.md

Take the next free number and never reuse or renumber one -- "supersedes 0007"
has to keep meaning the same thing forever. 0000 is reserved for this template,
so there is no ADR 0000.

Title the decision AS a decision: "Use a Cargo workspace", not "Workspace
decision" and not "Workspace?". Someone scanning the directory listing should be
able to learn what was decided without opening anything.

Delete every comment and every italic prompt as you fill it in. If a section
ends up empty, that is information -- work out why before deleting the heading.
-->

- **Status:** Proposed <!-- Proposed | Accepted | Superseded by NNNN | Deprecated -->
- **Date:** YYYY-MM-DD
- **Deciders:** names, or the team that owns the consequences

## Context

_The forces at play: the constraint, the problem, what is true today. Write it
so that someone who was not in the room can judge whether it still holds — that
is what makes the record useful in two years rather than merely present._

_No solutions here. If you are describing an option, it belongs below._

## Decision

_What was decided, in the active voice and the present tense: "We will …"._

_One paragraph. If it takes three, this is probably several decisions, and they
will be easier to supersede individually._

## Alternatives considered

_Every option that was genuinely on the table, and the specific reason it lost._

- **Option A** — why it lost.
- **Option B** — why it lost.

_This is the section people come back for. "We chose X" is recoverable from the
code; "we tried Y and it could not do Z" is not recoverable from anywhere. An
ADR with an empty Alternatives section is a changelog entry._

## Consequences

_What becomes easier, what becomes harder, and what has to be revisited if one
of the assumptions in Context stops being true._

_Include the costs. A record that lists only benefits reads as advocacy, and the
next person will not trust the rest of it either._
