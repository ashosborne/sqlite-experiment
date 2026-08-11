# auth-callback-api-002 — Column-read authorization (IGNORE yields NULL)

Slice: `auth-callback-api` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:22:35Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Read authorization per column; SQLITE_IGNORE causes NULL substitution in results.

## Entrypoints (citations)

- `sqlite3AuthReadCol()` (other) — `src/auth.c:104`, `src/auth.c:136`, `src/auth.c:182`

## Inputs / outputs / observables

- Column reads authorized per (table,column); IGNORE substitutes NULL in the result instead of erroring

## Behaviour (as implemented)

- sqlite3AuthReadCol (src/auth.c:104) / sqlite3AuthRead (src/auth.c:136) authorize SQLITE_READ per column; SQLITE_IGNORE rewrites the expression to NULL (src/auth.c:182)
- DENY on a column read errors the whole statement at prepare

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- IGNORE on a column used inside an index expression or WHERE still yields NULL semantics in evaluation

## Dependencies

- auth-callback-api-001

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/auth.c:104`
- `src/auth.c:136`
- `src/auth.c:182`
