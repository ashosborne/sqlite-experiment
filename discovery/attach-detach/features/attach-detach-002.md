# attach-detach-002 — DETACH DATABASE

Slice: `attach-detach` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:22:35Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Removes attached db; errors if db is locked or in use.

## Entrypoints (citations)

- `sqlite3Detach()` (other) — `src/attach.c:429`, `src/attach.c:293`

## Inputs / outputs / observables

- Schema disappears from database_list; error 'database X is locked' when statements still reference it

## Behaviour (as implemented)

- DETACH (sqlite3Detach src/attach.c:429 → detachFunc src/attach.c:293) closes the Btree and frees the schema slot

## Validation rules found in code

- Cannot detach 'main' or 'temp'; cannot detach inside a transaction that wrote to that db (locked error)

## Edge cases found in code

- Prepared statements referencing the detached schema expire

## Dependencies

- attach-detach-001

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/attach.c:429`
- `src/attach.c:293`
