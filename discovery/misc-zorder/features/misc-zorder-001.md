# misc-zorder-001 — z-order curve mapping functions

Slice: `misc-zorder` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:36:16Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Morton-code zorder()/unzorder() scalar functions for multi-dimensional indexing

## Entrypoints (citations)

- `sqlite3_zorder_init()` (other) — `ext/misc/zorder.c:119`

## Inputs / outputs / observables

- zorder(x0,x1,...) → interleaved-bit Morton code; unzorder(z,ndim,i) → i-th coordinate

## Behaviour (as implemented)

- init ext/misc/zorder.c:119: bit-interleaving scalar functions for up to ~20 dims within 63 bits

## Validation rules found in code

- Inputs treated as non-negative integers within bit budget

## Edge cases found in code

- Overflow of the 63-bit budget silently truncates high bits (caller contract)

## Dependencies

- loadext-api

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/zorder.c:119`
