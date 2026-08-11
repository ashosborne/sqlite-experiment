# misc-wholenumber-001 — wholenumber vtab

Slice: `misc-wholenumber` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:35:25Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Infinite integer sequence vtab (predecessor of generate_series)

## Entrypoints (citations)

- `sqlite3_wholenumber_init()` (other) — `ext/misc/wholenumber.c:276`

## Inputs / outputs / observables

- SELECT value FROM wholenumber WHERE value>0 AND value<100 — integer stream bounded by constraints

## Behaviour (as implemented)

- init ext/misc/wholenumber.c:276: vtab emitting consecutive integers; xBestIndex converts range constraints to bounds

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- Unconstrained scan is effectively unbounded — LIMIT required

## Dependencies

- vtab-core

## Assumptions / unknowns

- Predecessor of generate_series; retire question stands
- Retire in favour of misc-series?

## Evidence

- `ext/misc/wholenumber.c:276`
