# upsert-001 — Conflict-target resolution to unique index

Slice: `upsert` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:25:20Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

ON CONFLICT target column list matched to a unique index; error if no match.

## Entrypoints (citations)

- `sqlite3UpsertAnalyzeTarget()` (other) — `src/upsert.c:90`, `src/upsert.c:247`

## Inputs / outputs / observables

- Error 'ON CONFLICT clause does not match any PRIMARY KEY or UNIQUE constraint' when target unresolvable; chosen index visible via plan

## Behaviour (as implemented)

- sqlite3UpsertAnalyzeTarget (src/upsert.c:90) matches the conflict-target column list (+ WHERE for partial indexes) against unique indexes/PK; sqlite3UpsertOfIndex (src/upsert.c:247) selects the upsert clause applying to a given index at codegen

## Validation rules found in code

- Multiple ON CONFLICT clauses: each must name a target except an optional final catch-all DO NOTHING

## Edge cases found in code

- Rowid/INTEGER PRIMARY KEY conflicts match the ipk target form
- Partial unique index targets require matching WHERE expression (structural comparison)

## Dependencies

- dml-codegen

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/upsert.c:90`
- `src/upsert.c:247`
