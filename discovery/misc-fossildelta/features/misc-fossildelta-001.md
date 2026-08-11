# misc-fossildelta-001 — fossil delta functions + delta_parse vtab

Slice: `misc-fossildelta` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:37:13Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Fossil-format binary deltas between blobs; delta_parse TVF decodes them

## Entrypoints (citations)

- `sqlite3_fossildelta_init()` (other) — `ext/misc/fossildelta.c:1111`

## Inputs / outputs / observables

- delta_create(A,B) blob; delta_apply(A,D) → B; delta_output_size(D); delta_parse(D) TVF rows (op,a1,a2,data)

## Behaviour (as implemented)

- init ext/misc/fossildelta.c:1111: Fossil delta format (rolling-hash copy/insert ops with checksum trailer)

## Validation rules found in code

- delta_apply verifies checksum → error on mismatch

## Edge cases found in code

- Deterministic output for identical inputs (format-level contract)

## Dependencies

- loadext-api

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/fossildelta.c:1111`
