# printf-format-002 — C API mprintf/vmprintf/snprintf

Slice: `printf-format` · Status: `documented` · Confidence: `observed-in-code` · Card written: 2026-08-11T10:23:51Z
As-is behaviour card (Discovery v0.2 Phase B). Not a redesign, not a user story.

## Summary

Public C formatting entrypoints sharing the same core formatter.

## Entrypoints (citations)

- `sqlite3_mprintf()` (api) — `src/printf.c:1519`, `src/printf.c:1559`

## Inputs / outputs / observables

- mprintf/vmprintf return malloc'd strings (NULL on OOM); snprintf truncates to n-1 + NUL

## Behaviour (as implemented)

- sqlite3_mprintf (src/printf.c:1519) and snprintf (src/printf.c:1559) share the core formatter with all SQLite directives incl. %q/%Q/%w/%z (consume-and-free)

## Validation rules found in code

- snprintf returns the buffer pointer (NOT the byte count — differs from C99 snprintf; documented trap)

## Edge cases found in code

- %z frees its argument even on truncation paths

## Dependencies

- (none found in code)

## Assumptions / unknowns

- (none found in code)

## Evidence

- `src/printf.c:1519`
- `src/printf.c:1559`
