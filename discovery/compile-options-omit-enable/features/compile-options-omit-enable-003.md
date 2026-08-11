# compile-options-omit-enable-003 — ENABLE-gate census (surfaces added per build)

Slice: `compile-options-omit-enable` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:33:28Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

51 SQLITE_ENABLE_* guard references add surfaces (STAT4, DESERIALIZE, DBSTAT_VTAB, LOCKING_STYLE, ...).

## Entrypoints (citations)

- `SQLITE_ENABLE_* guards` (other) — `src/sqliteInt.h`, `src/os_unix.c:66`

## Inputs / outputs / observables

- Added surfaces per ENABLE flag (STAT4 samples, DBSTAT/DBPAGE/BYTECODE vtabs, FTS5/RTREE/SESSION/GEOPOLY modules, LOCKING_STYLE, PREUPDATE_HOOK...)

## Behaviour (as implemented)

- CENSUS CARD: 51 SQLITE_ENABLE_* guard references in src/sqliteInt.h (+ per-file gates e.g. src/os_unix.c:66) compile optional subsystems in; ./configure --dev era defaults differ from bare configure — the pinned baseline is bare configure (BASELINE.md)

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- Some enables change defaults rather than add APIs (e.g. ENABLE_STAT4 changes planning)

## Dependencies

- (none found in code)

## Assumptions / unknowns

- Same per-flag deferral as 002

## Evidence

- `src/sqliteInt.h`
- `src/os_unix.c:66`
