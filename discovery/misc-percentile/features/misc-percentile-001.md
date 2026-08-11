# misc-percentile-001 — percentile / median aggregates

Slice: `misc-percentile` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:36:16Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Order-statistics aggregates incl. percentile_cont/percentile_disc variants

## Entrypoints (citations)

- `sqlite3_percentile_init()` (other) — `ext/misc/percentile.c:480`

## Inputs / outputs / observables

- percentile(Y,P) P in 0..100; median(Y); percentile_cont(Y,F) F in 0..1; percentile_disc(Y,F) — aggregate results with interpolation (cont) vs discrete pick (disc)

## Behaviour (as implemented)

- init ext/misc/percentile.c:480: collects non-NULL numeric values, sorts at finalize, interpolates per variant

## Validation rules found in code

- Non-numeric input → error; P out of range → error

## Edge cases found in code

- Empty input → NULL; binary-identical duplicates fine (O(n log n) memory-resident)

## Dependencies

- loadext-api

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/percentile.c:480`
