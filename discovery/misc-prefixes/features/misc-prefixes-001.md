# misc-prefixes-001 — prefixes table-valued function

Slice: `misc-prefixes` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:35:25Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Emits all prefixes of a string as rows

## Entrypoints (citations)

- `sqlite3_prefixes_init()` (other) — `ext/misc/prefixes.c:310`

## Inputs / outputs / observables

- SELECT prefix FROM prefixes('string') — all prefixes longest-first incl. empty string

## Behaviour (as implemented)

- TVF init ext/misc/prefixes.c:310; pure function of its argument

## Validation rules found in code

- (none found in code)

## Edge cases found in code

- NULL input → no rows

## Dependencies

- vtab-core

## Assumptions / unknowns

- (none found in code)

## Evidence

- `ext/misc/prefixes.c:310`
