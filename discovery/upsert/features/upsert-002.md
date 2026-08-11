# upsert-002 — DO UPDATE / DO NOTHING execution

Slice: `upsert` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:25:20Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Generates UPDATE against conflicting row; excluded.* pseudo-table bindings.

## Entrypoints (citations)

- `sqlite3UpsertDoUpdate()` (other) — `src/upsert.c:267`, `src/upsert.c:55`

## Inputs / outputs / observables

- DO UPDATE executes against the conflicting row with excluded.* pseudo-values; DO NOTHING suppresses the error; changes() counts updates

## Behaviour (as implemented)

- sqlite3UpsertDoUpdate (src/upsert.c:267) generates the UPDATE program against the found conflicting rowid/key; excluded.* resolves to the would-have-inserted values; WHERE on DO UPDATE can skip the update

## Validation rules found in code

- Recursive conflicts from the DO UPDATE itself are NOT re-caught (error surfaces)

## Edge cases found in code

- Triggers: UPDATE triggers fire for the DO UPDATE path, not INSERT triggers

## Dependencies

- upsert-001

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/upsert.c:267`
- `src/upsert.c:55`
