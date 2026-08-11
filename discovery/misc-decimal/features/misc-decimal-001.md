# misc-decimal-001 — arbitrary-precision decimal arithmetic

Slice: `misc-decimal` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:36:16Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Exact decimal arithmetic over text representations; decimal collating sequence

## Entrypoints (citations)

- `sqlite3_decimal_init()` (other) — `ext/misc/decimal.c:922`

## Inputs / outputs / observables

- decimal_add/sub/mul(A,B), decimal_cmp(A,B), decimal(X) canonicalizer, decimal_pow2(N); 'decimal' collation ordering numeric-string columns

## Behaviour (as implemented)

- init ext/misc/decimal.c:922: exact arbitrary-precision decimal arithmetic on TEXT representations (no rounding); division intentionally absent

## Validation rules found in code

- Non-numeric text → error ('not a valid decimal')

## Edge cases found in code

- Preserves trailing zeros/scale through add/sub; huge exponents bounded by memory

## Dependencies

- loadext-api

## Assumptions / unknowns

- Financial-math consumer question stands
- Financial-math consumers downstream?

## Evidence

- `ext/misc/decimal.c:922`
