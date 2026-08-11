# session-002 — Changeset apply with conflict resolution

Slice: `session` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:31:19Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Apply with per-conflict callback returning OMIT/REPLACE/ABORT; foreign-key ordering rules.

## Entrypoints (citations)

- `sqlite3changeset_apply()` (api) — `ext/session/sqlite3session.c:5940`

## Inputs / outputs / observables

- apply results; conflict callback invocations with type (DATA/NOTFOUND/CONFLICT/CONSTRAINT/FOREIGN_KEY) and resolution (OMIT/REPLACE/ABORT)

## Behaviour (as implemented)

- changeset_apply (ext/session/sqlite3session.c:5940) replays a changeset inside a SAVEPOINT: per-op PK lookup, expected-old-value comparison → conflict callback on mismatch; REPLACE for DATA conflicts overwrites; ABORT rolls back the whole apply

## Validation rules found in code

- Target schema must match (column count/PK) else CONSTRAINT-type failures
- FK violations reported once at end with FOREIGN_KEY conflict type

## Edge cases found in code

- apply_v2 adds rebase-blob output and flags (NOSAVEPOINT, IGNORENOOP...)

## Dependencies

- session-001

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/session/sqlite3session.c:5940`
