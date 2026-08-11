# misc-totype-001 — strict conversion functions tointeger/toreal

Slice: `misc-totype` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:36:16Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

NULL-on-failure strict conversions (unlike CAST's clamping)

## Entrypoints (citations)

- `sqlite3_totype_init()` (other) — `ext/misc/totype.c:511`

## Inputs / outputs / observables

- tointeger(V) / toreal(V): exact-conversion-or-NULL (unlike CAST which clamps/truncates)

## Behaviour (as implemented)

- init ext/misc/totype.c:511: strict conversions — text must parse fully, reals must be exactly representable to convert to int

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- toreal(9007199254740993) → NULL (not exactly representable); tointeger(1.5) → NULL

## Dependencies

- loadext-api

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/totype.c:511`
