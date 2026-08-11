# misc-series-001 — generate_series table-valued function

Slice: `misc-series` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:35:25Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Eponymous vtab producing integer sequences with start/stop/step hidden columns

## Entrypoints (citations)

- `sqlite3_series_init()` (other) — `ext/misc/series.c:939`

## Inputs / outputs / observables

- SELECT value FROM generate_series(start,stop,step); hidden cols start/stop/step settable via WHERE; ascending/descending per step sign

## Behaviour (as implemented)

- Eponymous TVF (init ext/misc/series.c:939); xBestIndex maps constraints on hidden columns to cursor params; missing stop → infinite-ish bounded by LIMIT; step 0 → error

## Validation rules found in code

- Non-integer args coerced; step=0 errors

## Edge cases found in code

- Negative step requires start>=stop to produce rows; int64 overflow bounds respected

## Dependencies

- vtab-core
- loadext-api

## Assumptions / unknowns

- Built into the shell; library availability depends on build (run-2 flag)
- Often assumed built-in (shell embeds it) — confirm downstream build

## Evidence

- `ext/misc/series.c:939`
