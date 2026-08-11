# misc-btreeinfo-001 — sqlite_btreeinfo introspection vtab

Slice: `misc-btreeinfo` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:36:16Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Per-btree stats (depth, page counts, estimated entries) via sqlite_dbpage

## Entrypoints (citations)

- `sqlite3_btreeinfo_init()` (other) — `ext/misc/btreeinfo.c:439`

## Inputs / outputs / observables

- SELECT * FROM sqlite_btreeinfo — per-btree metadata (type, name, tbl_name, rootpage, hasRowid, nEntry estimate, nPage, depth, szPage, zSchema)

## Behaviour (as implemented)

- init ext/misc/btreeinfo.c:439: eponymous vtab estimating btree shape by probing pages via sqlite_dbpage (requires that vtab)

## Validation rules found in code

- Depends on SQLITE_ENABLE_DBPAGE_VTAB being available

## Edge cases found in code

- nEntry is an estimate (sampled), not a count

## Dependencies

- vtab-core
- introspection-vtabs

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/btreeinfo.c:439`
