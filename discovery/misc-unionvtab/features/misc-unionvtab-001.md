# misc-unionvtab-001 — unionvtab + swarmvtab (same source file)

Slice: `misc-unionvtab` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:35:25Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Presents many identically-shaped tables (optionally lazily-attached dbs — swarmvtab) as one table

## Entrypoints (citations)

- `sqlite3_unionvtab_init()` (other) — `ext/misc/unionvtab.c:1371`

## Inputs / outputs / observables

- unionvtab over N same-schema tables partitioned by rowid ranges; swarmvtab lazy-ATTACHes source dbs on demand

## Behaviour (as implemented)

- init ext/misc/unionvtab.c:1371: source-spec table (db,tbl,minRowid,maxRowid); queries route by rowid constraint to the right source; swarmvtab variant opens/closes member dbs with a configurable open limit

## Validation rules found in code

- Sources must have identical column definitions; overlapping ranges rejected

## Edge cases found in code

- Writes NOT supported (read-only vtab)

## Dependencies

- vtab-core
- attach-detach

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/unionvtab.c:1371`
