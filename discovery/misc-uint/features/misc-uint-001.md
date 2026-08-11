# misc-uint-001 — UINT collating sequence

Slice: `misc-uint` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:36:16Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Collation comparing embedded unsigned integers numerically (natural sort)

## Entrypoints (citations)

- `sqlite3_uint_init()` (other) — `ext/misc/uint.c:84`

## Inputs / outputs / observables

- ORDER BY col COLLATE UINT — digit runs compared numerically, e.g. 'x9' < 'x10'

## Behaviour (as implemented)

- init ext/misc/uint.c:84: collation comparing mixed text with embedded unsigned integer runs numerically (leading zeros equal-ish per rules)

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- Non-digit segments compare bytewise (memcmp semantics)

## Dependencies

- loadext-api

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/uint.c:84`
