# misc-ieee754-001 — IEEE754 float decomposition functions

Slice: `misc-ieee754` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:36:16Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Exact float<->(mantissa,exponent) mapping functions

## Entrypoints (citations)

- `sqlite3_ieee_init()` (other) — `ext/misc/ieee754.c:329`

## Inputs / outputs / observables

- ieee754(F) → 'ieee754(M,E)' text; ieee754(M,E) → float; ieee754_mantissa/exponent(F); ieee754_to_blob/from_blob

## Behaviour (as implemented)

- init ext/misc/ieee754.c:329: exact decomposition/recomposition of binary64 values

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- Denormals/Inf/NaN handled per IEEE bit patterns via blob forms

## Dependencies

- loadext-api

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/ieee754.c:329`
