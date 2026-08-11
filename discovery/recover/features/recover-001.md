# recover-001 — Recover corrupt db into new db

Slice: `recover` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:31:19Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Stepped recovery using sqlite_dbdata vtab (ext/recover/dbdata.c:1014); lost-and-found handling.

## Entrypoints (citations)

- `sqlite3_recover_init()` (api) — `ext/recover/sqlite3recover.c:2783`, `ext/recover/sqlite3recover.c:2877`, `ext/recover/dbdata.c:1014`

## Inputs / outputs / observables

- Recovered db contents vs corrupt source; lost_and_found_* tables for orphaned data; recover step progression

## Behaviour (as implemented)

- recover_init (ext/recover/sqlite3recover.c:2783) configures source→dest recovery; step (:2877) walks: schema recovery from sqlite_master remnants, per-table page-walk via sqlite_dbdata vtab (ext/recover/dbdata.c:1014), orphan collection into lost_and_found (option-controlled)

## Validation rules found in code

- Options: LOSTANDFOUND name, FREELIST_CORRUPT, ROWIDS, SLOWINDEXES via recover_config

## Edge cases found in code

- Corrupt cells partially recovered field-by-field; unrecoverable pages skipped silently (by design — completeness never claimed)

## Dependencies

- (none found in code)

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/recover/sqlite3recover.c:2783`
- `ext/recover/sqlite3recover.c:2877`
- `ext/recover/dbdata.c:1014`
