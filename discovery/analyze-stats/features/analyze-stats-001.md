# analyze-stats-001 — ANALYZE writes sqlite_stat1 (and stat4 when enabled)

Slice: `analyze-stats` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:26:32Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Per-index row sampling into stat tables; stat4 behind SQLITE_ENABLE_STAT4.

## Entrypoints (citations)

- `sqlite3Analyze()` (other) — `src/analyze.c:1457`, `src/analyze.c:977`, `src/analyze.c:20`, `src/analyze.c:23`

## Inputs / outputs / observables

- sqlite_stat1 rows (tbl, idx, stat text 'N M ...'); sqlite_stat4 samples when STAT4 enabled; ANALYZE variants: whole db, one table, one index

## Behaviour (as implemented)

- sqlite3Analyze (src/analyze.c:1457) compiles per-table scans via analyzeOneTable (src/analyze.c:977) using internal stat_init/push/get functions; writes stat1 (row count + per-column selectivity estimates); stat4 samples behind SQLITE_ENABLE_STAT4

## Validation rules found in code

- stat1 stat column format: space-separated integers, optional 'unordered'/'sz=' annotations

## Edge cases found in code

- PRAGMA optimize runs ANALYZE selectively based on query history; ANALYZE on empty table writes NULL idx row

## Dependencies

- btree

## Assumptions / unknowns

- Default baseline: STAT4 off unless configure enables — fingerprint
- STAT4 enabled in target build?

## Evidence

- `src/analyze.c:1457`
- `src/analyze.c:977`
- `src/analyze.c:20`
- `src/analyze.c:23`
